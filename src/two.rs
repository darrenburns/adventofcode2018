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

// Time: 0.09s
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
        for (char, count) in counts {
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