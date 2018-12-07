use std::fs::read_to_string;
use std::str::Chars;

const POLYMER_FILE: &'static str = "src/files/five/polymer.txt";
const UNITS: &'static str = "abcdefghijklmnopqrstuvwxyz";

// Time: 0.23s
pub fn length_of_final_string() -> usize {
    // part 1: react().len()

    // part 2:
    let (c, filtered_len) = UNITS.chars()
        .map(|c| (c, react_filtered(c).len()))
        .min_by_key(|&(c, filtered_len)| {
            filtered_len
        })
        .unwrap();

    println!("Filtering unit {} produces shortest final polymer of length {}.", c, filtered_len);
    filtered_len
}


fn react() -> String {
    polymer().chars().fold(String::new(), |mut polymer, unit| react_with_unit(&mut polymer, unit))
}

fn react_filtered(skip: char) -> String {
    polymer().chars()
        .filter(|&c| c != skip && c != skip.to_ascii_uppercase())
        .fold(String::new(), |mut polymer, unit| react_with_unit(&mut polymer, unit))
}

fn polymer() -> String {
    read_to_string(POLYMER_FILE).unwrap()
}

fn polar_opposites(a: char, b: char) -> bool {
    a.is_uppercase() && a.to_ascii_lowercase() == b ||
        a.is_lowercase() && a.to_ascii_uppercase() == b
}

fn react_with_unit(polymer: &mut String, unit: char) -> String {
    if let Some(prev_unit) = polymer.chars().rev().next() {
        if polar_opposites(prev_unit, unit) {
            polymer.pop();
        } else {
            polymer.push(unit);
        }
    } else {
        polymer.push(unit);
    }
    polymer.to_string()
}