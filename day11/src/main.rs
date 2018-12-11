fn cell_power_level(x: usize, y: usize, serial_number: usize) -> i32 {
    let rack_id = x + 10;
    return (((((rack_id * y + serial_number) * rack_id) / 100) % 10)) as i32 - 5;
}

fn calculate_power(start_x: usize, start_y: usize, serial_number: usize) -> i32 {
    let mut power_level = 0;
    for x in start_x..=start_x + 2 {
        for y in start_y..=start_y + 2 {
            power_level += cell_power_level(x, y, serial_number);
        }
    }

    return power_level;
}

fn problem_1(serial_number: usize) -> (usize, usize) {
    let mut max_power = i32::min_value();
    let mut coordinates = (0, 0);
    for x in 1..=298 {
        for y in 1..=298 {
            let p = calculate_power(x, y, serial_number);
            if p > max_power {
                max_power = p;
                coordinates = (x, y);
            }
        }
    }
    return coordinates;
}

fn main() {
    println!("{:?}", problem_1(9005));
}
