extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::cmp;

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

fn get_coordinates(lines: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"< *(-?\d+), +(-?\d+)>.*< *(-?\d+), +(-?\d+)>").unwrap();
    return lines.iter().map(|l| {
        let caps = re.captures(l).unwrap();
        return (
            (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap()),
            (caps.get(3).unwrap().as_str().parse().unwrap(), caps.get(4).unwrap().as_str().parse().unwrap())
        );
    }).collect();
}

fn find_min_time(coordinates: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut min_width = i32::max_value();
    let mut time = 1;
    loop {
        let mut min_x = i32::max_value();
        let mut max_x = i32::min_value();

        for ((pos_x, _pos_y), (vel_x, _vel_y)) in coordinates {
            min_x = cmp::min(min_x, pos_x + time * vel_x);
            max_x = cmp::max(max_x, pos_x + time * vel_x);
        }

        let width = max_x - min_x;
        if width > min_width {
            return time - 1;
        }

        min_width = width;

        time += 1;
    }
}

fn problem(coordinates: &Vec<((i32, i32), (i32, i32))>) {

    let time = find_min_time(&coordinates);

    let positions: Vec<(i32, i32)> = coordinates.iter().map(|c| {
        let ((pos_x, pos_y), (vel_x, vel_y)) = c;
        return (pos_x + time * vel_x, pos_y + time * vel_y);
    }).collect();

    let mut min_x = i32::max_value(); let mut min_y = i32::max_value();
    let mut max_x = i32::min_value(); let mut max_y = i32::min_value();
    for (x, y) in &positions {
        min_x = cmp::min(min_x, *x);
        min_y = cmp::min(min_y, *y);
        max_x = cmp::max(max_x, *x);
        max_y = cmp::max(max_y, *y);
    }

    let mut message = vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for (x, y) in &positions {
        message[(y - min_y) as usize][(x - min_x) as usize] = '#';
    }

    for message_line in message {
        println!("{:?}", message_line);
    }

    println!("{}", time);
}

fn main() {
    let lines = lines_from_file("input.txt");
    let coordinates = get_coordinates(lines);
    problem(&coordinates);
}
