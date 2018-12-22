
#[allow(dead_code)]
fn print_cave(cave: &Vec<Vec<usize>>) {
    for r in cave {
        for f in r {
            match f % 3 {
                0 => print!("."),
                1 => print!("="),
                2 => print!("|"),
                _ => panic!("Invalid erosion value"),
            }
        }
        println!();
    }
}

fn problem_1(depth: usize, target: (usize, usize)) -> usize {
    let (target_x, target_y) = target;
    let mut erosion = vec![vec![0; target_x + 1]; target_y + 1];
    fn calculate_erosion(geological_index: usize, depth: usize) -> usize { ((geological_index + depth) % 20183) };

    for y in 0..=target_y {
        for x in 0..=target_x {
            if (x == 0 && y == 0) || (x == target_x && y == target_y) {
                erosion[y][x] = calculate_erosion(0, depth);
            } else if x == 0 {
                erosion[y][x] = calculate_erosion(y * 48271, depth);
            } else if y == 0 {
                erosion[y][x] = calculate_erosion(x * 16807, depth);
            } else {
                erosion[y][x] = calculate_erosion(erosion[y - 1][x] * erosion[y][x - 1], depth);
            }
        }
    }

    let mut sum: usize = 0;
    for r in erosion {
        for x in r {
            sum += x % 3;
        }
    }

    return sum;
}

fn main() {
    println!("{}", problem_1(3339, (10, 715)));
}
