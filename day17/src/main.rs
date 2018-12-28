extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;

const SIZE: usize = 2000;

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

fn create_map(input: &Vec<String>) -> Vec<[char; SIZE]> {
    let mut geo_map = vec![];
    for _i in 0..SIZE {
        geo_map.push(['.'; SIZE]);
    }

    let re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)").unwrap();
    for line in input {
        let caps = re.captures(line).unwrap();
        let direction = caps.get(1).map(|c| c.as_str()).unwrap();
        let axis: usize = caps.get(2).map(|c| c.as_str().parse().unwrap()).unwrap();
        let from: usize = caps.get(4).map(|c| c.as_str().parse().unwrap()).unwrap();
        let to: usize = caps.get(5).map(|c| c.as_str().parse().unwrap()).unwrap();

        for i in from..=to {
            if direction == "x" {
                geo_map[i][axis] = '#';
            } else {
                geo_map[axis][i] = '#';
            }
        }
    }

    return geo_map;
}

#[allow(dead_code)]
fn print_geo_map(geo_map: &Vec<[char; SIZE]>) {
    for row in 0..geo_map.len() {
        for col in 0..geo_map[row].len() {
            print!("{}", geo_map[row][col]);
        }
        println!();
    }
    println!();
}

fn add_to_sources(position: (usize, usize), sources: &mut VecDeque<(usize, usize)>, visited: &mut HashSet<(usize, usize)>) {
    if !visited.contains(&position) {
        visited.insert(position);
        sources.push_back(position);
    }
}

fn has_wall(start_from: (usize, usize), next: fn(usize) -> usize, geo_map: &Vec<[char; SIZE]>) -> bool {
    let (row, mut col) = start_from;
    loop {
        if geo_map[row][col] == '#' {
            return true;
        }

        if geo_map[row + 1][col] != '#' {
            return false;
        }

        col = next(col);
    }
}

fn has_walls(start_from: (usize, usize), geo_map: &Vec<[char; SIZE]>) -> bool {
    return has_wall(start_from, |col| col - 1, geo_map) && has_wall(start_from, |col| col + 1, geo_map);
}

fn fill_layer(start_from: (usize, usize), c: char, next: fn(usize) -> usize, geo_map: &mut Vec<[char; SIZE]>, sources: &mut VecDeque<(usize, usize)>, visited: &mut HashSet<(usize, usize)>) -> bool {
    let (row, mut col) = start_from;
    loop {
        if geo_map[row][col] == '#' {
            return false;
        }

        geo_map[row][col] = c;

        if geo_map[row + 1][col] == '#' && geo_map[row + 1][col - 1] != '#' && geo_map[row + 1][col + 1] != '#' {
            add_to_sources((row, next(col)), sources, visited);

            let mut tmp_row = row + 1;
            while geo_map[tmp_row][col] == '#' {
                tmp_row += 1;
            }

            if geo_map[tmp_row][col] == '.' {
                return true;
            }
        } else if geo_map[row + 1][col] == '#' && geo_map[row + 1][col - 1] != '#' {
            add_to_sources((row, col - 1), sources, visited);
        } if geo_map[row + 1][col] == '#' && geo_map[row + 1][col - 1] != '#' {
            add_to_sources((row, col + 1), sources, visited);
        }

        col = next(col);
    }
}

fn fill_from_bottom(start_from: (usize, usize), geo_map: &mut Vec<[char; SIZE]>, sources: &mut VecDeque<(usize, usize)>, visited: &mut HashSet<(usize, usize)>) {
    let (mut row, col) = start_from;

    loop {
        let left_full = fill_layer((row, col), '%', |col| col - 1, geo_map, sources, visited);
        let right_full = fill_layer((row, col), '%',  |col| col + 1, geo_map, sources, visited);
        if left_full || right_full {
            fill_layer((row, col), '|', |col| col - 1, geo_map, sources, visited);
            fill_layer((row, col), '|',  |col| col + 1, geo_map, sources, visited);
            break;
        }
        row -= 1;
    }
}

fn spill_to_side(start_from: (usize, usize), next: fn(usize) -> usize, geo_map: &mut Vec<[char; SIZE]>, sources: &mut VecDeque<(usize, usize)>, visited: &mut HashSet<(usize, usize)>) {
    let (row, mut col) = start_from;
    loop {
        if geo_map[row + 1][col] == '#' {
            geo_map[row][col] = '%';
        }
        col = next(col);

        if geo_map[row + 1][col] != '#' {
            add_to_sources((row, col), sources, visited);
            return;
        }
    }
}

fn spill(start_from: (usize, usize), geo_map: &mut Vec<[char; SIZE]>, sources: &mut VecDeque<(usize, usize)>, visited: &mut HashSet<(usize, usize)>) {
    spill_to_side(start_from, |col| col - 1, geo_map, sources, visited);
    spill_to_side(start_from, |col| col + 1, geo_map, sources, visited);
}

fn pour(geo_map: &mut Vec<[char; SIZE]>, sources: &mut VecDeque<(usize, usize)>, visited: &mut HashSet<(usize, usize)>) {
    let (mut row, col) = sources.pop_front().unwrap();
    loop {
        if row == SIZE - 1 {
            return;
        }

        if geo_map[row][col - 1] == '%' || geo_map[row][col + 1] == '%' {
            geo_map[row][col] = '%';
        } else {
            geo_map[row][col] = '|';
        }

        if geo_map[row + 1][col] == '#' {
            break;
        }

        row += 1;
    }

    if row == SIZE {
        return;
    }

    if has_walls((row, col), geo_map) {
        fill_from_bottom((row, col), geo_map, sources, visited);
    } else {
        spill((row, col), geo_map, sources, visited);
    }
}

fn get_min_max_row(geo_map: &Vec<[char; SIZE]>) -> (usize, usize) {
    let mut min_row = usize::max_value();
    let mut max_row = usize::min_value();

    for row in 0..geo_map.len() {
        for col in 0..SIZE {
            if geo_map[row][col] == '#' {
                max_row = max(row, max_row);
                min_row = min(row, min_row);
                break;
            }
        }
    }

    return (min_row, max_row);
}

fn fill(geo_map: &mut Vec<[char; SIZE]>) {
    let mut visited = HashSet::new();
    let mut sources = VecDeque::new();
    add_to_sources((0, 500), &mut sources, &mut visited);

    while !sources.is_empty() {
        pour(geo_map, &mut sources, &mut visited);
    }
}

fn count_water(geo_map: &Vec<[char; SIZE]>, min_row: usize, max_row: usize, chars: &Vec<char>) -> usize {
    let mut num_water = 0;
    for row in min_row..=max_row {
        for col in 0..SIZE {
            for c in chars {
                if geo_map[row][col] == *c {
                    num_water += 1;
                }
            }
        }
    }

    return num_water;
}

fn problem_1(geo_map: &Vec<[char; SIZE]>, min_row: usize, max_row: usize) -> usize {
    return count_water(geo_map, min_row, max_row, &vec!['%', '|']);
}

fn problem_2(geo_map: &Vec<[char; SIZE]>, min_row: usize, max_row: usize) -> usize {
    return count_water(geo_map, min_row, max_row, &vec!['%']);
}

fn main() {
    let input = lines_from_file("input.txt");

    let mut geo_map = create_map(&input);
    let (min_row, max_row) = get_min_max_row(&geo_map);

    fill(&mut geo_map);

    println!("{}", problem_1(&mut geo_map, min_row, max_row));
    println!("{}", problem_2(&mut geo_map, min_row, max_row));
}
