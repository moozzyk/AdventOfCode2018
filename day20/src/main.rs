use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::VecDeque;

const SIZE: usize = 16000;

fn read_first_line<P>(filename: P) -> String where
    P: AsRef<Path> {
    let file = File::open(filename).expect("no such file");
    let mut buf = BufReader::new(file);

    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    return line;
}

#[allow(dead_code)]
fn print_map(map: &Vec<[char; SIZE]>) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            print!("{}", map[row][col]);
        }
        println!();
    }
}

fn create_map() -> Vec<[char; SIZE]> {
    let mut map = vec![];
    for row in 0..SIZE {
        map.push([if row % 2 == 0 {'#'} else {'.'} ; SIZE]);
        let mut i = row % 2;
        while i < map[row].len() {
            map[row][i] = '?';
            i += 2;
        }
    }
    return map;
}

fn build_graph(position: (usize, usize), map: &mut Vec<[char; SIZE]>, pattern: &Vec<char>, idx: usize) -> usize {
    let (mut row, mut col) = position;

    let mut i = idx;
    while i < pattern.len() {
        match pattern[i] {
            'E' => {
                map[row][col + 1] = ' '; map[row][col + 2] = ' ';
                col += 2;
            },
            'W' => {
                map[row][col - 1] = ' '; map[row][col - 2] = ' ';
                col -= 2;
            },
            'N' => {
                map[row - 1][col] = ' ';
                map[row - 2][col] = ' ';
                row -= 2;
            },
            'S' => {
                map[row + 1][col] = ' ';
                map[row + 2][col] = ' ';
                row += 2;
            },
            ')' => return i,
            '(' => i = build_graph((row, col), map, pattern, i + 1),
            '|' => {
                row = position.0;
                col = position.1;
            },
            '^' => (),
            '$' => (),
            _ => panic!("unexpected: {} {}", i, pattern[i]),
        }

        i += 1;
    }

    return i;
}

fn fill_walls(map: &mut Vec<[char; SIZE]>) {
    for row in map {
        for col in 0..row.len() {
            if row[col] == '?' {
                row[col] = '#';
            }
        }
    }
}

fn bfs(start_pos: (usize, usize), map: &mut Vec<[char; SIZE]>) -> (usize, usize) {
    let mut max_steps = 0;
    let mut dist_1000 = 0;
    let mut q = VecDeque::new();
    q.push_back(((start_pos), 0));

    while !q.is_empty() {
        let (position, steps) = q.pop_front().unwrap();
        let (row, col) = position;
        if map[row][col] != ' ' {
            continue;
        }
        map[row][col] = 'x';

        max_steps = std::cmp::max(max_steps, steps);
        if steps >= 1000 {
            dist_1000 += 1;
        }
        if map[row + 1][col] == ' ' { q.push_back(((row + 2, col), steps + 1)); }
        if map[row - 1][col] == ' ' { q.push_back(((row - 2, col), steps + 1)); }
        if map[row][col + 1] == ' ' { q.push_back(((row, col + 2), steps + 1)); }
        if map[row][col - 1] == ' ' { q.push_back(((row, col - 2), steps + 1)); }
    }

    return (max_steps, dist_1000);
}

fn problem_1_2() {
    let mut input = read_first_line("input.txt");
    input.pop();
    let mut map = create_map();
    let start_pos = (SIZE / 2 - 1, SIZE / 2);
    map[start_pos.0][start_pos.1] = ' ';
    build_graph((SIZE / 2 - 1, SIZE / 2), &mut map, &input.chars().collect(), 0);
    fill_walls(&mut map);
    let (max_steps, dist_1000) = bfs(start_pos, &mut map);
    println!("Steps: {}, Num rooms: {}", max_steps, dist_1000);
}

fn main() {
    problem_1_2();
}
