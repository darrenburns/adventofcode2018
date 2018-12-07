use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

const END_OF_TIMESTAMP_INDEX: usize = 19;


// Time: 0.09s
pub fn checksum() -> i32 {
    let file = File::open("src/files/four/guard_shifts.txt").unwrap();
    let mut lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    lines.sort();

    let mut guards = HashMap::new();
    let mut guard_id: i32 = 0;
    let mut sleep_start_min: i32 = 0;
    for line in lines {
        let minute: i32 = line[15..17].parse().unwrap();
        let mut chars = line.chars();
        let action = Act::from_str(&chars.nth(END_OF_TIMESTAMP_INDEX).unwrap().to_string()).unwrap();
        match action {
            Act::StartShift => {
                guard_id = chars
                    .skip_while(|&c| c != '#')
                    .skip(1)
                    .take_while(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse()
                    .unwrap();
                guards.entry(guard_id).or_insert(vec![0; 60]);
            }
            Act::FallAsleep => {
                sleep_start_min = minute;
            }
            Act::WakeUp => {
                let mins = guards.get_mut(&guard_id).unwrap();
                for min in sleep_start_min..minute {
                    mins[min as usize] += 1;
                }
            }
            _ => ()
        };
    }

    let guard_sleep_stats = guards.iter()
        .map(|(guard_id, mins)| {
            let total_mins_slept = mins.iter().sum::<i32>();
            let (most_popular_minute, num_repeats) = mins.iter()
                .enumerate()
                .max_by_key(|&(_, min)| min)
                .map(|(i, num_repeats)| (i as i32, num_repeats))
                .unwrap();
            (guard_id, total_mins_slept, most_popular_minute, num_repeats)
        });

    // For part 1
    let sleepiest_guard = guard_sleep_stats.clone().max_by_key(|&(_, total_mins_asleep, ..)| total_mins_asleep);
    let (gid, _, most_popular_minute, _) = sleepiest_guard.unwrap();

    // For part 2
    let single_min_repeated_most_guard = guard_sleep_stats.max_by_key(|&(.., num_repeats)| num_repeats);
    let (gid, _, most_popular_minute, _) = single_min_repeated_most_guard.unwrap();
    gid * most_popular_minute

}

enum Act { StartShift, FallAsleep, WakeUp }

impl FromStr for Act {
    type Err = ();

    fn from_str(s: &str) -> Result<Act, ()> {
        match s {
            "G" => Ok(Act::StartShift),
            "f" => Ok(Act::FallAsleep),
            "w" => Ok(Act::WakeUp),
            _ => Err(()),
        }
    }
}
