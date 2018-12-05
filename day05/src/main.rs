use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn read_first_line<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let mut buf = BufReader::new(file);

    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    return line;
}

fn problem1() {
    let mut line = read_first_line("input.txt");
    line.pop(); // remove trailing \n

    let chars: Vec<u8> = line.as_bytes().to_vec();
    let mut stack = Vec::new();
    for c in chars {
        if stack.len() == 0 || (stack[stack.len() - 1] != c - 32 && stack[stack.len() - 1] != c + 32) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    println!("{}", stack.len());
}

fn main() {
    problem1();
}