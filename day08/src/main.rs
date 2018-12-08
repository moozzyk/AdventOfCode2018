use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn read_first_line<P>(filename: P) -> String where
    P: AsRef<Path> {
    let file = File::open(filename).expect("no such file");
    let mut buf = BufReader::new(file);

    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    return line;
}

fn metadata_sum(input: &Vec<u32>, mut index: usize) -> (u32, usize) {
    let num_nodes = input[index];
    let num_metadata = input[index + 1] as usize;
    index += 2;

    let mut sum: u32 = 0;
    for _ in 0..num_nodes {
        let (child_sum, new_index) = metadata_sum(&input, index);
        sum += child_sum;
        index = new_index;
    }

    let s: u32 = input[index..index + num_metadata].iter().sum();
    sum += s;
    index += num_metadata;
    return (sum, index);
}

fn problem1(input: &str) {
    let tree = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let (metadata_sum, _) = metadata_sum(&tree, 0);

    println!("{}", metadata_sum);
}

fn main() {
    let input = read_first_line("input.txt");
    problem1(&input);
}
