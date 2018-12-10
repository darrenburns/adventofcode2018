/*
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
*/

/*
 Build a map of pre-requisites and follow it e.g. map C -> [A, F] above,
 but only go one letter at a time. We're can perform a slightly modified breadth first search here.
 The values in this map should always be sorted.

MAP
 C -> [A, F]  # i.e. C is a pre-requisite of A and F
 A -> [B, D]
 B -> [E]
 D -> [E]
 F -> [E]

Also build a map of "blockers", i.e. the inverse of the above. The values in this don't need to be sorted
 E -> [B, D, F]  # the pre-requisites of E are B, D, and F. We cannot use E (E is blocked) until these have been satisfied.
 A -> [C]
 F -> [C]
 and so on ...

QUEUE:
  E, F

For each element of the queue, we need to store it's pre-requisites. We can only pop an item
from the queue if all of its prerequisites are met.

How do we find where to start?
- Only one letter will not appear as a key in our pre-requisites map. This is the starting letter.


find the only element with no pre-requisites (C)
goto C
lookup the first map to see what C is a pre-requisite of, and find A, F
for each P of the pre-requisites A, F:
    remove C as a blocker for P

pop off first elem with all met pre-requisites (A)  # i.e. an empty "blocker" list (or all blockers marked resolved)
goto A
pop off first step S in pre_requisites.get(A) such that blockers.get(S).is_empty
goto B
pop off first elem with all met pre-requisites (D)
goto D

and so on...

*/

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

const INSTRUCTIONS_FILE: &'static str = "src/files/seven/instructions.txt";

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Step(char);

// We want to use the BinaryHeap, which is a max-heap so that we always use the character
// that comes first lexicographically, so we define the ordering implementations such that
// they are reversed (and so the heap is essentially used as a min-heap).
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


#[derive(Debug, Copy, Clone)]
struct Dependency {
    must_do: Step,
    to_unblock: Step,
}

impl FromStr for Dependency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = "Unable to parse dependency string.";
        Ok(Dependency {
            must_do: Step(s.chars().nth(5).expect(err_msg)),
            to_unblock: Step(s.chars().nth(36).expect(err_msg)),
        })
    }
}

pub fn get_instruction_ordering() -> String {
    let dependencies = get_dependencies_from_file();

    let (pre_requisites, mut blockers) = dependencies.iter()
        .fold((HashMap::new(), HashMap::new()), |mut maps, dep| {
            maps.0
                .entry(dep.must_do)
                .or_insert(BinaryHeap::new())
                .push(dep.to_unblock);
            maps.1
                .entry(dep.to_unblock)
                .or_insert(vec![])
                .push(dep.must_do);
            maps
        });

    println!("Pre-requisites (k is required by v) = {:?}", pre_requisites);
    println!("Blockers (k is blocked by v) = {:?}", blockers);

    let blockers_clone = blockers.clone();
    let pre_req_set: HashSet<&Step> = pre_requisites.keys().collect();
    let blocker_set: HashSet<&Step> = blockers_clone.keys().collect();

    let mut step = *pre_req_set.difference(&blocker_set).next().expect("Multiple initial steps possible.");
    let final_step = *blocker_set.difference(&pre_req_set).next().expect("Multiple final steps possible.");

    println!("Initial step is = {:?}, final step is = {:?}", step, final_step);

    let mut ordering = step.0.to_string();
    while ordering.len() < dependencies.len() - 1 {
        println!("Ordering is: {}", ordering);
        println!("Step is {:?}", step);

        // This step had blocked some other steps.
        // Those steps are now updated to reflect that this is no longer the case.
        let steps_blocked = pre_requisites.get(step).unwrap();
        println!("Step is blocking {:?}", steps_blocked);
        steps_blocked.iter()
            .for_each(|&blocked_step| {
                blockers
                    .entry(blocked_step)
                    .and_modify(|block_list| {
                        block_list
                            .remove(block_list.iter().position(|s| s == step).expect(
                                "Blockers & pre-requisite maps are out of sync."
                            ));
                    });
            });

        println!("{:?} is blocked by {:?}", steps_blocked, step);

        // Get the first step which was required the current step to be done, and has no blockers
        // Completion of this step will have unlocked at least one of them.
        let next_step = steps_blocked.iter()
            .find(|step| {
                println!("--> Looking for step {:?} that no longer has any blockers in {:?}", step, blockers);
                blockers.get(step).expect("Blockers out of sync").is_empty()
            });


        // TODO: This block is almost certainly wrong.
        if let Some(found_step) = next_step {
            step = found_step;
            ordering.push(found_step.0);
        } else {
            let penultimate_steps: String = blockers.get(final_step).unwrap().iter()
                .map(|step| step.0)
                .collect();
            ordering.push_str(&penultimate_steps);
            ordering.push(final_step.0);
        }

    }
//    println!("dependencies: {:?}", dependencies.collect::<Vec<Dependency>>());

    ordering
}


fn get_dependencies_from_file() -> Vec<Dependency> {
    let file = File::open(INSTRUCTIONS_FILE).unwrap();
    BufReader::new(file).lines()
        .map(|l| Dependency::from_str(&l.unwrap()).expect("Unable to parse dependency in file."))
        .collect()
}

