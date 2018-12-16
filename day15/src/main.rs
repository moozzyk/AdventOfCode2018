use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::Ordering;

fn lines_from_file<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

#[derive(Debug)]
struct Creature
{
    row: usize,
    col: usize,
    race: char,
    hits: i32,
    attack: i32,
}

fn get_creatures(map: &Vec<Vec<char>>, elf_attack: i32, gnome_attack: i32) -> Vec<Creature> {
    let mut creatures = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'G' {
                creatures.push(Creature{row: row, col: col, race: map[row][col], hits: 200, attack: gnome_attack});
            } else if map[row][col] == 'E' {
                creatures.push(Creature{row: row, col: col, race: map[row][col], hits: 200, attack: elf_attack});
            }
        }
    }

    return creatures;
}

fn try_attack(map: &mut Vec<Vec<char>>, creatures: &mut Vec<Creature>, attacker_idx: usize) -> bool {
    let enemy_race = if creatures[attacker_idx].race == 'G' { 'E' } else { 'G' };
    let mut candidate_idx = usize::max_value();

    { // just to prevent 'cannot borrow `*creatures` as mutable because it is also borrowed as immutable'
        let creature = &creatures[attacker_idx];
        for i in 0..creatures.len() {
            if creatures[i].race == enemy_race && creatures[i].hits > 0 &&
               ((creatures[i].row == creature.row - 1 && creatures[i].col == creature.col) ||
                (creatures[i].row == creature.row && creatures[i].col == creature.col - 1) ||
                (creatures[i].row == creature.row && creatures[i].col == creature.col + 1) ||
                (creatures[i].row == creature.row + 1 && creatures[i].col == creature.col)) {
                    if candidate_idx == usize::max_value() {
                        candidate_idx = i;
                    } else {
                        if (creatures[candidate_idx].hits > creatures[i].hits) ||
                            (creatures[candidate_idx].hits == creatures[i].hits &&
                                (creatures[candidate_idx].row > creatures[i].row ||
                                    (creatures[candidate_idx].row == creatures[i].row && creatures[candidate_idx].col > creatures[i].col))) {
                            candidate_idx = i;
                        }
                    }
            }
        }
    }

    if candidate_idx == usize::max_value() {
        // println!("{:?} has no one to attack", creatures[attacker_idx]);
        return false;
    }

    // println!("{:?} attacks {:?}", creatures[attacker_idx], creatures[candidate_idx]);


    let mut damage = 0; { damage = creatures[attacker_idx].attack; }
    let creature = &mut creatures[candidate_idx];
    creature.hits -= damage;
    if creature.hits <= 0 {
        map[creature.row][creature.col] = '.';
    }

    return true;
}

fn num_steps(map: &mut Vec<Vec<char>>, from: (usize, usize), to: (usize, usize)) -> (usize, (usize, usize), (usize, usize)) {
    if map[to.0][to.1] != '.' {
        return (usize::max_value(), (usize::max_value(), usize::max_value()), (usize::max_value(), usize::max_value()));
    }

    // println!("({}, {}) -> ({}, {})", from.0, from.1, to.0, to.1);

    let mut visited = HashSet::new();
    visited.insert((from.0, from.1));

    let mut queue = VecDeque::new();
    queue.push_back((from.0 - 1, from.1, (from.0 - 1, from.1), 1));
    queue.push_back((from.0, from.1 - 1, (from.0, from.1 - 1), 1));
    queue.push_back((from.0, from.1 + 1, (from.0, from.1 + 1), 1));
    queue.push_back((from.0 + 1, from.1, (from.0 + 1, from.1), 1));

    while !queue.is_empty() {
        let (row, col, initial_move, steps) = queue.pop_front().unwrap();

        if row == to.0 && col == to.1 {
            return (steps, to, initial_move);
        }

        if visited.contains(&(row, col)) || map[row][col] != '.' {
            continue;
        }

        visited.insert((row, col));

        queue.push_back((row - 1, col, initial_move, steps + 1));
        queue.push_back((row, col - 1, initial_move, steps + 1));
        queue.push_back((row, col + 1, initial_move, steps + 1));
        queue.push_back((row + 1, col, initial_move, steps + 1));
    }

    return (usize::max_value(), (usize::max_value(), usize::max_value()), (usize::max_value(), usize::max_value()));
}

fn smaller_path(p1: (usize, (usize, usize), (usize, usize)), p2: (usize, (usize, usize), (usize, usize))) -> (usize, (usize, usize), (usize, usize)) {
    let (steps1, (row1, col1), _) = p1;
    let (steps2, (row2, col2), _) = p2;

    if steps1 < steps2 { return p1; }
    if steps1 > steps2 { return p2; }
    if row1 < row2 { return p1; }
    if row1 > row2 { return p2; }
    if col1 < col2 { return p1; }
    if col1 > col2 { return p2; }

    return p1;
}

