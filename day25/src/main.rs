use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn dist(c1: &Vec<i32>, c2: &Vec<i32>) -> i32 {
    (c1[0] - c2[0]).abs() + (c1[1] - c2[1]).abs() + (c1[2] - c2[2]).abs() + (c1[3] - c2[3]).abs()
}

fn bfs(start: usize, graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>) {
    let mut q = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        if visited.contains(&n) {
            continue;
        }

        visited.insert(n);
        for neighbor in graph[n].iter() {
            q.push_back(*neighbor);
        }
    }
}

fn problem_1() {
    let lines = lines_from_file("input.txt");
    let coordinates: Vec<Vec<i32>> = lines.iter().map(|l| l.split(',').map(|n| n.parse().unwrap()).collect()).collect();

    let mut graph = vec![vec![];coordinates.len()];

    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            if dist(&coordinates[i], &coordinates[j]) <= 3 {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut components = 0;

    for n in 0..coordinates.len() {
        if !visited.contains(&n) {
            components += 1;
            bfs(n, &graph, &mut visited);
        }
    }

    println!("{}", components);
}

fn main() {
    problem_1();
}
