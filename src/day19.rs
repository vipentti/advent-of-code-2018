#![allow(dead_code)]
use aoc::{CustomError, Result};

use std::io::{self, Write};

use std::ops::{Index, IndexMut};
use std::str::FromStr;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    // part1(&s)?;
    part2(&s)?;

    Ok(())
}

fn show_instructions(code: &[Instruction]) {
    for (i, c) in code.iter().enumerate() {
        eprintln!("{: <3}: {:?}", i, c);
    }
}

fn part1(s: &str) -> Result<i32> {
    let mut instructions = Vec::new();

    let ips: Vec<_> = s
        .lines()
        .filter(|s| s.starts_with("#ip"))
        .map(|s| s.replace("#ip ", ""))
        .collect();

    assert!(ips.len() <= 1);

    let ip_reg = ips
        .first()
        .unwrap()
        .parse::<usize>()
        .map_err(|e| Box::new(e))?;

    for line in s.lines().filter(|s| !s.starts_with('#')) {
        let inst = line.parse::<Instruction>()?;
        instructions.push(inst);
    }

    // eprintln!("{:?}", instructions);
    eprintln!("{}", ip_reg);

    let mut machine = Machine::empty();
    show_instructions(&instructions);

    machine.set_ip_reg(ip_reg);
    machine.set_code(instructions);

    eprintln!("{:?}", machine);

    machine.run()?;

    eprintln!("{:?}", machine);

    Ok(0)
}

