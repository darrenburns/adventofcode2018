use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::usize;

use uuid::Uuid;

const COORDS_FILE: &'static str = "src/files/six/example.txt";

type Grid = Vec<Vec<Point>>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point { id: String, x: usize, y: usize, closest_id: Option<String> }

impl Point {
    fn new() -> Self {
        Point { id: Uuid::new_v4().to_string(), x: 0, y: 0, closest_id: None }
    }
    fn with_owner(closest_id: String) -> Self {
        Point { id: Uuid::new_v4().to_string(), x: 0, y: 0, closest_id: Some(closest_id) }
    }
    fn self_owned(x: usize, y: usize) -> Self {
        let uuid = Uuid::new_v4().to_string();
        Point { id: uuid.clone(), x, y, closest_id: Some(uuid) }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: usize = s.chars()
            .take_while(|c| c.to_string() != ",")
            .collect::<String>()
            .parse()
            .unwrap();
        let y: usize = s.chars()
            .skip_while(|c| !c.is_whitespace())
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap();

        Ok(Point::self_owned(x - 1, y - 1))
    }
}

pub fn get_largest_area() -> i32 {
    let points = get_points();
    let grid = make_grid(&points);
    let ignore_ids = infinite_area_points_ids(&grid);

    println!("{:?}", ignore_ids);

    let areas = grid.iter()
        .flatten()
        .filter(move |p| ignore_ids.contains(&p.id))
        .fold(HashMap::new(),
              |mut area_by_point: HashMap<String, i32>, point| {
                  let entry = area_by_point.entry(point.clone().id).or_default();
                  *entry += 1;
                  area_by_point
              });


    println!("{:?}", areas);
    1
}

fn get_points() -> Vec<Point> {
    BufReader::new(File::open(COORDS_FILE).unwrap())
        .lines()
        .map(|l| Point::from_str(&l.unwrap()).unwrap())
        .collect()
}

/// Construct a Grid, which surrounds all of the points (bounding box)
fn make_grid(points: &Vec<Point>) -> Grid {
    let min_x = points.iter().min_by_key(|p| p.x).unwrap().x;
    let min_y = points.iter().min_by_key(|p| p.y).unwrap().y;
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x + 1;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y + 1;

    println!("minx {}, miny {}, maxx {}, maxy {}", min_x, min_y, max_x, max_y);

    // All points will be translated left/up by min_x/min_y
    let height = max_y - min_y;
    let width = max_x - min_x;

    println!("height {}, width {}", height, width);

    let mut grid = points.iter()
        .fold(vec![vec![Point::new(); width]; height],
              |mut grid, point| {
                  let trans_x = point.x - min_x;
                  let trans_y = point.y - min_y;
                  grid[trans_y][trans_x] = Point::self_owned(trans_x, trans_y);
                  grid
              });

    populate_closest_points(&points, &mut grid);

    println!("{:?}", grid);
    grid
}

fn populate_closest_points(points: &Vec<Point>, grid: &mut Grid) {
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            // For this cell, find the Manhattan distances to all points
            let mut distances = points.iter()
                .map(|p| {
                    /*
                    ..........
                    .A........
                    ..........
                    .......C..
                    x..D......
                    .....E....
                    .B........
                    ..........
                    ..........
                    ........F.
                    */
                    let manhattan = (p.x as i32 - x as i32).abs() + (p.y as i32 - y as i32).abs();
//                    println!(
//                        "({}, {}) has Manhattan distance {} to point at ({}, {})",
//                        x, y, manhattan, p.x, p.y
//                    );

                    (p, manhattan)
                })
                .filter(|(_, dist)| *dist > 0);

            let mut min_distance = distances.clone().next().unwrap().1;
            let mut min_distance_count = 0;
            for (point, manhattan) in distances.clone() {
                if manhattan < min_distance {
                    min_distance = manhattan;
                    min_distance_count = 1;
                } else if manhattan == min_distance {
                    min_distance_count += 1;
                }
            }

            /*
                    Aaaa.ccc
                    aaddeccc
                    adddeccC
                    .dDdeecc
                    b.deEeec
                    Bb.eeee.
                    bb.eeeff
                    bb.eefff
                    bb.ffffF
            */

            if grid[y][x].closest_id.is_none() {
                if min_distance_count > 1 {
                    println!("Setting grid[{}][{}] = None", y, x);
                    grid[y][x].closest_id = None;
                } else {
                    let (closest, dist) = distances
                        .min_by_key(|&(_, manhattan_distance)| manhattan_distance)
                        .unwrap();
                    println!("Setting grid[{}][{}] closest point at grid[{}][{}]", y, x, closest.y, closest.x);
                    grid[y][x].closest_id = Some(closest.clone().id);
                }
            }
        }
    }
}

fn infinite_area_points_ids(grid: &Grid) -> HashSet<String> {
    // Move around the outskirts of the grid, noting any points
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    grid.iter()
        .enumerate()
        .flat_map(move |(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(move |(col_idx, point)| {
                    let is_horizontal_edge = row_idx == 0usize || row_idx == num_rows - 1;
                    let is_vertical_edge = *col_idx == 0usize || *col_idx == num_cols - 1;
                    is_horizontal_edge || is_vertical_edge
                })
                .map(|(col_idx, p)| p.clone().id)
        })
        .collect()
}