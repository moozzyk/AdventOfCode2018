use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn problem1(input: Vec<String>) {
    let mut graph = HashMap::new();
    let mut incoming_edges = HashMap::new();

    for l in input {
        let mut parts = l.split_whitespace().collect::<Vec<&str>>();
        let parent = parts[1].to_string();
        let child = parts[7].to_string();
        if !graph.contains_key(&parent) {
            graph.insert(parent.clone(), vec![child.clone()]);
        } else {
            graph.get_mut(&parent).unwrap().push(child.clone());
        }

        if !incoming_edges.contains_key(&child) {
            incoming_edges.insert(child.clone(), HashSet::new());
        }
        incoming_edges.get_mut(&child).unwrap().insert(parent.clone());
    }

    let mut candidates = HashSet::new();
    for n in graph.keys() {
        if !incoming_edges.contains_key(n) {
            candidates.insert(n);
        }
    }

    let mut result = String::from("");
    while !candidates.is_empty() {
        let current = candidates.iter().min().unwrap().clone();
        result.push_str(current);
        candidates.remove(current);
        if graph.contains_key(current) {
            for node in graph[current].iter() {
                incoming_edges.get_mut(node).unwrap().remove(current);
                if incoming_edges[node].is_empty() {
                    candidates.insert(node);
                }
            }
        }
    }

    println!("{}", result);
}

fn main() {
    let lines = lines_from_file("input.txt");
    problem1(lines);
}
