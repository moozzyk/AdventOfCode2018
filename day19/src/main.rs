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
    ip: i32,
    ip_reg: usize,
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

    fn run(&mut self, program: &Vec<Vec<&str>>) {

        loop {
            //  println!("{}, {}", self.ip, program.len());
            if self.ip < 0 || self.ip >= program.len() as i32 {
                break;
            }

            // println!("IP: {:02}, Registers: {:?}", self.ip, self.registers);

            self.registers[self.ip_reg] = self.ip as i32;
            let line = &program[self.ip as usize];
            let opcode = line[0];
            let arg1:i32 = line[1].parse().unwrap();
            let arg2:i32 = line[2].parse().unwrap();
            let arg3:i32 = line[3].parse().unwrap();
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
                "seti" => self.seti(arg1 as i32, arg3 as usize),
                "gtir" => self.gtir(arg1 as i32, arg2 as usize, arg3 as usize),
                "gtri" => self.gtri(arg1 as usize, arg2 as i32, arg3 as usize),
                "gtrr" => self.gtrr(arg1 as usize, arg2 as usize, arg3 as usize),
                "eqir" => self.eqir(arg1 as i32, arg2 as usize, arg3 as usize),
                "eqri" => self.eqri(arg1 as usize, arg2 as i32, arg3 as usize),
                "eqrr" => self.eqrr(arg1 as usize, arg2 as usize, arg3 as usize),
                _ => panic!("Unexpected op-code {}", line[0]),
            }
            self.ip = self.registers[self.ip_reg];
            self.ip += 1;
        }

        println!("Halted. Registers: {:?}", self.registers);
    }
}

fn problem_1() {
    let lines = lines_from_file("input.txt");
    let program: Vec<Vec<&str>> =
        lines
            .iter()
            .skip(1)
            .map(|l| l.split_whitespace().collect())
            .collect();

    let ip_reg: usize = lines[0].split_whitespace().last().unwrap().parse().unwrap();

    let mut cpu = Cpu{registers: vec![0, 0, 0, 0, 0, 0], ip: 0, ip_reg: ip_reg};
    cpu.run(&program);
}

fn problem_2() {
    const N:i64 = 10551288; // stored in r3
    let mut res:i64 = 0;
    for i in 1..=N {
        if N % i == 0 {
            res += i;
        }
    }

    println!("{}", res);
}

fn main() {
    problem_1();
    problem_2();
}
