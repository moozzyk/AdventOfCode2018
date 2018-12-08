use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
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

fn to_coordinate(s: &String) -> (i32, i32) {
    let v = s.split(", ").collect::<Vec<&str>>();
    return (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap());
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    let (p1_x, p1_y) = p1;
    let (p2_x, p2_y) = p2;
    return ((p1_x - p2_x).abs() + (p1_y - p2_y).abs()) as u32;
}

fn problem1(coordinates: &Vec<(i32, i32)>) {
    let mut field = [[0; 500]; 500];
    for x in 0..500 {
        for y in 0..500 {
            let mut min_dist = u32::max_value();
            let mut c1: i32 = -1;
            let mut c2: i32 = -1;
            for i in 0..coordinates.len() {
                let dist = distance((x, y), coordinates[i]);
                if dist < min_dist {
                    min_dist = dist;
                    c1 = i as i32;
                    c2 = -1;
                } else if dist == min_dist {
                    c2 = i as i32;
                }
            }

            if c2 != -1 {
                field[x as usize][y as usize] = -1;
            } else {
                field[x as usize][y as usize] = c1;
            }
        }
    }

    let mut counts = HashMap::new();
    let mut invalid = HashSet::new();
    for x in 0..500 {
        for y in 0..500 {
            if x == 0 || y == 0 || x == 499 || y == 499 {
                invalid.insert(field[x][y]);
            } else {
                if !counts.contains_key(&field[x][y]) {
                    counts.insert(field[x][y], 0);
                }
                *counts.get_mut(&field[x as usize][y as usize]).unwrap() += 1;
            }
        }
    }

    let mut max_size = -1;
    for (item, size) in counts.iter() {
        if !invalid.contains(item) && size > &max_size {
            max_size = *size;
        }
    }

    println!("{}", max_size);
}

fn problem2(coordinates: &Vec<(i32, i32)>) {
    let mut result = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let mut total_dist = 0;
            for i in 0..coordinates.len() {
                total_dist += distance((x - 500, y - 500), coordinates[i]);
            }
            if total_dist < 10000 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}

fn main() {
    let coordinates = lines_from_file("input.txt")
        .iter()
        .map(|s| to_coordinate(s))
        .collect::<Vec<(i32, i32)>>();

    problem1(&coordinates);
    problem2(&coordinates);
}
