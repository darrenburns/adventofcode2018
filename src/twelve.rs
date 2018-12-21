use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const INITIAL_STATE: &'static str = "###......#.#........##.###.####......#..#####.####..#.###..#.###.#..#..#.#..#..#.##...#..##......#.#";
const NUM_GENERATIONS: i32 = 20;
const FIFTY_BILLION: i64 = 50_000_000_000;

pub fn get_living_plant_numbers_summed() -> i64 {
    let lines = BufReader::new(File::open("src/files/twelve/notes.txt").unwrap()).lines()
        .map(|l| l.unwrap());

    let mut notes_map = lines.fold(HashMap::new(), |mut map, line| {
        let (key, value) = scan_fmt!(&line, "{} => {}", String, String);
        map.insert(key.unwrap(), value.unwrap());
        map
    });


    let mut seen_plants = HashSet::new();
    let mut current_generation = INITIAL_STATE.to_string();
    let mut generation = 0;
    loop {
        let mut next_generation = "".to_string();
        let padded_current = "....".to_string() + &current_generation + "....";
        for plant in 2..padded_current.len() - 2 {
            let neighbours = padded_current[plant - 2..plant + 2 + 1].to_string();
            next_generation.push_str(notes_map.get(&neighbours).unwrap());
        }

        let just_plants = just_plants(current_generation.clone());
        if seen_plants.contains(&just_plants) {
            // We've reached the point of convergence
            let num_remaining_generations = FIFTY_BILLION - generation;
            let current_gen_sum = sum_plants(-2 * generation, current_generation.clone());
            let next_gen_sum = sum_plants(-2 * (generation + 1), next_generation.clone());
            let increase_each_gen = next_gen_sum - current_gen_sum;
            let sum_for_remaining_gens = num_remaining_generations * increase_each_gen;
            let final_sum = current_gen_sum + sum_for_remaining_gens;
            println!("gen [{}] current_sum = {} increase_each_gen = {}, num_remaining_gens = {}", generation, current_gen_sum, increase_each_gen, num_remaining_generations);
            return final_sum;
        } else {
            seen_plants.insert(just_plants);
        }

        current_generation = next_generation;
        generation += 1;
    }
}

fn sum_plants(lowest_pot_num: i64, generation: String) -> i64 {
    (lowest_pot_num..generation.len() as i64)
        .zip(generation.chars())
        .filter(|&(pot_num, ch)| ch == '#')
        .map(|(pot_num, _)| {
            pot_num
        })
        .sum::<i64>()
}

fn just_plants(generation: String) -> String {
    generation.replace(".", " ").trim().replace(" ", ".")
}

// Eventually the pattern will converge, and the plants will shift along
// by a single pot each generation. As a result, the sum will increase by
// a consistent amount each time (in my case 42 each time).

// From the point where we reach this convergence, find how many generations we are
// from 50 billion (50 billion - generation), and multiply this by 42, then add
// this on to the existing sum.