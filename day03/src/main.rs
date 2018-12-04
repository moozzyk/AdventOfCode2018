extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn problem1() {
    let lines = lines_from_file("input.txt");

    let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut fabric = [[0; 1000]; 1000];

    for line in lines.iter() {
        let caps = re.captures(line).unwrap();
        let start_row: usize = caps.get(1).map(|c| c.as_str().parse().unwrap()).unwrap();
        let start_col: usize = caps.get(2).map(|c| c.as_str().parse().unwrap()).unwrap();
        let width: usize = caps.get(3).map(|c| c.as_str().parse().unwrap()).unwrap();
        let height: usize = caps.get(4).map(|c| c.as_str().parse().unwrap()).unwrap();
        for row in start_row..start_row + width {
            for col in start_col..start_col + height {
                fabric[row][col] += 1;
            }
        }
    }

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if fabric[x][y] > 1 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn problem2() {
    let lines = lines_from_file("input.txt");

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut fabric = [[0; 1000]; 1000];
    let mut ids: HashSet<i32> = HashSet::new();

    for line in lines.iter() {
        let caps = re.captures(line).unwrap();
        let id: i32 = caps.get(1).map(|c| c.as_str().parse().unwrap()).unwrap();
        let start_row: usize = caps.get(2).map(|c| c.as_str().parse().unwrap()).unwrap();
        let start_col: usize = caps.get(3).map(|c| c.as_str().parse().unwrap()).unwrap();
        let width: usize = caps.get(4).map(|c| c.as_str().parse().unwrap()).unwrap();
        let height: usize = caps.get(5).map(|c| c.as_str().parse().unwrap()).unwrap();

        ids.insert(id);
        for row in start_row..start_row + width {
            for col in start_col..start_col + height {
                if fabric[row][col] != 0 {
                    ids.remove(&id);
                    ids.remove(&fabric[row][col]);
                }
                fabric[row][col] = id;
            }
        }
    }

    println!("{:?}", ids);
}

fn main() {
    problem1();
    problem2();
}