fn part2(s: &str) -> Result<i32> {
    let mut instructions = Vec::new();

    let ips: Vec<_> = s
        .lines()
        .filter(|s| s.starts_with("#ip"))
        .map(|s| s.replace("#ip ", ""))
        .collect();

    assert!(ips.len() <= 1);

    let ip_reg = ips
        .first()
        .unwrap()
        .parse::<usize>()
        .map_err(|e| Box::new(e))?;

    for line in s.lines().filter(|s| !s.starts_with('#')) {
        let inst = line.parse::<Instruction>()?;
        instructions.push(inst);
    }

    eprintln!("{:?}", instructions);
    show_instructions(&instructions);
    eprintln!("{}", ip_reg);

    let mut machine = Machine::empty();

    machine.set_ip_reg(ip_reg);
    machine.set_code(instructions);
    machine.set_registers(Registers([1, 0, 0, 0, 0, 0]));

    eprintln!("{:?}", machine);

    machine.skip()?;

    eprintln!("{:?}", machine);
    if machine.ip < machine.code.len() {
        eprintln!("{:?}", machine.code[machine.ip - 1]);
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let input = r"
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
        ";

        assert_eq!(6, part1(input.trim()).unwrap());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Reg(i32);

impl From<i32> for Reg {
    fn from(v: i32) -> Self {
        Reg(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Value(i32);

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Instruction {
    Addr(Reg, Reg, Reg),
    Addi(Reg, Value, Reg),

    Mulr(Reg, Reg, Reg),
    Muli(Reg, Value, Reg),

    Banr(Reg, Reg, Reg),
    Bani(Reg, Value, Reg),

    Borr(Reg, Reg, Reg),
    Bori(Reg, Value, Reg),

    Setr(Reg, Reg, Reg),
    Seti(Value, Reg, Reg),

    Gtir(Value, Reg, Reg),
    Gtri(Reg, Value, Reg),
    Gtrr(Reg, Reg, Reg),

    Eqir(Value, Reg, Reg),
    Eqri(Reg, Value, Reg),
    Eqrr(Reg, Reg, Reg),
}

type InstructionFn = fn(i32, i32, i32) -> Instruction;

impl Instruction {
    fn addr(a: i32, b: i32, c: i32) -> Self {
        Instruction::Addr(a.into(), b.into(), c.into())
    }
    fn addi(a: i32, b: i32, c: i32) -> Self {
        Instruction::Addi(a.into(), b.into(), c.into())
    }
    fn mulr(a: i32, b: i32, c: i32) -> Self {
        Instruction::Mulr(a.into(), b.into(), c.into())
    }
    fn muli(a: i32, b: i32, c: i32) -> Self {
        Instruction::Muli(a.into(), b.into(), c.into())
    }
    fn banr(a: i32, b: i32, c: i32) -> Self {
        Instruction::Banr(a.into(), b.into(), c.into())
    }
    fn bani(a: i32, b: i32, c: i32) -> Self {
        Instruction::Bani(a.into(), b.into(), c.into())
    }
    fn borr(a: i32, b: i32, c: i32) -> Self {
        Instruction::Borr(a.into(), b.into(), c.into())
    }
    fn bori(a: i32, b: i32, c: i32) -> Self {
        Instruction::Bori(a.into(), b.into(), c.into())
    }
    fn setr(a: i32, b: i32, c: i32) -> Self {
        Instruction::Setr(a.into(), b.into(), c.into())
    }
    fn seti(a: i32, b: i32, c: i32) -> Self {
        Instruction::Seti(a.into(), b.into(), c.into())
    }
    fn gtir(a: i32, b: i32, c: i32) -> Self {
        Instruction::Gtir(a.into(), b.into(), c.into())
    }
    fn gtri(a: i32, b: i32, c: i32) -> Self {
        Instruction::Gtri(a.into(), b.into(), c.into())
    }
    fn gtrr(a: i32, b: i32, c: i32) -> Self {
        Instruction::Gtrr(a.into(), b.into(), c.into())
    }
    fn eqir(a: i32, b: i32, c: i32) -> Self {
        Instruction::Eqir(a.into(), b.into(), c.into())
    }
    fn eqri(a: i32, b: i32, c: i32) -> Self {
        Instruction::Eqri(a.into(), b.into(), c.into())
    }
    fn eqrr(a: i32, b: i32, c: i32) -> Self {
        Instruction::Eqrr(a.into(), b.into(), c.into())
    }

    fn to_ctor(&self) -> InstructionFn {
        match self {
            Instruction::Addr(_, _, _) => Instruction::addr,
            Instruction::Addi(_, _, _) => Instruction::addi,
            Instruction::Mulr(_, _, _) => Instruction::mulr,
            Instruction::Muli(_, _, _) => Instruction::muli,
            Instruction::Banr(_, _, _) => Instruction::banr,
            Instruction::Bani(_, _, _) => Instruction::bani,
            Instruction::Borr(_, _, _) => Instruction::borr,
            Instruction::Bori(_, _, _) => Instruction::bori,
            Instruction::Setr(_, _, _) => Instruction::setr,
            Instruction::Seti(_, _, _) => Instruction::seti,
            Instruction::Gtir(_, _, _) => Instruction::gtir,
            Instruction::Gtri(_, _, _) => Instruction::gtri,
            Instruction::Gtrr(_, _, _) => Instruction::gtrr,
            Instruction::Eqir(_, _, _) => Instruction::eqir,
            Instruction::Eqri(_, _, _) => Instruction::eqri,
            Instruction::Eqrr(_, _, _) => Instruction::eqrr,
        }
    }

    fn is_comparison(&self) -> bool {
        use self::Instruction::*;
        match self {
            Instruction::Gtir(_, _, _) => true,
            Instruction::Gtri(_, _, _) => true,
            Instruction::Gtrr(_, _, _) => true,
            Instruction::Eqir(_, _, _) => true,
            Instruction::Eqri(_, _, _) => true,
            Instruction::Eqrr(_, _, _) => true,

            _ => false,
        }
    }
}

impl FromStr for Instruction {
    type Err = CustomError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values: Vec<&str> = s
            .trim_matches(|p| p == '[' || p == ']')
            .split(' ')
            .map(|s| s.trim())
            .collect();

        let inst = values[0];
        let a = values[1]
            .parse::<i32>()
            .map_err(|_| CustomError("Invalid value".to_string()))?;
        let b = values[2]
            .parse::<i32>()
            .map_err(|_| CustomError("Invalid value".to_string()))?;
        let c = values[3]
            .parse::<i32>()
            .map_err(|_| CustomError("Invalid value".to_string()))?;

        match inst {
            "addr" => Ok(Instruction::addr(a, b, c)),
            "addi" => Ok(Instruction::addi(a, b, c)),
            "mulr" => Ok(Instruction::mulr(a, b, c)),
            "muli" => Ok(Instruction::muli(a, b, c)),
            "banr" => Ok(Instruction::banr(a, b, c)),
            "bani" => Ok(Instruction::bani(a, b, c)),
            "borr" => Ok(Instruction::borr(a, b, c)),
            "bori" => Ok(Instruction::bori(a, b, c)),
            "setr" => Ok(Instruction::setr(a, b, c)),
            "seti" => Ok(Instruction::seti(a, b, c)),
            "gtir" => Ok(Instruction::gtir(a, b, c)),
            "gtri" => Ok(Instruction::gtri(a, b, c)),
            "gtrr" => Ok(Instruction::gtrr(a, b, c)),
            "eqir" => Ok(Instruction::eqir(a, b, c)),
            "eqri" => Ok(Instruction::eqri(a, b, c)),
            "eqrr" => Ok(Instruction::eqrr(a, b, c)),
            _ => Err(CustomError("Invalid instruction".to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Registers([i32; 6]);

impl Index<Reg> for Registers {
    type Output = i32;

    fn index(&self, index: Reg) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<Reg> for Registers {
    fn index_mut(&mut self, index: Reg) -> &mut i32 {
        &mut self.0[index.0 as usize]
    }
}

#[derive(Debug, Clone)]
struct Machine {
    registers: Registers,
    ip_reg: Option<Reg>,
    ip: usize,
    code: Vec<Instruction>,
}

impl Machine {
    fn empty() -> Self {
        Machine {
            registers: Default::default(),
            ip_reg: None,
            ip: 0,
            code: Vec::new(),
        }
    }

    fn with_regs(regs: Registers) -> Self {
        Machine {
            registers: regs,
            ip_reg: None,
            ip: 0,
            code: Vec::new(),
        }
    }

    fn set_code(&mut self, code: Vec<Instruction>) {
        self.code = code;
    }

    fn set_ip_reg(&mut self, reg: usize) {
        self.ip_reg = Some(Reg(reg as i32));
    }

    fn registers(&self) -> &Registers {
        &self.registers
    }

    fn set_registers(&mut self, regs: Registers) {
        self.registers = regs;
    }

    fn get_ip(&self) -> usize {
        if let Some(reg) = self.ip_reg {
            self.registers[reg] as usize
        } else {
            self.ip
        }
    }

    fn store_ip(&mut self) {
        if let Some(reg) = self.ip_reg {
            self.registers[reg] = self.ip as i32;
        }
    }

    fn move_ip(&mut self) {
        if let Some(reg) = self.ip_reg {
            let reg_val = self.registers[reg] as usize;

            if reg_val != self.ip {
                self.ip = reg_val;
            // self.ip += 1;
            } else {
                // self.ip += 1;
            }
        } else {
            // self.ip += 1;
        }

        self.ip += 1;
    }

    fn skip(&mut self) -> Result<()> {
        /*
        let mut r1 = 1;
        let mut r3 = 996;
        let mut r0 = 0;
        let mut r4 = 0;


        while r1 <= r3 {
            let mut r5 = 1;

            while r5 <= r3 {
                r4 = r5 * r1;

                if r3 == r4 {
                    r0 = r0 + r1;
                } else {
                    r5 = r5 + 1;
                }
            }

            r1 = r1 + 1;
        }

        eprintln!("")
        */

        let r0 = Reg(0);
        let r1 = Reg(1);
        let r3 = Reg(3);
        let r4 = Reg(4);
        let r5 = Reg(5);

        self.registers[r1] = 1;
        self.registers[r3] = 10_551_396; //996;
                                         // self.registers[r3] = 996; //996;
        self.registers[r0] = 0;
        self.registers[r4] = 0;

        eprintln!("{:?}", self.registers);

        // First R5 * R1 = 996
        // R1 = 1
        // R5 = 996

        // Second R5 * R1 = 996
        // R1 = 2
        // R5 = 996 / 2 = 498

        self.registers[r5] = 1;
        while self.registers[r1] <= self.registers[r3] {
            // if r5 * r1 == r3 -> r0 = r0 + r1

            // let step = self.registers[r3] / self.registers[r1];

            // self.registers[r5] = self.registers[r5] + step;

            let step = self.registers[r3] / self.registers[r1];
            self.registers[r5] = step;

            if self.registers[r5] * self.registers[r1] == self.registers[r3] {
                self.registers[r0] = self.registers[r0] + self.registers[r1];
                // eprintln!("A: {:?}", self.registers);
            }

            // self.registers[r5] = step;

            // while self.registers[r5] <= self.registers[r3] {
            //     self.registers[r4] = self.registers[r5] * self.registers[r1];

            //     if self.registers[r3] == self.registers[r4] {
            //         self.registers[r0] =  self.registers[r0] + self.registers[r1];
            //         eprintln!("A: {:?}", self.registers);
            //     }
            //     self.registers[r5] =  self.registers[r5] + 1;
            // }
            // eprintln!("{: <3} B: {:?}", step, self.registers);

            self.registers[r1] = self.registers[r1] + 1;
        }

        Ok(())
    }

    fn run_opt(&mut self) -> Result<()> {
        let wait_reg = Reg(5);
        let expected = Reg(3);
        let value_reg = Reg(0);
        let add_reg = Reg(0);
        eprintln!();
        let mut c = 0usize;
        let mut prev = 0;
        let mut prev1 = 0;
        let mut prev4 = 0;
        /*
         *
         * If R4 == R3 {
         *    R0 = R0 + R1
         *    R5 = R5 + 1
         * }
         *
         * If R5 > R3 {
         * }
         */
        loop {
            self.store_ip();
            let ip = self.get_ip();
            if ip > self.code.len() - 1 {
                break;
            }
            let inst = self.code[ip];

            // eprintln!("Before {:?}", self.registers);

            // eprintln!("Executing {} -> {:?}", ip, inst);
            let before = self.registers.clone();
            self.execute(inst)?;
            self.move_ip();

            // if self.registers[Reg(0)] > 2000 && self.registers[Reg(5)] > 980 {
            if self.registers[Reg(0)] != prev
                || prev1 != self.registers[Reg(1)]
                || self.registers[Reg(1)] == self.registers[Reg(5)]
                || self.registers[Reg(4)] == self.registers[Reg(3)]
                || (prev4 == 0 && self.registers[Reg(4)] == 1)
            {
                prev = self.registers[Reg(0)];
                prev1 = self.registers[Reg(1)];
                c += 1;
            }
            eprintln!(
                "{: <3}: {:?} {:?} {:?}",
                ip, before, inst, self.registers
            );

            prev4 = self.registers[Reg(4)];

            // eprintln!("After {:?}", self.registers);
            // eprintln!();
        }
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        eprintln!();
        let mut c = 0usize;
        let mut prev = 0;
        let mut prev1 = 0;
        let mut prev4 = 0;

        let mut previ = Instruction::addr(0, 0, 0);

        let stderr = io::stderr();
        let mut handle = stderr.lock();
        loop {
            self.store_ip();
            let ip = self.get_ip();
            if ip > self.code.len() - 1 {
                break;
            }
            let inst = self.code[ip];

            // eprintln!("Before {:?}", self.registers);

            // eprintln!("Executing {} -> {:?}", ip, inst);
            let before = self.registers.clone();
            self.execute(inst)?;
            self.move_ip();

            // if self.registers[Reg(0)] > 2000 && self.registers[Reg(5)] > 980 {
            if self.registers[Reg(0)] != prev
                || prev1 != self.registers[Reg(1)]
                || self.registers[Reg(1)] == self.registers[Reg(5)]
                || self.registers[Reg(4)] == self.registers[Reg(3)]
                || (prev4 == 0 && self.registers[Reg(4)] == 1)
                || (ip == 15)
            {
                prev = self.registers[Reg(0)];
                prev1 = self.registers[Reg(1)];
                // writeln!(handle, "{:?} {:?} {:?}", before, inst, self.registers)?;
                c += 1;
            }
            writeln!(
                handle,
                "{: <3}: {:?} {:?} {:?}",
                ip, before, inst, self.registers
            );
            // eprintln!("{:?} {:?} {:?}", before, inst, self.registers);

            // eprintln!("After {:?}", self.registers);
            // eprintln!();
        }
        Ok(())
    }

    fn execute(&mut self, instr: Instruction) -> Result<()> {
        use self::Instruction::*;

        match instr {
            Addr(a, b, c) => {
                self.registers[c] = self.registers[a] + self.registers[b];
            }
            Addi(a, b, c) => {
                self.registers[c] = self.registers[a] + b.0;
            }
            Mulr(a, b, c) => {
                self.registers[c] = self.registers[a] * self.registers[b];
            }
            Muli(a, b, c) => {
                self.registers[c] = self.registers[a] * b.0;
            }
            Banr(a, b, c) => {
                self.registers[c] = self.registers[a] & self.registers[b];
            }
            Bani(a, b, c) => {
                self.registers[c] = self.registers[a] & b.0;
            }
            Borr(a, b, c) => {
                self.registers[c] = self.registers[a] | self.registers[b];
            }
            Bori(a, b, c) => {
                self.registers[c] = self.registers[a] | b.0;
            }
            Setr(a, _b, c) => {
                self.registers[c] = self.registers[a];
            }
            Seti(a, _b, c) => {
                self.registers[c] = a.0;
            }
            Gtir(a, b, c) => {
                if a.0 > self.registers[b] {
                    self.registers[c] = 1;
                } else {
                    self.registers[c] = 0;
                }
            }
            Gtri(a, b, c) => {
                if self.registers[a] > b.0 {
                    self.registers[c] = 1;
                } else {
                    self.registers[c] = 0;
                }
            }
            Gtrr(a, b, c) => {
                if self.registers[a] > self.registers[b] {
                    self.registers[c] = 1;
                } else {
                    self.registers[c] = 0;
                }
            }
            Eqir(a, b, c) => {
                if a.0 == self.registers[b] {
                    self.registers[c] = 1;
                } else {
                    self.registers[c] = 0;
                }
            }
            Eqri(a, b, c) => {
                if self.registers[a] == b.0 {
                    self.registers[c] = 1;
                } else {
                    self.registers[c] = 0;
                }
            }
            Eqrr(a, b, c) => {
                if self.registers[a] == self.registers[b] {
                    self.registers[c] = 1;
                } else {
                    self.registers[c] = 0;
                }
            }
        }

        Ok(())
    }
}
