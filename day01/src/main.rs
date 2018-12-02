use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    problem1();
    problem2()
}

fn problem1() {
    let mut result = 0;
    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        result += line.unwrap().parse().unwrap()
    }

    println!("{}", result)
}

fn problem2() {
    let mut vec: Vec<i32> = Vec::new();
    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap().parse().unwrap())
    }

    let mut seen: HashSet<i32> = HashSet::new();
    let mut index = 0;
    let mut frequency = 0;
    loop {
        frequency += vec[index % vec.len()];
        if seen.contains(&frequency) {
            println!("{}", frequency);
            break
        }
        seen.insert(frequency);
        index += 1
    }
}
