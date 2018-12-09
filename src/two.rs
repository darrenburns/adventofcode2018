// Count the number of lines that contain any of the same character twice
// Count the number of lines that contain any of the same character thrice
// Multiply the above 2 numbers to calculate the checksum.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct RepeatCounts {
    two: i32,
    three: i32,
}

// Time: 0.01s
pub fn calculate_checksum() -> i32 {
    let file = File::open("src/files/two/boxes.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let repeat_counts = lines.fold(RepeatCounts { two: 0, three: 0 }, |mut repeats, line| {
        // Build a character frequency histogram for the line
        let counts = line.unwrap().chars()
            .fold(HashMap::new(), |mut char_counts, char| {
                let counter = char_counts.entry(char).or_insert(0);
                *counter += 1;
                char_counts
            });

        // Check if the line has any characters with two or three occurrences,
        // but make sure we only count them once per line.
        let mut has_double = false;
        let mut has_triple = false;
        for (_char, count) in counts {
            if has_double && has_triple {
                break;
            }
            if count == 2 && !has_double {
                has_double = true;
                repeats.two += 1;
            } else if count == 3 && !has_triple {
                has_triple = true;
                repeats.three += 1;
            }
        }

        repeats
    });

    repeat_counts.two * repeat_counts.three
}


// Time: 0.01s
pub fn find_common_letters() -> String {
    let file = File::open("src/files/two/boxes.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Unable to read line."))
        .collect();

    for line_idx in 0..lines.len() {
        if line_idx != lines.len() - 1 {
            let left_line = &lines[line_idx];
            for right_line in &lines[line_idx + 1..] {
                if has_single_char_difference(left_line, right_line) {
                    return get_common_chars(left_line, right_line)
                }
            }
        }
    }
    "".to_string()
}

fn has_single_char_difference(left: &str, right: &str) -> bool {
    // All lines have the same length, so no need to check that.
    let mut num_chars_different = 0;

    let left_chars = left.chars();
    let mut right_chars = right.chars();

    for left in left_chars {
        let right = right_chars.next().unwrap();  // we know left.len() == right.len()
        if left != right {
            num_chars_different += 1;
            if num_chars_different > 1 {
                return false;
            }
        }
    }
    num_chars_different == 1
}

fn get_common_chars(left: &str, right: &str) -> String {
    let mut common_chars = "".to_string();
    let mut right_chars = right.chars();
    for left_char in left.chars() {
        if left_char == right_chars.next().unwrap() {
            common_chars.push(left_char);
        }
    }
    common_chars
}
