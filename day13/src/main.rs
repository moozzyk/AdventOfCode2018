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
    crashed: bool,
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

        for j in 0..carts.len() {
            if j != i && !carts[j].crashed && carts[i].row == carts[j].row && carts[i].col == carts[j].col {
                carts[i].crashed = true;
                carts[j].crashed = true;
            }
        }
    }
}

fn get_cart_list(map: &mut Vec<Vec<char>>) -> Vec<Cart> {
    let mut carts = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                '>' => {
                    map[row][col] = '-';
                    carts.push(Cart{row: row, col: col, direction: RIGHT, turn: TURN_LEFT, crashed: false});
                },
                '<' => {
                    map[row][col] = '-';
                    carts.push(Cart{row: row, col: col, direction: LEFT, turn: TURN_LEFT, crashed: false});
                },
                '^' => {
                    map[row][col] = '|';
                    carts.push(Cart{row: row, col: col, direction: UP, turn: TURN_LEFT, crashed: false});
                },
                'v' => {
                    map[row][col] = '|';
                    carts.push(Cart{row: row, col: col, direction: DOWN, turn: TURN_LEFT, crashed: false});
                },
                _ => ()
            }
        }
    }

    return carts;

}

fn problem_1(mut map: Vec<Vec<char>>) {
    let mut carts = get_cart_list(&mut map);
    loop {
        carts.sort();
        if carts.iter().any(|c| c.crashed) {
            break;
        }
        tick(&map, &mut carts);
    }

    for c in carts.iter().filter(|c| c.crashed) {
        println!("{}, {}", c.col, c.row);
        break;
    }
}

fn problem_2(mut map: Vec<Vec<char>>) {
    let mut carts = get_cart_list(&mut map);

    while carts.len() > 1 {
        carts.sort();
        tick(&map, &mut carts);
        let mut i:i32 = (carts.len() - 1) as i32;
        while i >= 0 {
            if carts[i as usize].crashed {
                carts.remove(i as usize);
            }
            i -= 1;
        }
    }

    println!("{}, {}", carts[0].col, carts[0].row);
}

fn main() {
    let map = lines_from_file("input.txt");
    problem_1(map.clone());
    problem_2(map.clone());
}
