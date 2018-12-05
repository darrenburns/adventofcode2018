use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn calculate_frequency() -> Result<i32> {
    let file = File::open("src/files/one/frequencies.txt")?;
    BufReader::new(file).lines()
        .fold(Ok(0), |acc, line| Ok(acc? + line?.parse::<i32>().unwrap()))
}

