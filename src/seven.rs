use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

const INSTRUCTIONS_FILE: &'static str = "src/files/seven/instructions.txt";
const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Copy, Clone)]
struct Dependency {
    must_do: char,
    to_unblock: char,
}

impl FromStr for Dependency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = "Unable to parse dependency string.";
        Ok(Dependency {
            must_do: s.chars().nth(5).expect(err_msg),
            to_unblock: s.chars().nth(36).expect(err_msg),
        })
    }
}


pub fn get_instruction_ordering() -> String {
    let dependencies = get_dependencies_from_file();

    let (mut is_blocking, mut is_blocked_by) = dependencies.iter()
        .fold((HashMap::new(), HashMap::new()), |mut maps, dep| {
            maps.0
                .entry(dep.must_do)
                .or_insert(vec![])
                .push(dep.to_unblock);

            maps.1
                .entry(dep.to_unblock)
                .or_insert(vec![])
                .push(dep.must_do);
            maps
        });

    let mut step = ALPHABET.chars()
        .find(|step| !is_blocked_by.contains_key(step))
        .expect("No initial step found. All steps are waiting on other steps to be completed.");


    let is_blocking_clone = is_blocking.clone();
    ALPHABET.chars()
        .filter(|step| !is_blocking_clone.contains_key(step))
        .for_each(|step| {
            is_blocking.insert(step, vec![]);
        });

    let is_blocked_by_clone = is_blocked_by.clone();
    ALPHABET.chars()
        .filter(|step| !is_blocked_by_clone.contains_key(step))
        .for_each(|step| {
            is_blocked_by.insert(step, vec![]);
        });

    let mut ordering = step.to_string();
    while ordering.len() < is_blocked_by.len() {
        let steps_blocked = is_blocking.get(&step).unwrap();
        for &blocked_step in steps_blocked {
            is_blocked_by
                .entry(blocked_step)
                .and_modify(|block_list| {
                    block_list
                        .remove(block_list.iter().position(|&s| s == step).expect(
                            "Blockers & pre-requisite maps are out of sync."
                        ));
                });
        }

        let steps_seen = ordering.chars().collect::<HashSet<char>>();
        step = get_next_non_blocked(&is_blocked_by, &steps_seen);

        ordering.push(step);
    }

    ordering
}

fn get_next_non_blocked(
    is_blocked_by: &HashMap<char, Vec<char>>,
    steps_seen: &HashSet<char>,
) -> char {
    *is_blocked_by.iter()
        .filter(|(step, blocked_by)| blocked_by.is_empty() && !steps_seen.contains(step))
        .map(|(step, _)| step)
        .min()
        .expect("Unable to find a step without any blockers.")
}

fn get_dependencies_from_file() -> Vec<Dependency> {
    let file = File::open(INSTRUCTIONS_FILE).unwrap();
    BufReader::new(file).lines()
        .map(|l| Dependency::from_str(&l.unwrap()).expect("Unable to parse dependency in file."))
        .collect()
}
