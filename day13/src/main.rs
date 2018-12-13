use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

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

const UP: u32  = 0;
const DOWN: u32 = 1;
const LEFT: u32 = 2;
const RIGHT: u32 = 3;

const TURN_LEFT :u32 = 0;
// const STRAIGHT: u32 = 1;
const TURN_RIGHT :u32 = 2;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
struct Cart {
    row: usize,
    col: usize,
    direction: u32,
    turn: u32,
}

fn move_up(map: &Vec<Vec<char>>, cart: &mut Cart) {
    cart.row -= 1;
    match map[cart.row][cart.col] {
        '/' => cart.direction = RIGHT,
        '\\' => cart.direction = LEFT,
        '+' => {
            if cart.turn == TURN_LEFT {
                cart.direction = LEFT;
            } else if cart.turn == TURN_RIGHT {
                cart.direction = RIGHT;
            }
            cart.turn = (cart.turn + 1) % 3;
        },
        '|' => (),
        _ => panic!("Unexpected tile: {}", map[cart.row][cart.col]),
    }
}

fn move_down(map: &Vec<Vec<char>>, cart: &mut Cart) {
    cart.row += 1;
    match map[cart.row][cart.col] {
        '/' => cart.direction = LEFT,
        '\\' => cart.direction = RIGHT,
        '+' => {
            if cart.turn == TURN_LEFT {
                cart.direction = RIGHT;
            } else if cart.turn == TURN_RIGHT {
                cart.direction = LEFT;
            }
            cart.turn = (cart.turn + 1) % 3;
        },
        '|' => (),
        _ => panic!("Unexpected tile: {}", map[cart.row][cart.col]),
    }
}

fn move_left(map: &Vec<Vec<char>>, cart: &mut Cart) {
    cart.col -= 1;
    match map[cart.row][cart.col] {
        '/' => cart.direction = DOWN,
        '\\' => cart.direction = UP,
        '+' => {
            if cart.turn == TURN_LEFT {
                cart.direction = DOWN;
            } else if cart.turn == TURN_RIGHT {
                cart.direction = UP;
            }
            cart.turn = (cart.turn + 1) % 3;
        },
        '-' => (),
        _ => panic!("Unexpected tile: {}", map[cart.row][cart.col]),
    }
}

fn move_right(map: &Vec<Vec<char>>, cart: &mut Cart) {
    cart.col += 1;
    match map[cart.row][cart.col] {
        '/' => cart.direction = UP,
        '\\' => cart.direction = DOWN,
        '+' => {
            if cart.turn == TURN_LEFT {
                cart.direction = UP;
            } else if cart.turn == TURN_RIGHT {
                cart.direction = DOWN;
            }
            cart.turn = (cart.turn + 1) % 3;
        },
        '-' => (),
        _ => panic!("Unexpected tile: {}", map[cart.row][cart.col]),
    }
}

fn tick(map: &Vec<Vec<char>>, carts: &mut Vec<Cart>) {
    for i in 0..carts.len() {
        match carts[i].direction {
            UP => move_up(&map, &mut carts[i]),
            DOWN => move_down(&map, &mut carts[i]),
            LEFT => move_left(&map, &mut carts[i]),
            RIGHT => move_right(&map, &mut carts[i]),
            _ => panic!("Invalid direction"),
        }
    }
}

fn problem_1(map: &mut Vec<Vec<char>>) {
    let mut carts = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                '>' => {
                    map[row][col] = '-';
                    carts.push(Cart{row: row, col: col, direction: RIGHT, turn: TURN_LEFT});
                },
                '<' => {
                    map[row][col] = '-';
                    carts.push(Cart{row: row, col: col, direction: LEFT, turn: TURN_LEFT});
                },
                '^' => {
                    map[row][col] = '|';
                    carts.push(Cart{row: row, col: col, direction: UP, turn: TURN_LEFT});
                },
                'v' => {
                    map[row][col] = '|';
                    carts.push(Cart{row: row, col: col, direction: DOWN, turn: TURN_LEFT});
                },
                _ => ()
            }
        }
    }

    loop {
        carts.sort();
        for i in 1..carts.len() {
            if carts[i].row == carts[i - 1].row && carts[i].col == carts[i - 1].col {
                println!("x: {}, y:{}", carts[i].col, carts[i].row);
                return;
            }
        }
        tick(map, &mut carts);
        // println!("{:?}", &carts);

    }
}

fn main() {
    let mut map = lines_from_file("input.txt");
    problem_1(&mut map);
}
