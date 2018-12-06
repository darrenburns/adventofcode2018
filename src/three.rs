use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Tiles = Vec<Vec<i32>>;

struct Location {
    row: u16,
    col: u16,
}

struct Dimension {
    height: u16,
    width: u16,
}

struct Claim {
    location: Location,
    dimensions: Dimension,
}

// Time: 0.20s
pub fn find_num_square_inches_overlapping() -> usize {
    // How many square inches of tiles are within two or more claims
    // i.e. how many elements of the matrix have a value > 1
    let mut tiles = vec![vec![0; 1024]; 1024];

    let file = File::open("src/files/three/claims.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());

    for line in lines {
        let claim = deserialise_claim(&line);
        add_layer_to_tiles(&mut tiles, claim);
    }

    count_overlaps(tiles)
}

// #1 @ 1,3: 4x4
fn deserialise_claim(line: &str) -> Claim {
    let mut split_line = line.split_whitespace();

    let raw_location_segment = split_line.clone().skip(2).next().unwrap();
    let raw_location = &raw_location_segment[..&raw_location_segment.len() - 1];
    let mut location_split = raw_location.split(",");
    let row = location_split.next().unwrap().parse::<u16>().unwrap();
    let col = location_split.next().unwrap().parse::<u16>().unwrap();
    let location = Location { row, col };

    let raw_dimensions = split_line.skip(3).next().unwrap();
    let mut dimension_split = raw_dimensions.split("x");
    let height_raw = dimension_split.next().unwrap();
    let height = height_raw.parse::<u16>().unwrap();
    let width = dimension_split.next().unwrap().parse::<u16>().unwrap();
    let dimensions = Dimension { height, width };

    Claim { location, dimensions }
}

fn add_layer_to_tiles(tiles: &mut Tiles, claim: Claim) {
    let Claim { location, dimensions } = claim;
    let Location { row, col } = location;
    let Dimension { height, width } = dimensions;

    for row_idx in row..(row + height) {
        let row = tiles.get_mut(row_idx as usize).unwrap();

        for col_idx in col..(col + width) {
             row[col_idx as usize] += 1;
        }
    }
}

fn count_overlaps(tiles: Tiles) -> usize {
    tiles.iter().flatten().filter(|t| **t > 1).count()
}
