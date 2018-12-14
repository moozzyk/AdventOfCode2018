fn match_suffix(recipes: &Vec<usize>, suffix: &Vec<usize>) -> bool {
    return recipes.len() >= suffix.len() && recipes[recipes.len() - suffix.len()..] == suffix[..];
}

fn num_to_vec(input: usize) -> Vec<usize> {
    let mut suffix = vec![];
    let mut x = input;
    while x > 0 {
        suffix.push(x % 10);
        x = x / 10;
    }
    suffix.reverse();
    suffix
}

fn problem_2(input: usize) {
    let suffix = num_to_vec(input);
    let mut recipes = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;

    loop {
        let new_recipe = recipes[e1] + recipes[e2];
        if new_recipe > 9 {
            recipes.push(new_recipe / 10);
            if match_suffix(&recipes, &suffix) {
                break;
            }
        }
        recipes.push(new_recipe % 10);
        if match_suffix(&recipes, &suffix) {
            break;
        }

        e1 = (e1 + recipes[e1] + 1) % recipes.len();
        e2 = (e2 + recipes[e2] + 1) % recipes.len();
    }

    println!("{}", recipes.len() - suffix.len());
}

fn problem_1(input: usize) {
    let mut recipes = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;

    while recipes.len() < input + 10 {
        let new_recipe = recipes[e1] + recipes[e2];
        if new_recipe > 9 {
            recipes.push(new_recipe / 10);
        }
        recipes.push(new_recipe % 10);

        e1 = (e1 + recipes[e1] + 1) % recipes.len();
        e2 = (e2 + recipes[e2] + 1) % recipes.len();
    }

    for c in recipes[input..input + 10].iter() {
        print!("{}", c);
    }
    println!();
}

fn main() {
    problem_1(540391);
    problem_2(540391);
}
