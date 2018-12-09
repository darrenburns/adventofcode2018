use core::borrow::Borrow;
use core::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::usize;

use uuid::Uuid;

const COORDS_FILE: &'static str = "src/files/six/coords.txt";
const MANHATTAN_LIMIT: i32 = 10000;

type Grid = Vec<Vec<GridCell>>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GridCell {
    // We ignore GridCells when they're tied closest to 2 points
    Border,
    // A point from the input list
    Point { id: String, row: usize, col: usize },
    // A cell which wasn't part of the input list, but belongs within the territory one of those points
    TerritoryOfPoint { point_id: String },
    // A GridCell that hasn't been initialised
    Empty,
}

impl GridCell {
    fn point(row: usize, col: usize) -> Self {
        GridCell::Point { id: Uuid::new_v4().to_string(), row, col }
    }
    fn in_territory_of(point_id: String) -> Self {
        GridCell::TerritoryOfPoint { point_id }
    }
    fn id(&self) -> Option<String> {
        match self {
            GridCell::Point { id, .. } => Some(id.clone()),
            _ => None
        }
    }
    fn get_territory_id(&self) -> Option<String> {
        match self {
            GridCell::Point { id, .. } => Some(id.clone()),
            GridCell::TerritoryOfPoint { point_id, .. } => Some(point_id.clone()),
            _ => None,
        }
    }
    fn row(&self) -> Option<usize> {
        match self {
            GridCell::Point { row, .. } => Some(*row),
            _ => None
        }
    }
    fn col(&self) -> Option<usize> {
        match self {
            GridCell::Point { col, .. } => Some(*col),
            _ => None
        }
    }
}

impl FromStr for GridCell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row: usize = s.chars()
            .skip_while(|c| !c.is_whitespace())
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap();
        let col: usize = s.chars()
            .take_while(|c| c.to_string() != ",")
            .collect::<String>()
            .parse()
            .unwrap();

        Ok(GridCell::point(row - 1, col - 1))
    }
}

// Part 1 - Time: 0.23s
pub fn get_largest_area() -> i32 {
    let points = load_points_from_coords_file();
    let grid = build_grid(&points);

    let points_with_infinite_territory = points_with_infinite_territory(&grid);
    let (point_id, &area_of_territory) = grid.iter()
        .flatten()
        .filter(|cell| {
            let territory_id = match cell {
                GridCell::Point { id, .. } => id,
                GridCell::TerritoryOfPoint { point_id } => point_id,
                _ => return false
            };
            !points_with_infinite_territory.contains(territory_id)
        })
        .filter(|cell| match cell {
            GridCell::Point { .. } | GridCell::TerritoryOfPoint { .. } => true,
            GridCell::Border => false,
            _ => panic!("Critical logic error"),
        })
        .fold(HashMap::new(),
              |mut area_by_point: HashMap<String, i32>, point| {
                  if let Some(id_of_territory) = point.clone().get_territory_id() {
                      let entry = area_by_point.entry(id_of_territory).or_default();
                      *entry += 1;
                  }
                  area_by_point
              })
        .iter()
        .max_by_key(|&(_, area_of_territory)| area_of_territory)
        .expect("No territories found.");

    area_of_territory
}

// Part 2 - 0.24s
pub fn find_region() -> i32 {
    let points = load_points_from_coords_file();
    let grid = build_grid(&points);

    let region_size = grid.iter()
        .chain(grid.iter())  // The region can lie outwith the min-max bounding box!
        .enumerate()
        .fold(0, |acc, (row_idx, row)| {
            acc + row.iter()
                .chain(row.iter())
                .enumerate()
                .map(|(col_idx, cell)| {
                    let manhattan_sum_this_cell = points.iter()
                        .fold(0, |manhattan_sum, point|
                            manhattan_sum + manhattan_distance(&point, row_idx, col_idx),
                        );
                    manhattan_sum_this_cell
                })
                .filter(|manhattan_sum| *manhattan_sum < MANHATTAN_LIMIT)
                .count()
        });

    region_size as i32
}

