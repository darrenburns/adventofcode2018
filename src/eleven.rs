const GRID_SERIAL_NUMBER: usize = 2866;
const GRID_SQUARE_SIZE: usize = 300;
const SEARCH_SQUARE_SIZE: usize = 3;


#[derive(Debug, Default, Clone, Copy)]
pub struct FuelCell {
    pub x: usize,
    pub y: usize,
    pub rack_id: usize,
    pub power_level: isize,
}

pub fn get_optimal_cell_coords() -> FuelCell {
    let mut grid = vec![vec![FuelCell::default(); GRID_SQUARE_SIZE]; GRID_SQUARE_SIZE];

    // Build the grid of fuel cells
    for y_index in 0..GRID_SQUARE_SIZE {
        for x_index in 0..GRID_SQUARE_SIZE {
            let x = x_index + 1;
            let y = y_index + 1;
            let rack_id = x + 10;
            let mut power_level = ((rack_id * y) + GRID_SERIAL_NUMBER) as isize;
            power_level *= rack_id as isize;
            power_level = (power_level % 1000) / 100;
            power_level -= 5;
            grid[y_index][x_index] = FuelCell { x, y, rack_id, power_level };
        }
    }

    let mut best_cell = grid[0][0];
    let mut max_subgrid_power = 0;
    let mut optimum_subgrid_size = 0;
    for search_square_size in (0..GRID_SQUARE_SIZE) {
        println!("Searching subgrids of size {}", search_square_size);
        for y_index in 0..GRID_SQUARE_SIZE - search_square_size - 1 {
            for x_index in 0..GRID_SQUARE_SIZE - search_square_size - 1 {
                let mut this_subgrid_power = 0;
                for y in y_index..y_index + search_square_size {
                    for x in x_index..x_index + search_square_size {
                        this_subgrid_power += grid[y][x].power_level;
                    }
                }
                if this_subgrid_power > max_subgrid_power {
                    max_subgrid_power = this_subgrid_power;
                    best_cell = grid[y_index][x_index];
                    optimum_subgrid_size = search_square_size;
                }
            }
        }
    }

    println!("Subgrid size of best cell = {}", optimum_subgrid_size);
    best_cell
}
