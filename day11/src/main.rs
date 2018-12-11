fn cell_power_level(x: usize, y: usize, serial_number: usize) -> i32 {
    let rack_id = x + 10;
    return (((((rack_id * y + serial_number) * rack_id) / 100) % 10)) as i32 - 5;
}

fn calculate_power(start_x: usize, start_y: usize, size: usize, serial_number: usize) -> i32 {
    let mut power_level = 0;
    for x in start_x..start_x + size {
        for y in start_y..start_y + size {
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
            let p = calculate_power(x, y, 3, serial_number);
            if p > max_power {
                max_power = p;
                coordinates = (x, y);
            }
        }
    }
    return coordinates;
}

fn fill_power_grid(grid: &mut[[i32; 301]; 301], serial_number: usize) {
    for x in 1..=300 {
        for y in 1..=300 {
            grid[x][y] = cell_power_level(x, y, serial_number);
        }
    }
}

fn fill_sums(grid: &[[i32; 301]; 301], row_sum: &mut[[i32; 301]; 301], col_sum: &mut[[i32; 301]; 301]) {
    for x in 1..300 {
        for y in 1..300 {
            row_sum[x][y] = row_sum[x][y - 1] + grid[x][y];
            col_sum[x][y] = col_sum[x - 1][y] + grid[x][y];
        }
    }
}

fn problem_2(serial_number: usize) -> (usize, usize, usize) {
    let mut grid = [[0i32; 301]; 301];
    let mut row_sum = [[0i32; 301]; 301];
    let mut col_sum = [[0i32; 301]; 301];
    fill_power_grid(&mut grid, serial_number);
    fill_sums(&grid, &mut row_sum, &mut col_sum);

    let mut max_power = i32::min_value();
    let mut result = (0, 0, 0);
    for x in 1..300 {
        for y in 1..300 {
            let mut power = 0;
            for size in 0..300 - std::cmp::max(x, y) {
                let row_sum_val = row_sum[x + size][y + size - 1] - row_sum[x + size][y - 1];
                let col_sum_val = col_sum[x + size - 1][y + size] - col_sum[x - 1][y + size];
                let grid_val = grid[x + size][y + size];

                power = power + row_sum_val + col_sum_val + grid_val;

                if power > max_power {
                    max_power = power;
                    result = (x, y, size + 1);
                }
            }
        }
    }

    return result;
}

fn main() {
    println!("{:?}", problem_1(9005));
    println!("{:?}", problem_2(9005));
}
