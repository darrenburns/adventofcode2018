use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const INITIAL_STATE: &'static str = "###......#.#........##.###.####......#..#####.####..#.###..#.###.#..#..#.#..#..#.##...#..##......#.#";
const NUM_GENERATIONS: i32 = 20;

pub fn get_living_plant_numbers_summed() -> i32 {
    let lines = BufReader::new(File::open("src/files/twelve/notes.txt").unwrap()).lines().map(|l| l.unwrap());

    let mut notes_map = lines.fold(HashMap::new(), |mut map, line| {
        let (key, value) = scan_fmt!(&line, "{} => {}", String, String);
        map.insert(key.unwrap(), value.unwrap());
        map
    });


    let mut current_generation = INITIAL_STATE.to_string();
    for generation in 1..NUM_GENERATIONS + 1 {
        let mut next_generation = "".to_string();
        let padded_current = "....".to_string() + &current_generation + "....";
        for plant in 2..padded_current.len() - 2 {
            let neighbours = padded_current[plant - 2..plant + 2 + 1].to_string();
            next_generation.push_str(notes_map.get(&neighbours).unwrap());
        }
        current_generation = next_generation.clone();
        println!("{:2}: {}", generation, current_generation);
    }

    let lowest_pot_num = -2 * NUM_GENERATIONS;
    (lowest_pot_num..current_generation.len() as i32)
        .zip(current_generation.chars())
        .filter(|&(pot_num, ch)| ch == '#')
        .map(|(pot_num, _)| {
            pot_num
        })
        .sum()
}