fn build_grid(points: &Vec<GridCell>) -> Grid {
    let min_row = points.iter().min_by_key(|p| p.row().unwrap()).unwrap().row().unwrap();
    let min_col = points.iter().min_by_key(|p| p.col().unwrap()).unwrap().col().unwrap();
    let max_row = points.iter().max_by_key(|p| p.row().unwrap()).unwrap().row().unwrap() + 1;
    let max_col = points.iter().max_by_key(|p| p.col().unwrap()).unwrap().col().unwrap() + 1;

    let height = max_row - min_row;
    let width = max_col - min_col;

    // All points will be translated left/up by min_x/min_y
    let points: Vec<GridCell> = points.iter()
        .map(|p| GridCell::point(p.row().unwrap() - min_row, p.col().unwrap() - min_col))
        .collect();

    fill_grid_with(&points, height, width)
}

fn load_points_from_coords_file() -> Vec<GridCell> {
    BufReader::new(File::open(COORDS_FILE).unwrap())
        .lines()
        .map(|l| GridCell::from_str(&l.unwrap()).unwrap())
        .collect()
}

/// Construct a Grid, which surrounds all of the points (bounding box)
fn fill_grid_with(points: &Vec<GridCell>, height: usize, width: usize) -> Grid {
    let mut grid = points.iter()
        .fold(vec![vec![GridCell::Empty; width]; height],
              |mut grid, p| {
                  grid[p.row().unwrap()][p.col().unwrap()] = p.clone();
                  grid
              });

    populate_closest_points(&points, &mut grid);
    grid
}

fn populate_closest_points(points: &Vec<GridCell>, grid: &mut Grid) {
    for row_index in 0..grid.len() {
        let row = &grid[row_index];
        for col_index in 0..row.len() {
            // For this cell, find the Manhattan distances to all points
            let mut distances = points.iter()
                .map(|p| {
                    (p.id().unwrap(), manhattan_distance(&p, row_index, col_index))
                });

            let mut cell_territory = GridCell::in_territory_of(distances.clone().next().unwrap().0);
            let mut min_distance = distances.clone().next().unwrap().1;
            let mut min_distance_count = 0;

            for (point_id, manhattan) in distances.clone() {
                if manhattan < min_distance {
                    min_distance = manhattan;
                    cell_territory = GridCell::in_territory_of(point_id);
                    min_distance_count = 1;
                } else if manhattan == min_distance {
                    min_distance_count += 1;
                }
            }

            if min_distance_count > 1 {
                // If we're tied closest to multiple points, we're on a border cell
                grid[row_index][col_index] = GridCell::Border;
            } else {
                // Otherwise this cell is within the territory of a point
                grid[row_index][col_index] = cell_territory;
            }
        }
    }
}

fn points_with_infinite_territory(grid: &Grid) -> HashSet<String> {
    // Move around the outskirts of the grid, noting the closest points
    // The closest recorded points of each of these cells have an infinite area and should be ignored.
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    grid.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(move |(col_idx, cell)| {
                    let is_horizontal_edge = row_idx == 0usize || row_idx == num_rows - 1;
                    let is_vertical_edge = *col_idx == 0usize || *col_idx == num_cols - 1;
                    is_horizontal_edge || is_vertical_edge
                })
                .filter(|(col_idx, cell)| {
                    match cell {
                        GridCell::Empty => panic!("Empty cell after populating grid."),
                        GridCell::Border => false,
                        _ => true,
                    }
                })
                .map(|(col_idx, cell)| {
                    match cell {
                        GridCell::TerritoryOfPoint { point_id } => point_id.to_string(),
                        GridCell::Point { id, .. } => id.to_string(),
                        _ => panic!("Critical logic error.")
                    }
                })
        })
        .collect()
}

fn manhattan_distance(point: &GridCell, target_row: usize, target_col: usize) -> i32 {
    (point.col().unwrap() as i32 - target_col as i32).abs() +
        (point.row().unwrap() as i32 - target_row as i32).abs()
}