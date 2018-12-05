use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::cmp;

fn read_first_line<P>(filename: P) -> String where
    P: AsRef<Path> {
    let file = File::open(filename).expect("no such file");
    let mut buf = BufReader::new(file);

    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    return line;
}

fn problem<F>(line: &String, skip: F) -> usize where
    F: Fn(u8) -> bool {
    let chars: Vec<u8> = line.as_bytes().to_vec();
    let mut stack = Vec::new();
    for c in chars {
        if !skip(c) {
            if stack.len() == 0 || (stack[stack.len() - 1] != c - 32 && stack[stack.len() - 1] != c + 32) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
    }
    return stack.len();
}

fn problem1(line: &String) {
    println!("{}", problem(&line, |_c| false));
}

fn problem2(line: &String) {
    let mut min_len = line.len() + 1;
    for c in 97..123 /* 'a' to 'z' */ {
        let len = problem(&line, |ch| c == ch | 0b00100000);
        min_len = cmp::min(min_len, len);
    }
    println!("{}", min_len);
}

fn main() {
    let mut line = read_first_line("input.txt");
    line.pop(); // remove trailing \n

    problem1(&line);
    problem2(&line);
}