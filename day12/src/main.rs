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

fn grow_plants(initial: &Vec<char>, transform: &HashMap<String, char>, num_generations: usize) -> Vec<char>  {
    let mut plants = vec!['.'; SIZE];
    let middle = plants.len() / 2;
    for i in 0..initial.len() {
        plants[middle + i] = initial[i];
    }

    for _generation in 0..num_generations {
        generation(&mut plants, transform);
    }

    return plants;
}

fn problem_1(initial: &Vec<char>, transform: &HashMap<String, char>) {

    let plants = grow_plants(initial, transform, 20);
    let mut result = 0;
    for i in 0..plants.len() {
        if plants[i] == '#' {
            result += -(SIZE as i32)/2 + i as i32;
        }
    }

    println!("{}", result);
}

fn problem_2(initial: &Vec<char>, transform: &HashMap<String, char>) {

    let plants = grow_plants(initial, transform, 100);
    let mut result = 0;
    for i in 0..plants.len() {
        if plants[i] == '#' {
            result += -(SIZE as i64)/2 + i as i64 + 50000000000 - 100;
        }
    }

    println!("{}", result);
}


fn main() {
    let transform = create_transform();
    let initial = ".##..#.#..##..##..##...#####.#.....#..#..##.###.#.####......#.......#..###.#.#.##.#.#.###...##.###.#";
    problem_1(&initial.chars().collect(), &transform);
    problem_2(&initial.chars().collect(), &transform);
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