fn is_in_range(map: &mut Vec<Vec<char>>, creature: &Creature) -> bool {
    let enemy_race = if creature.race == 'E' { 'G' } else { 'E' };
    return map[creature.row - 1][creature.col] == enemy_race ||
        map[creature.row][creature.col - 1] == enemy_race ||
        map[creature.row][creature.col + 1] == enemy_race ||
        map[creature.row + 1][creature.col] == enemy_race;
}

fn move_creature(map: &mut Vec<Vec<char>>, creatures: &mut Vec<Creature>, idx: usize) {
    if is_in_range(map, &creatures[idx]) {
        return;
    }

    let mut range_fields: Vec<(usize, (usize, usize), (usize, usize))> = Vec::new();
    for enemy in creatures.iter().filter(|c| c.race != creatures[idx].race && c.hits > 0) {
        range_fields.push(num_steps(map, (creatures[idx].row, creatures[idx].col), (enemy.row - 1, enemy.col)));
        range_fields.push(num_steps(map, (creatures[idx].row, creatures[idx].col), (enemy.row, enemy.col - 1)));
        range_fields.push(num_steps(map, (creatures[idx].row, creatures[idx].col), (enemy.row, enemy.col + 1)));
        range_fields.push(num_steps(map, (creatures[idx].row, creatures[idx].col), (enemy.row + 1, enemy.col)));
    }

    let mut min_range = (usize::max_value(), (usize::max_value(), usize::max_value()), (usize::max_value(), usize::max_value()));
    for r in range_fields {
        min_range = smaller_path(min_range, r);
    }

    let (_, _, (first_step_row, first_step_col)) = min_range;
    if min_range.0 < usize::max_value() {
        map[creatures[idx].row][creatures[idx].col] = '.';
        map[first_step_row][first_step_col] = creatures[idx].race;
        creatures[idx].row = first_step_row;
        creatures[idx].col = first_step_col;
    }
}

fn is_battle_over(creatures: &Vec<Creature>) -> bool {
    let mut num_elves_alive = 0;
    let mut num_gnomes_alive = 0;
    for i in 0..creatures.len() {
        if creatures[i].hits > 0 {
            if creatures[i].race == 'E' {
                num_elves_alive += 1;
            } else {
                num_gnomes_alive += 1;
            }
        }
    }

    return num_elves_alive == 0 || num_gnomes_alive == 0;
}

fn turn(map: &mut Vec<Vec<char>>, creatures: &mut Vec<Creature>) -> bool {

    creatures.sort_by(|l, r| {
        if l.row < r.row || (l.row == r.row && l.col < r.col) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });


    for i in 0..creatures.len() {
        if creatures[i].hits <= 0 {
            continue;
        }

        if is_battle_over(&creatures) {
            return true;
        }

        if !try_attack(map, creatures, i) {
            move_creature(map, creatures, i);
            try_attack(map, creatures, i);
        }
    }

    println!();
    draw_map(&map);

    for c in creatures {
        println!("{:?}", c);
    }

    println!("==================================================================\n");
    return false;
}

fn draw_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn problem_1(input_path: String) {
    let mut map = lines_from_file(input_path);
    let mut creatures = get_creatures(&map, 3, 3);

    let mut turn_number = 0;
    while !is_battle_over(&creatures) {
        println!("Turn {}", turn_number);
        if !turn(&mut map, &mut creatures) {
            turn_number += 1;
        }
    }

    let hit_points_left:i32 =
        creatures
            .iter()
            .filter(|c| c.hits > 0)
            .map(|c| c.hits)
            .sum();

    draw_map(&map);
    println!("hit points: {}, turns: {}, result: {}", hit_points_left, turn_number, turn_number * hit_points_left);
}

fn problem_2(input_path: &String) {
    let mut elf_attack = 3;
    loop {
        let mut map = lines_from_file(input_path);
        let mut creatures = get_creatures(&map, elf_attack, 3);

        let mut turn_number = 0;
        while !is_battle_over(&creatures) {
            println!("Turn {}", turn_number);
            if !turn(&mut map, &mut creatures) {
                turn_number += 1;
            }
        }

        draw_map(&map);

        if !creatures.iter().any(|c| c.race == 'E' && c.hits <= 0) {
            let hit_points_left:i32 =
                creatures
                    .iter()
                    .filter(|c| c.hits > 0)
                    .map(|c| c.hits)
                    .sum();

            println!("hit points: {}, turns: {}, result: {}", hit_points_left, turn_number, turn_number * hit_points_left);
            return;
        }

        println!(":(");


        elf_attack += 1;
    }
}


fn main() {
    problem_1("input.txt".to_string());
    problem_2(&"input.txt".to_string());
}
