use std::fs::read_to_string;
use std::str::Chars;

use rayon::prelude::*;

const POLYMER_FILE: &'static str = "src/files/five/polymer.txt";
const UNITS: &'static str = "abcdefghijklmnopqrstuvwxyz";

// Time: Part 1: 0.09s, Part 2: 0.14s
pub fn length_of_final_string() -> usize {
    let reacted = react(read_to_string(POLYMER_FILE).unwrap());

    // part 2:
    let (c, filtered_len) = UNITS.par_chars()
        .map(move |c| (c, react_filtered(&reacted, c).len()))
        .min_by_key(|&(_, filtered_len)| filtered_len)
        .unwrap();

    println!("Filtering unit {} produces shortest final polymer of length {}.", c, filtered_len);
    filtered_len
}


fn react(polymer: String) -> String {
    polymer.chars()
        .fold(".".to_string(), |mut polymer, unit| react_with_unit(&mut polymer, unit))
}

fn react_filtered(polymer: &String, skip: char) -> String {
    polymer.chars()
        .filter(|&c| c != skip && c != skip.to_ascii_uppercase())
        .fold(".".to_string(), |mut polymer, unit| react_with_unit(&mut polymer, unit))
}

fn polar_opposites(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn react_with_unit(polymer: &mut String, unit: char) -> String {
    let prev_unit = polymer.chars().last().unwrap();
    if polar_opposites(prev_unit, unit) {
        polymer.pop();
    } else {
        polymer.push(unit);
    }
    polymer.to_string()
}