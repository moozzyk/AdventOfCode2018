use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
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

#[derive(Debug)]
struct Cpu {
    registers: Vec<i64>,
    ip: i64,
    ip_reg: usize,
}

impl Cpu {
    fn addr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] + self.registers[r2];
    }

    fn addi(&mut self, r1: usize, v: i64, r_out: usize) {
        self.registers[r_out] = self.registers[r1] + v;
    }

    fn mulr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] * self.registers[r2];
    }

    fn muli(&mut self, r1: usize, v: i64, r_out: usize) {
        self.registers[r_out] = self.registers[r1] * v;
    }

    fn banr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] & self.registers[r2];
    }

    fn bani(&mut self, r1: usize, v: i64, r_out: usize) {
        self.registers[r_out] = self.registers[r1] & v;
    }

    fn borr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1] | self.registers[r2];
    }

    fn bori(&mut self, r1: usize, v: i64, r_out: usize) {
        self.registers[r_out] = self.registers[r1] | v;
    }

    fn setr(&mut self, r1: usize, r_out: usize) {
        self.registers[r_out] = self.registers[r1];
    }

    fn seti(&mut self, v: i64, r_out: usize) {
        self.registers[r_out] = v;
    }

    fn gtir(&mut self, v: i64, r1: usize, r_out: usize) {
        self.registers[r_out] = if v > self.registers[r1] { 1 } else { 0 };
    }

    fn gtri(&mut self, r1: usize, v: i64, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] > v { 1 } else { 0 };
    }

    fn gtrr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] > self.registers[r2] { 1 } else { 0 };
    }

    fn eqir(&mut self, v: i64, r1: usize, r_out: usize) {
        self.registers[r_out] = if v == self.registers[r1] { 1 } else { 0 };
    }

    fn eqri(&mut self, r1: usize, v: i64, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] == v { 1 } else { 0 };
    }

    fn eqrr(&mut self, r1: usize, r2: usize, r_out: usize) {
        self.registers[r_out] = if self.registers[r1] == self.registers[r2] { 1 } else { 0 };
    }

    fn run(&mut self, program: &Vec<(&str, i64, i64, i64)>) {
        let mut r5_seen = HashSet::new();

        loop {
            if self.ip < 0 || self.ip >= program.len() as i64 {
                break;
            }

            if self.ip == 28 {
                println!("Registers: {:?}", self.registers);
                let r5 = self.registers[5];
                if r5_seen.contains(&r5) {
                    println!("repeated. exiting");
                    break;
                } else {
                    r5_seen.insert(r5);
                }
            }

            self.registers[self.ip_reg] = self.ip as i64;
            let (opcode, arg1, arg2, arg3) = program[self.ip as usize];
            match opcode {
                "addr" => self.addr(arg1 as usize, arg2 as usize, arg3 as usize),
                "addi" => self.addi(arg1 as usize, arg2, arg3 as usize),
                "mulr" => self.mulr(arg1 as usize, arg2 as usize, arg3 as usize),
                "muli" => self.muli(arg1 as usize, arg2, arg3 as usize),
                "banr" => self.banr(arg1 as usize, arg2 as usize, arg3 as usize),
                "bani" => self.bani(arg1 as usize, arg2, arg3 as usize),
                "borr" => self.borr(arg1 as usize, arg2 as usize, arg3 as usize),
                "bori" => self.bori(arg1 as usize, arg2, arg3 as usize),
                "setr" => self.setr(arg1 as usize, arg3 as usize),
                "seti" => self.seti(arg1 as i64, arg3 as usize),
                "gtir" => self.gtir(arg1 as i64, arg2 as usize, arg3 as usize),
                "gtri" => self.gtri(arg1 as usize, arg2 as i64, arg3 as usize),
                "gtrr" => self.gtrr(arg1 as usize, arg2 as usize, arg3 as usize),
                "eqir" => self.eqir(arg1 as i64, arg2 as usize, arg3 as usize),
                "eqri" => self.eqri(arg1 as usize, arg2 as i64, arg3 as usize),
                "eqrr" => self.eqrr(arg1 as usize, arg2 as usize, arg3 as usize),
                _ => panic!("Unexpected op-code {}", opcode),
            }
            self.ip = self.registers[self.ip_reg];
            self.ip += 1;
        }

        println!("Halted. Registers: {:?}", self.registers);
    }
}

fn problem_1_2() {
    let lines = lines_from_file("input.txt");
    let source: Vec<Vec<&str>> =
        lines
            .iter()
            .skip(1)
            .map(|l| l.split_whitespace().collect())
            .collect();

    let program = source
        .iter()
        .map(|l| (l[0], l[1].parse().unwrap(), l[2].parse().unwrap(), l[3].parse().unwrap()))
        .collect();

    let ip_reg: usize = lines[0].split_whitespace().last().unwrap().parse().unwrap();
    let mut cpu = Cpu{registers: vec![0, 0, 0, 0, 0, 0], ip: 0, ip_reg: ip_reg};
    cpu.run(&program);
}

fn main() {
    problem_1_2();
}
