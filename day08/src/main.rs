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

fn problem1(tree: &Vec<u32>) -> u32 {
    let (metadata_sum, _) = metadata_sum(&tree, 0);
    return metadata_sum;
}

fn metadata_sum_2(tree: &Vec<u32>, mut index: usize) -> (u32, usize) {
    let num_nodes = tree[index];
    let num_metadata = tree[index + 1] as usize;
    index += 2;

    if num_nodes == 0 {
        let sum = tree[index..index + num_metadata].iter().sum();
        index += num_metadata;
        return (sum, index);
    }

    let mut node_sum = Vec::new();
    for _i in 0..num_nodes {
        let (sum, new_index) = metadata_sum_2(&tree, index);
        node_sum.push(sum);
        index = new_index;
    }

    let mut sum = 0;
    for i in index..index + num_metadata {
        sum += node_sum.get((tree[i] - 1) as usize).unwrap_or(&0);
    }

    index += num_metadata;
    return (sum, index);
}

fn problem2(tree: &Vec<u32>) -> u32 {
    let (result, _) = metadata_sum_2(tree, 0);
    return result;
}

fn main() {
    let input = read_first_line("input.txt")
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("{}", problem1(&input));
    println!("{}", problem2(&input));
}
