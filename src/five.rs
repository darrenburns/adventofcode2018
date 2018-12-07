use std::fs::read_to_string;
use std::str::Chars;

const POLYMER_FILE: &'static str = "src/files/five/polymer.txt";
const UNITS: &'static str = "abcdefghijklmnopqrstuvwxyz";

// Time: Part 1: 0.15s, Part 2: 0.54s
pub fn length_of_final_string() -> usize {
    let reacted = react(read_to_string(POLYMER_FILE).unwrap());

    // part 2:
    let (c, filtered_len) = UNITS.chars()
        .map(move |c| (c, react_filtered(&reacted, c).len()))
        .min_by_key(|&(c, filtered_len)| {
            filtered_len
        })
        .unwrap();

    println!("Filtering unit {} produces shortest final polymer of length {}.", c, filtered_len);
    filtered_len
}


fn react(polymer: String) -> String {
    polymer.chars().fold(String::new(), |mut polymer, unit| react_with_unit(&mut polymer, unit))
}

fn react_filtered(polymer: &String, skip: char) -> String {
    polymer.chars()
        .filter(|&c| c != skip && c != skip.to_ascii_uppercase())
        .fold(String::new(), |mut polymer, unit| react_with_unit(&mut polymer, unit))
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