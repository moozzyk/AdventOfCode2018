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

fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn neighboring_resources(row: usize, col: usize, map: &Vec<Vec<char>>) -> (usize, usize, usize) {
    let mut open = 0;
    let mut lumber = 0;
    let mut trees = 0;

    for row_i in if row == 0 {0} else {row-1}..=if row < map.len() - 1 { row + 1 } else { map.len() - 1 } {
        for col_i in if col == 0 {0} else {col - 1}..=if col < map[row].len() - 1 {col + 1} else {map[row].len() - 1} {
            if row_i != row || col_i != col {
                match map[row_i][col_i] {
                    '.' => open += 1,
                    '|' => trees += 1,
                    '#' => lumber += 1,
                    _ => panic!("Invalid input"),
                }
            }
        }
    }
    return (open, lumber, trees);
}

fn next_field_value(row: usize, col: usize, map: &Vec<Vec<char>>) -> char {
    let (_open, lumber, trees) = neighboring_resources(row, col, map);
    match map[row][col] {
        '.' => if trees >= 3 {'|'} else {'.'},
        '|' => if lumber >= 3 {'#'} else {'|'},
        '#' => if lumber >= 1 && trees >= 1 {'#'} else {'.'},
        _ => panic!("Invalid input"),
    }
}

fn minute(map: &mut Vec<Vec<char>>) {
    let tmp = map.clone();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            map[row][col] = next_field_value(row, col, &tmp);
        }
    }
}

fn get_resource_value(map: &Vec<Vec<char>>) -> u32 {
    let mut trees = 0;
    let mut lumber = 0;
    for row in map.iter() {
        for c in row.iter() {
           match c {
               '|' => trees += 1,
               '#' => lumber += 1,
               _ => (),
           }
        }
    }

    return trees * lumber;
}

fn problem1() {
    let mut map = lines_from_file("input.txt");
    // print_map(&map);
    for _i in 0..10 {
        minute(&mut map);
        // println!();
        // print_map(&map);
    }

    println!("{}", get_resource_value(&map));
}

fn problem2() {
    let mut map = lines_from_file("input.txt");
    let mut values = vec![];

    for _i in 0..1000 {
        minute(&mut map);
        let resource_value = get_resource_value(&map);
        values.push(resource_value);
        // println!("{}", resource_value);
    }

    let mut cycle = 0;
    for i in (0..values.len() - 2).rev() {
        if values[i] == *values.last().unwrap() {
            cycle = values.len() - 1 - i;
            break;
        }
    }

    println!("{}", values[1000 - 1 - cycle  + (1000000000 - 1000) % cycle]);
}

fn main() {
    problem1();
    problem2();
}
