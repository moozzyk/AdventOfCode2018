fn problem_1(input: usize) {
    let mut recipes = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;

    while recipes.len() < input + 10 {
        let new_recipe = recipes[e1] + recipes[e2];
        if new_recipe > 9 {
            recipes.push((new_recipe / 10) % 10);
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
}
