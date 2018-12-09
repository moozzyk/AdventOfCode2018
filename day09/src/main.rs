use std::collections::HashMap;

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

fn main() {
    println!("{}", problem(464, 71730));
    println!("{}", problem(464, 71730 * 100));
}
