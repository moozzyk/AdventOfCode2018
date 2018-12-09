use std::collections::HashMap;
use std::collections::VecDeque;

// Circular list would likely be much simpler and faster
fn problem(num_players: u32, last_marble: u32) -> u32 {
    let mut score = HashMap::new();
    let mut circle = vec![0];
    let mut current_idx = 0;
    let mut marble = 1;
    let mut player = 1;
    while marble <= last_marble {
        if marble % 23 != 0 {
            let insert_after = current_idx + 1;
            current_idx = insert_after % circle.len() + 1;
            circle.insert(current_idx, marble);
        } else {
            current_idx = (circle.len() + current_idx - 7) % circle.len();
            let removed = circle.remove(current_idx);
            if !score.contains_key(&player) {
                score.insert(player, 0);
            }
            *score.get_mut(&player).unwrap() += marble + removed;
        }

        marble += 1;
        player += 1;
        if player > num_players {
            player = 1;
        }
    }

    return *score.values().max().unwrap();
}

// Reddit FTW
fn problem_fast(num_players: usize, last_marble: u32) -> u32 {
    let mut player = 0;
    let mut score = vec![0; num_players];

    let mut circle = VecDeque::new();
    circle.push_back(0);
    for marble in 1..=last_marble {
        if marble % 23 != 0 {
            for _ in 1..=2 {
                let m = circle.pop_front().unwrap();
                circle.push_back(m);
            }
            circle.push_front(marble);
        } else {
            for _ in 1..7 {
                let m = circle.pop_back().unwrap();
                circle.push_front(m);
            }

            score[player % num_players] += marble +
circle.pop_back().unwrap();
        }
        player += 1;
    }

    return *score.iter().max().unwrap();
}

fn main() {
    println!("{}", problem(464, 71730));
    println!("{}", problem_fast(464, 71730 * 100));
}
