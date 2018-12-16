use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

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

#[derive(Debug)]
struct Cpu {
    registers: Vec<i32>,
}

impl Cpu {
    fn addr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] + self.registers[r2];
    }

    fn addi(&mut self, r1: usize, v: i32, r_out: usize) {
        self.registers[r_out] = self.registers[r1] + v;
    }

    fn mulr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] * self.registers[r2];
    }

    fn muli(&mut self, r1: usize, v: i32, r_out: usize) {
        self.registers[r_out] = self.registers[r1] * v;
    }

    fn banr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] & self.registers[r2];
    }

    fn bani(&mut self, r1: usize, v: i32, r_out: usize) {
        self.registers[r_out] = self.registers[r1] & v;
    }

    fn borr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] | self.registers[r2];
    }

    fn bori(&mut self, r1: usize, v: i32, r_out: usize) {
        self.registers[r_out] = self.registers[r1] | v;
    }

    fn setr(&mut self, r1: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1];
    }

    fn seti(&mut self, v: i32, r_out: usize) {
        self.registers[r_out] = v;
    }

    fn gtir(&mut self, v: i32, r1: usize, r_out: usize) {
        self.registers[r_out] = if v > self.registers[r1] { 1 } else { 0 };
    }

    fn gtri(&mut self, r1: usize, v: i32, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] > v { 1 } else { 0 };
    }

    fn gtrr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] > self.registers[r2] { 1 } else { 0 };
    }

    fn eqir(&mut self, v: i32, r1: usize, r_out: usize) {
        self.registers[r_out] = if v == self.registers[r1] { 1 } else { 0 };
    }

    fn eqri(&mut self, r1: usize, v: i32, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] == v { 1 } else { 0 };
    }

    fn eqrr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] == self.registers[r2] { 1 } else { 0 };
    }

    fn run(&mut self, program: &Vec<Vec<i32>>) {
        for line in program {
            // println!("{:?}", line);
            match line[0] {
                1 => self.addr(line[1] as usize, line[2] as usize, line[3] as usize),
                13 => self.addi(line[1] as usize, line[2], line[3] as usize),
                15 => self.mulr(line[1] as usize, line[2] as usize, line[3] as usize),
                14 => self.muli(line[1] as usize, line[2], line[3] as usize),
                0 => self.banr(line[1] as usize, line[2] as usize, line[3] as usize),
                9 => self.bani(line[1] as usize, line[2], line[3] as usize),
                8 => self.borr(line[1] as usize, line[2] as usize, line[3] as usize),
                5 => self.bori(line[1] as usize, line[2], line[3] as usize),
                3 => self.setr(line[1] as usize, line[3] as usize),
                7 => self.seti(line[1] as i32, line[3] as usize),
                6 => self.gtir(line[1] as i32, line[2] as usize, line[3] as usize),
                12 => self.gtri(line[1] as usize, line[2] as i32, line[3] as usize),
                4 => self.gtrr(line[1] as usize, line[2] as usize, line[3] as usize),
                10 => self.eqir(line[1] as i32, line[2] as usize, line[3] as usize),
                2 => self.eqri(line[1] as usize, line[2] as i32, line[3] as usize),
                11 => self.eqrr(line[1] as usize, line[2] as usize, line[3] as usize),
                _ => panic!("Unexpected op-code {}", line[0]),
            }
        }
    }
}

fn can_be_addr(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.addr(input[1] as usize, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_addi(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.addi(input[1] as usize, input[2], input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_mulr(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.mulr(input[1] as usize, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_muli(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.muli(input[1] as usize, input[2], input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_banr(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.banr(input[1] as usize, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_bani(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.bani(input[1] as usize, input[2], input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_borr(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.borr(input[1] as usize, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_bori(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.bori(input[1] as usize, input[2], input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_setr(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.setr(input[1] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_seti(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.seti(input[1] as i32, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_gtir(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.gtir(input[1] as i32, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_gtri(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.gtri(input[1] as usize, input[2] as i32, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_gtrr(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.gtrr(input[1] as usize, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_eqir(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.eqir(input[1] as i32, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_eqri(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.eqri(input[1] as usize, input[2] as i32, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn can_be_eqrr(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    let mut cpu = Cpu{registers: before.to_vec()};
    cpu.eqrr(input[1] as usize, input[2] as usize, input[3] as usize);
    return if &cpu.registers == after {1} else {0}
}

fn try_instructions(input: &Vec<i32>, before: &Vec<i32>, after: &Vec<i32>) -> usize {
    return
        can_be_addr(input, before, after) + can_be_addi(input, before, after) +
        can_be_mulr(input, before, after) + can_be_muli(input, before, after) +
        can_be_banr(input, before, after) + can_be_bani(input, before, after) +
        can_be_borr(input, before, after) + can_be_bori(input, before, after) +
        can_be_setr(input, before, after) + can_be_seti(input, before, after) +
        can_be_gtri(input, before, after) + can_be_gtir(input, before, after) + can_be_gtrr(input, before, after) +
        can_be_eqri(input, before, after) + can_be_eqir(input, before, after) + can_be_eqrr(input, before, after);
}

fn problem_1() {
    let lines = lines_from_file("problem1.txt");
    let mut result = 0;
    for chunk in lines.chunks(3) {
        let mut input: Vec<Vec<i32>> = Vec::new();
        for l in chunk {
            input.push(l.split_whitespace().map(|s| s.parse().unwrap()).collect());
        }

        if try_instructions(&input[1], &input[0], &input[2]) >= 3 {
            result += 1;
        }
    }

    println!("{}", result);
}

fn problem_2() {
    let lines = lines_from_file("problem2.txt");
    let program = lines
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut cpu = Cpu{registers: vec![0, 0, 0, 0]};
    cpu.run(&program);
    println!("{:?}", cpu);
}

fn main() {
    problem_1();
    problem_2();
}


