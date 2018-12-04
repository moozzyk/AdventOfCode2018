use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

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

fn extract_minute(s: &String) -> i32 {
    return
        s.split(' ')
            .collect::<Vec<&str>>()[1][3..5]
            .parse::<i32>()
            .unwrap();
}
fn get_interval(from: &String, to: &String) -> (i32, i32) {
    return (extract_minute(from), extract_minute(to));
}

fn problem1() {
    let mut lines = lines_from_file("input.txt");
    lines.sort_unstable();

    let mut sleep_times = HashMap::new();

    let mut line_idx = 0;
    while line_idx < lines.len() {
        let guard_id = &lines[line_idx].split(' ').collect::<Vec<&str>>()[3][1..];
        line_idx += 1;

        if !sleep_times.contains_key(&guard_id) {
            sleep_times.insert(guard_id, Vec::new());
        }

        while line_idx < lines.len() && !&lines[line_idx].contains("Guard") {
            let interval = get_interval(&lines[line_idx], &lines[line_idx + 1]);
            sleep_times.get_mut(&guard_id).unwrap().push(interval);
            line_idx += 2;
        }
    }

    let mut max_minutes = 0;
    let mut guard_id = "";
    for (g, intervals) in &sleep_times {
        let minutes = intervals.iter().fold(0, |r, (from, to)| r + to - from);
        if minutes > max_minutes {
            max_minutes = minutes;
            guard_id = g;
        }
    }

    let mut minute_freq = [0; 60];
    for interval in &sleep_times[&guard_id] {
        let (from, to) = interval;
        for minute in *from..*to {
            minute_freq[minute as usize] += 1;
        }
    }

    let mut max_freq = 0;
    let mut max_freq_idx = 0;
    for idx in 0usize..60usize {
        if minute_freq[idx] > max_freq {
            max_freq = minute_freq[idx];
            max_freq_idx = idx;
        }
    }

    println!("{}", guard_id.parse::<i32>().unwrap() * max_freq_idx as i32);
}

// TODO: need to understand lifetimes to be able to extract these methods (i.e. returning HashMap)
fn problem2() {
    let mut lines = lines_from_file("input.txt");
    lines.sort_unstable();

    let mut sleep_times = HashMap::new();

    let mut line_idx = 0;
    while line_idx < lines.len() {
        let guard_id = &lines[line_idx].split(' ').collect::<Vec<&str>>()[3][1..];
        line_idx += 1;

        if !sleep_times.contains_key(&guard_id) {
            sleep_times.insert(guard_id, Vec::new());
        }

        while line_idx < lines.len() && !&lines[line_idx].contains("Guard") {
            let interval = get_interval(&lines[line_idx], &lines[line_idx + 1]);
            sleep_times.get_mut(&guard_id).unwrap().push(interval);
            line_idx += 2;
        }
    }

    let mut guard_id = "";
    let mut max_minute_freq = 0;
    let mut max_minute_freq_idx = 0;

    for (g, intervals) in &sleep_times {

        let mut minute_freq = [0; 60];
        for interval in intervals {
            let (from, to) = interval;
            for minute in *from..*to {
                minute_freq[minute as usize] += 1;
            }
        }

        for idx in 0usize..60usize {
            if minute_freq[idx] > max_minute_freq {
                max_minute_freq = minute_freq[idx];
                max_minute_freq_idx = idx;
                guard_id = g;
            }
        }
    }

    println!("{}", guard_id.parse::<i32>().unwrap() * max_minute_freq_idx as i32);
}

fn main() {
    problem1();
    problem2();
}
