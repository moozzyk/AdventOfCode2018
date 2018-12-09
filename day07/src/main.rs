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

fn build_graph(input: Vec<String>) -> (HashMap<String, Vec<String>>, HashMap<String, HashSet<String>>){
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

    return (graph, incoming_edges);
}

fn problem1(graph: &HashMap<String, Vec<String>>, incoming_edges_: &HashMap<String, HashSet<String>>) -> String {
    let mut incoming_edges = incoming_edges_.clone();

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

    return result;
}

fn problem2(graph: &HashMap<String, Vec<String>>, incoming_edges_: &HashMap<String, HashSet<String>>) -> u32 {
    let mut incoming_edges = incoming_edges_.clone();

    let mut tasks = HashSet::new();
    for n in graph.keys() {
        if !incoming_edges.contains_key(n) {
            tasks.insert(n);
        }
    }

    let min_task_time = 60;
    let mut time = 0;
    let mut available_workers = 5;
    let mut scheduled_tasks: HashMap<u32, Vec<String>> = HashMap::new();
    loop {
        match scheduled_tasks.remove(&time) {
            Some(completed_tasks) => {
                let num_completed_tasks = &completed_tasks.len();
                for task in completed_tasks {
                    if graph.contains_key(&task) {
                        for next_task in graph[&task].iter() {
                            incoming_edges.get_mut(next_task).unwrap().remove(&task);
                            if incoming_edges[next_task].is_empty() {
                                tasks.insert(next_task);
                            }
                        }
                    }
                }

                available_workers += num_completed_tasks;

                if scheduled_tasks.is_empty() && tasks.is_empty() {
                    return time;
                }
            }
            None => {}
        }

        while available_workers > 0 && !tasks.is_empty() {
            let task = tasks.iter().min().unwrap().clone();
            let completion_time = time + min_task_time + (task.as_bytes()[0] as u32 - 64);
            if !scheduled_tasks.contains_key(&completion_time) {
                scheduled_tasks.insert(completion_time, Vec::new());
            }
            scheduled_tasks.get_mut(&completion_time).unwrap().push(task.to_string());
            tasks.remove(&task);
            available_workers -= 1;
        }

        time = time + 1;
    }
}

fn main() {
    let lines = lines_from_file("input.txt");
    let (graph, incoming_edges) = build_graph(lines);

    println!("{}", problem1(&graph, &incoming_edges));
    println!("{}", problem2(&graph, &incoming_edges));
}
