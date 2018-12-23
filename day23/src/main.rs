extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use regex::Regex;

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

fn get_nanobots_data(lines: Vec<String>) -> Vec<((i64, i64, i64), i64)> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
    return lines.iter().map(|l| {
        let caps = re.captures(l).unwrap();
        return ((caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap()), caps[4].parse().unwrap());
    }).collect();
}

fn in_range(bot: &((i64, i64, i64), i64), strongest: &((i64, i64, i64), i64)) -> bool {
    let ((x1, y1, z1), r) = strongest;
    let ((x2, y2, z2), _) = bot;

    let dist = (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs();

    return dist <= *r;
}

fn problem_1() {
    let lines = lines_from_file("input.txt");
    let nanobots_data = get_nanobots_data(lines);

    let strongest = nanobots_data.iter().max_by_key(|n| n.1).unwrap();
    let num_in_range = nanobots_data.iter().fold(0, |acc, bot| acc + if in_range(bot, strongest) {1} else {0});

    println!("{}", num_in_range);
}

fn main() {
    problem_1();
}
