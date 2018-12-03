use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

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

fn has_freq(s: &String, freq: u32) -> u32 {
    let mut frequencies = HashMap::new();
    for c in s.chars() {
        if !frequencies.contains_key(&c) {
            frequencies.insert(c, 1);
        } else {
            *frequencies.get_mut(&c).unwrap() += 1;
        }
    }

    for f in frequencies.values() {
        if *f == freq {
            return 1;
        }
    }

    return 0
}

fn problem1() {
    let lines = lines_from_file("input.txt");
    let num_twos: u32 =
        lines.iter()
            .map(|s| has_freq(s, 3))
            .sum();

    let num_threes: u32 =
        lines.iter()
            .map(|s| has_freq(s, 2))
            .sum();

    println!("{}", num_twos * num_threes);
}

fn main() {
    problem1();
}
