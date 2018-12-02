use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    problem1()
}

fn problem1() {
    let file = File::open("input.txt").unwrap();

    let mut result = 0;
    for line in BufReader::new(file).lines() {
        result += line.unwrap().parse().unwrap()
    }

    println!("{}", result)
}
