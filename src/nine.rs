use std::collections::VecDeque;

const NUM_PLAYERS: usize = 432;
const LAST_MARBLE_VALUE: usize = 7101900;

pub fn winning_elf_score() -> usize {

    let mut ring = VecDeque::new();  // current marble is always at the front of the deque
    let mut player_scores = vec![0; NUM_PLAYERS];  // player_scores[i] is the score of elf number i

    ring.push_front(0);

    for marble_number in 1..LAST_MARBLE_VALUE {
        if marble_number % 23 == 0 {
            // move 7 marbles counterclockwise
            for _ in 0..7 {
                let back = ring.pop_back().expect("cannot pop marble at back");
                ring.push_front(back);
            }
            let points = marble_number + ring.pop_front().expect("cannot pop marble at front");
            player_scores[marble_number % NUM_PLAYERS] += points
        } else {
            // move 2 marbles clockwise
            for _ in 0..2 {
                let front = ring.pop_front().expect("cannot pop marble at back");
                ring.push_back(front);
            }
            ring.push_front(marble_number);
        }
    }

    *player_scores.iter().max().unwrap()
}