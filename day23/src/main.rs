extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::collections::BTreeMap;

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

#[derive(Debug)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
    size: i64
}

fn get_initial_cube_size(bots: &Vec<((i64, i64, i64), i64)>) -> i64 {
    let mut max_coordinate = bots.iter().fold(0, |max_coordinate, bot| max(max((bot.0).0.abs(), (bot.0).1.abs()), max((bot.0).2.abs(), max_coordinate)));

    let mut n = 0;
    while max_coordinate > 0 {
        n += 1;
        max_coordinate >>= 1;
    }

    return 1 << n;
}

fn in_cube_range(cube: &Cube, bot: &((i64, i64, i64), i64)) -> bool {
    fn coordinate_ditance(c: i64, from: i64, to: i64) -> i64 {

        if c >= from && c <= to {
            return 0;
        }

        return min((c - from).abs(), (c - to).abs());
    }

    let ((x, y, z), r) = bot;
    let dist =
        coordinate_ditance(*x, cube.x, cube.x + cube.size) +
        coordinate_ditance(*y, cube.y, cube.y + cube.size) +
        coordinate_ditance(*z, cube.z, cube.z + cube.size);

    return dist <= *r;
}

fn get_num_bots_in_cube(cube: &Cube, bots: &Vec<((i64, i64, i64), i64)>) -> i64 {
    bots.iter().fold(0, |num_bots, bot| if in_cube_range(cube, bot) { num_bots + 1 } else { num_bots})
}

fn enqueue(queue: &mut BTreeMap<i64, VecDeque<Cube>>, num_bots: i64, cube: Cube) {
    if !queue.contains_key(&num_bots) {
        let mut v = VecDeque::new();
        v.push_back(cube);
        queue.insert(num_bots, v);
    } else {
        queue.get_mut(&num_bots).unwrap().push_back(cube);
    }
}

fn dequeue(queue: &mut BTreeMap<i64, VecDeque<Cube>>) -> (i64, Cube) {
    // there must be a way to make this simpler...
    fn highest_key(queue: &mut BTreeMap<i64, VecDeque<Cube>>) -> i64 {
        let (key, _) = queue.iter().next_back().unwrap();
        return *key;
    }

    let key = highest_key(queue);
    let cube = queue.get_mut(&key).unwrap().pop_front().unwrap();
    if queue[&key].is_empty() {
        queue.remove(&key);
    }

    return (key, cube);
}

fn enqueue_cube(x: i64, y: i64, z: i64, size: i64, bots: &Vec<((i64, i64, i64), i64)>, queue: &mut BTreeMap<i64, VecDeque<Cube>>) {
    let cube = Cube{x: x, y: y, z: z, size: size};
    let num_bots = get_num_bots_in_cube(&cube, bots);
    if num_bots > 0 {
        enqueue(queue, num_bots, cube);
    }
}

fn problem_2() {
    let lines = lines_from_file("input.txt");
    let nanobots_data = get_nanobots_data(lines);

    let cube_size = get_initial_cube_size(&nanobots_data);
    let cube = Cube{x: -cube_size, y: -cube_size, z: -cube_size, size: cube_size * 2};
    let mut queue = BTreeMap::new();
    enqueue_cube(-cube_size, -cube_size, -cube_size, cube_size * 2, &nanobots_data, &mut queue);

    let mut best = 0;
    let mut best_coordinate = (0i64, 0i64, 0i64);
    while !queue.is_empty() {
        let (num_bots, cube) = dequeue(&mut queue);
        if cube.size == 0 {
            if best < num_bots {
                best = num_bots;
                best_coordinate = (cube.x, cube.y, cube.z);
            }
            continue;
        }
        if num_bots < best {
            continue;
        }

        let new_size = cube.size / 2;

        enqueue_cube(cube.x, cube.y, cube.z, new_size, &nanobots_data, &mut queue);
        enqueue_cube(cube.x, cube.y, cube.z + new_size, new_size, &nanobots_data, &mut queue);
        enqueue_cube(cube.x, cube.y + new_size, cube.z, new_size, &nanobots_data, &mut queue);
        enqueue_cube(cube.x, cube.y + new_size, cube.z + new_size, new_size, &nanobots_data, &mut queue);
        enqueue_cube(cube.x + new_size, cube.y, cube.z, new_size, &nanobots_data, &mut queue);
        enqueue_cube(cube.x + new_size, cube.y, cube.z + new_size, new_size, &nanobots_data, &mut queue);
        enqueue_cube(cube.x + new_size, cube.y + new_size, cube.z, new_size, &nanobots_data, &mut queue);
        enqueue_cube(cube.x + new_size, cube.y + new_size, cube.z + new_size, new_size, &nanobots_data, &mut queue);
    }

    println!("{}", best_coordinate.0.abs() + best_coordinate.1.abs() + best_coordinate.2.abs());
}

fn main() {
   problem_1();
   problem_2();
}
