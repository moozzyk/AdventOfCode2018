use std::collections::HashMap;

const SIZE: usize = 500;

fn generation(plants: &mut Vec<char>, transform: &HashMap<String, char>) {
    let mut new_generation = Vec::new();
    for i in 2..plants.len() - 2 {
        let w:String = plants[i - 2..=i + 2].iter().collect();
        match transform.get(&w) {
            Some(t) => new_generation.push(t),
            _ => new_generation.push(&'.'),
        }
    }

    for i in 0..new_generation.len() {
        plants[2 + i] = *new_generation[i];
    }
}

fn problem_1(initial: &Vec<char>, transform: &HashMap<String, char>) {
    let mut plants = vec!['.'; SIZE];
    let middle = plants.len() / 2;
    for i in 0..initial.len() {
        plants[middle + i] = initial[i];
    }

    for _generation in 0..20 {
        generation(&mut plants, transform);
    }

    let mut result = 0;
    for i in 0..plants.len() {
        if plants[i] == '#' {
            result += -(SIZE as i32)/2 + i as i32;
        }
    }

    println!("{}", result);
}

fn main() {
    let transform = create_transform();
    let initial = ".##..#.#..##..##..##...#####.#.....#..#..##.###.#.####......#.......#..###.#.#.##.#.#.###...##.###.#";
    problem_1(&initial.chars().collect(), &transform);
}

fn create_transform() -> HashMap<String, char> {
    let mut transform = HashMap::new();

    transform.insert(".##.#".to_string(), '#');
    transform.insert("##.#.".to_string(), '#');
    transform.insert("##...".to_string(), '#');
    transform.insert("#....".to_string(), '.');
    transform.insert(".#..#".to_string(), '.');
    transform.insert("#.##.".to_string(), '.');
    transform.insert(".##..".to_string(), '.');
    transform.insert(".#.##".to_string(), '.');
    transform.insert("###..".to_string(), '.');
    transform.insert("..##.".to_string(), '#');
    transform.insert("#####".to_string(), '#');
    transform.insert("#...#".to_string(), '#');
    transform.insert(".#...".to_string(), '#');
    transform.insert("###.#".to_string(), '#');
    transform.insert("#.###".to_string(), '#');
    transform.insert("##..#".to_string(), '.');
    transform.insert(".###.".to_string(), '#');
    transform.insert("...##".to_string(), '.');
    transform.insert("..#.#".to_string(), '.');
    transform.insert("##.##".to_string(), '#');
    transform.insert("....#".to_string(), '.');
    transform.insert("#.#.#".to_string(), '#');
    transform.insert("#.#..".to_string(), '.');
    transform.insert(".####".to_string(), '.');
    transform.insert("...#.".to_string(), '#');
    transform.insert("..###".to_string(), '.');
    transform.insert("..#..".to_string(), '#');
    transform.insert(".....".to_string(), '.');
    transform.insert("####.".to_string(), '.');
    transform.insert("#..##".to_string(), '#');
    transform.insert(".#.#.".to_string(), '.');
    transform.insert("#..#.".to_string(), '#');
    transform
}