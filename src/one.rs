use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, SeekFrom, Seek};


// Time: 0.07s
pub fn calculate_frequency() -> Result<i32> {
    let file = File::open("src/files/one/frequencies.txt")?;
    BufReader::new(file).lines()
        .fold(Ok(0), |acc, line| Ok(acc? + line?.parse::<i32>().unwrap()))
}

// Time: 0.95s
pub fn first_repeated_frequency() -> i32 {
    let mut file = File::open("src/files/one/frequencies.txt").unwrap();
    let mut seen_totals: HashSet<i32> = [0].iter().cloned().collect();
    let mut total = 0;
    loop {
        let lines = BufReader::new(&file).lines();
        for line in lines {
            total += line.unwrap().parse::<i32>().unwrap();
            if seen_totals.contains(&total) {
                return total;
            }
            seen_totals.insert(total);
        }
        file.seek(SeekFrom::Start(0));
    }
}