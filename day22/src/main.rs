use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::cmp::min;

const ROCKY: usize = 0;
const WET: usize = 1;
const NARROW: usize = 2;

fn get_cell_display_type(cell_type: usize) -> char {
    match cell_type {
        ROCKY => '.',
        WET => '=',
        NARROW => '|',
        _ => panic!("Invalid erosion value"),
    }
}

#[allow(dead_code)]
fn print_cave(cave: &Vec<Vec<usize>>) {
    for r in cave {
        for f in r {
            print!("{}", get_cell_display_type(f % 3));
        }
        println!();
    }
}

fn create_erosion_map(depth: usize, height: usize, width: usize, target: (usize, usize)) -> Vec<Vec<usize>> {
    fn calculate_erosion(geological_index: usize, depth: usize) -> usize { ((geological_index + depth) % 20183) };

    let mut erosion_map = vec![vec![0; width]; height];
    let (target_x, target_y) = target;

    for y in 0..height {
        for x in 0..width {
            if (x == 0 && y == 0) || (x == target_x && y == target_y) {
                erosion_map[y][x] = calculate_erosion(0, depth);
            } else if x == 0 {
                erosion_map[y][x] = calculate_erosion(y * 48271, depth);
            } else if y == 0 {
                erosion_map[y][x] = calculate_erosion(x * 16807, depth);
            } else {
                erosion_map[y][x] = calculate_erosion(erosion_map[y - 1][x] * erosion_map[y][x - 1], depth);
            }
        }
    }

    return erosion_map;
}

fn problem_1(depth: usize, target: (usize, usize)) -> usize {
    let (target_x, target_y) = target;

    let erosion_map = create_erosion_map(depth, target_y + 1, target_x + 1, target);

    let mut sum: usize = 0;
    for r in erosion_map {
        for x in r {
            sum += x % 3;
        }
    }

    return sum;
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum Equipment {
    Neither = 0,
    Torch,
    ClimbingGear,
}

fn can_use_gear(x: usize, y: usize, erosion_map: &Vec<Vec<usize>>, equipment: Equipment) -> bool {
    match erosion_map[y][x] % 3 {
        ROCKY => return equipment == Equipment::Torch || equipment == Equipment::ClimbingGear,
        WET => return equipment == Equipment::Neither || equipment == Equipment::ClimbingGear,
        NARROW => return equipment == Equipment::Neither || equipment == Equipment::Torch,
        _ => panic!("Can't get here...")
    }
}

fn add_to_queue(queue: &mut BTreeMap<usize, VecDeque<(usize, usize, Equipment, usize)>>, item: (usize, usize, Equipment, usize)) {
    let key = item.3;
    if !queue.contains_key(&key) {
        let mut q = VecDeque::new();
        q.push_back(item);
        queue.insert(key, q);
    } else {
        queue.get_mut(&key).unwrap().push_back(item);
    }
}

fn problem_2(depth: usize, target: (usize, usize)) -> usize {
    let (target_x, target_y) = target;

    let erosion_map = create_erosion_map(depth, target_y * 150, target_x * 150, target);

    let mut min_time = usize::max_value();

    let mut visited = HashSet::new();
    // simulate min heap
    let mut queue: BTreeMap<usize, VecDeque<(usize, usize, Equipment, usize)>> = BTreeMap::new();
    add_to_queue(&mut queue, (0, 0, Equipment::Torch, 0));
    while !queue.is_empty() {
        // feels clumsy
        let key = queue.keys().nth(0).unwrap().clone();
        if queue[&key].is_empty() {
            queue.remove(&key);
            continue;
        }

        let (x, y, equipment, mut time) = queue.get_mut(&key).unwrap().pop_front().unwrap();

        if x == target_x && y == target_y {
            if equipment != Equipment::Torch {
                time = time + 7;
            }
            min_time = min(min_time, time);
            continue;
        }

        if visited.contains(&(x, y, equipment)) || time >= min_time || !can_use_gear(x, y, &erosion_map, equipment) {
            continue;
        }

        for new_equipment in vec![Equipment::Neither, Equipment::Torch, Equipment::ClimbingGear] {
            if can_use_gear(x, y, &erosion_map, new_equipment) {
                let new_time = time + if new_equipment == equipment {1} else {8};
                add_to_queue(&mut queue, (x + 1, y, new_equipment, new_time));
                add_to_queue(&mut queue, (x, y + 1, new_equipment, new_time));

                if x > 0 {
                    add_to_queue(&mut queue, (x - 1, y, new_equipment, new_time));
                }

                if y > 0 {
                    add_to_queue(&mut queue, (x, y - 1, new_equipment, new_time));
                }
            }
        }
        visited.insert((x, y, equipment));
    }

    return min_time;
}

fn main() {
    println!("{}", problem_1(3339, (10, 715)));
    println!("{}", problem_2(3339, (10, 715)));
}

