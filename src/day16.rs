#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::Result;

use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Registers([i32; 4]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct ByteCode {
    instruction: i32,
    a: i32,
    b: i32,
    c: i32,
}

fn maybe_register(val: i32) -> bool {
    val >= 0 && val <= 3
}

impl ByteCode {
    fn to_possible_instructions(&self) -> Vec<Instruction> {
        // A cannot may be a register
        use self::Instruction::*;

        let a = self.a;
        let b = self.b;
        let c = self.c;

        let inst: Vec<Instruction> = vec![
            Addr(a.into(), b.into(), c.into()),
            Addi(a.into(), b.into(), c.into()),
            Mulr(a.into(), b.into(), c.into()),
            Muli(a.into(), b.into(), c.into()),
            Banr(a.into(), b.into(), c.into()),
            Bani(a.into(), b.into(), c.into()),
            Borr(a.into(), b.into(), c.into()),
            Bori(a.into(), b.into(), c.into()),
            Setr(a.into(), b.into(), c.into()),
            Seti(a.into(), b.into(), c.into()),
            Gtir(a.into(), b.into(), c.into()),
            Gtri(a.into(), b.into(), c.into()),
            Gtrr(a.into(), b.into(), c.into()),
            Eqir(a.into(), b.into(), c.into()),
            Eqri(a.into(), b.into(), c.into()),
            Eqrr(a.into(), b.into(), c.into()),
        ];
        inst
    }
}

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

impl FromStr for Registers {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values: Vec<&str> = s
            .trim_matches(|p| p == '[' || p == ']')
            .split(',')
            .map(|s| s.trim())
            .collect();

        let reg_0 = values[0].parse::<i32>()?;
        let reg_1 = values[1].parse::<i32>()?;
        let reg_2 = values[2].parse::<i32>()?;
        let reg_3 = values[3].parse::<i32>()?;

        Ok(Registers([reg_0, reg_1, reg_2, reg_3]))
    }
}

impl FromStr for ByteCode {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values: Vec<&str> = s
            .trim_matches(|p| p == '[' || p == ']')
            .split(' ')
            .map(|s| s.trim())
            .collect();

        let reg_0 = values[0].parse::<i32>()?;
        let reg_1 = values[1].parse::<i32>()?;
        let reg_2 = values[2].parse::<i32>()?;
        let reg_3 = values[3].parse::<i32>()?;

        // Ok(ByteCode([reg_0, reg_1, reg_2, reg_3]))
        Ok(ByteCode {
            instruction: reg_0,
            a: reg_1,
            b: reg_2,
            c: reg_3,
        })
    }
}

struct Machine {
    registers: Registers,
}

impl Machine {
    fn empty() -> Self {
        Machine {
            registers: Default::default(),
        }
    }

    fn with_regs(regs: Registers) -> Self {
        Machine { registers: regs }
    }

    fn registers(&self) -> &Registers {
        &self.registers
    }

    fn set_registers(&mut self, regs: Registers) {
        self.registers = regs;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Test {
    initial: Registers,
    expected: Registers,
    code: ByteCode,
}

impl Test {
    fn run_test(&self) -> usize {
        let instructions = self.code.to_possible_instructions();

        let mut count = 0;

        for instr in instructions {
            let mut machine = Machine::with_regs(self.initial);

            machine.execute(instr).unwrap();

            if machine.registers() == &self.expected {
                count += 1;
            }
        }

        count
    }

    fn run_with(&self, instructions: &[Instruction]) -> Vec<Instruction> {
        let mut result = Vec::new();

        for instr in instructions {
            let mut machine = Machine::with_regs(self.initial);

            let ctor = instr.to_ctor();

            let a = self.code.a;
            let b = self.code.b;
            let c = self.code.c;

            let transformed = (ctor)(a, b, c);

            machine.execute(transformed).unwrap();

            if machine.registers() == &self.expected {
                result.push(transformed);
            }
        }

        result
    }

    fn get_instructions(&self) -> (ByteCode, Vec<Instruction>) {
        let instructions = self.code.to_possible_instructions();

        let mut result = Vec::new();

        for instr in instructions {
            let mut machine = Machine::with_regs(self.initial);

            machine.execute(instr).unwrap();

            if machine.registers() == &self.expected {
                result.push(instr);
            }
        }

        (self.code, result)
    }
}

fn part1(s: &str) -> Result<i32> {
    let lines: Vec<_> = s.lines().filter(|f| !f.is_empty()).collect();

    let mut tests: Vec<_> = Vec::new();

    for index in (0..lines.len()).step_by(3) {
        let before = &lines[index];
        let input = &lines[index + 1];
        let after = &lines[index + 2];

        if !before.starts_with("Before: ") {
            eprintln!("Broke at {} {}", index, before);
            break;
        }

        assert!(before.starts_with("Before: "));
        assert!(after.starts_with("After:  "));

        let initial_regs =
            before.replace("Before: ", "").parse::<Registers>()?;
        let result_regs = after.replace("After:  ", "").parse::<Registers>()?;
        let bytecode = input.parse::<ByteCode>()?;

        // eprintln!("before {:?}", initial_regs);
        // eprintln!("input {:?}", bytecode);
        // eprintln!("after {:?}", result_regs);

        tests.push(Test {
            initial: initial_regs,
            expected: result_regs,
            code: bytecode,
        });
    }

    let how_many = tests.iter().map(Test::run_test).filter(|v| *v >= 3).count();

    eprintln!("part1 result {}", how_many);

    Ok(how_many as i32)
}

fn collect_instructions_from_samples(
    mut tests: HashMap<i32, Vec<Test>>,
) -> HashMap<i32, InstructionFn> {
    let mut instruction_map: HashMap<i32, InstructionFn> = HashMap::new();
    let mut inst_set: HashSet<InstructionFn> = HashSet::new();

    let mut count = 0;

    loop {
        if count >= 1000 || tests.is_empty() {
            break;
        }

        for (k, tests) in tests.iter() {
            let first = tests.first().unwrap();
            let (_, original) = first.get_instructions();

            let mut instrs: Vec<_> = original
                .into_iter()
                .filter(|inst| !inst_set.contains(&inst.to_ctor()))
                .collect();
            // eprintln!("start k: {} -> {:?}", k, instrs);

            for test in tests.iter().skip(1) {
                instrs = test.run_with(&instrs);
                if instrs.is_empty() {
                    break;
                }
                // eprintln!("run k: {} -> {:?}", k, instrs);
            }

            if instrs.len() == 1 {
                eprintln!("{} end k: {} -> {:?}", count, k, instrs);
                instruction_map.insert(*k, instrs.first().unwrap().to_ctor());
                inst_set.insert(instrs.first().unwrap().to_ctor());
            }
        }

        for k in instruction_map.keys() {
            tests.remove(&k);
        }

        count += 1;
    }

    instruction_map
}

fn part2(s: &str) -> Result<i32> {
    let lines: Vec<_> = s.lines().filter(|f| !f.is_empty()).collect();

    let mut tests: Vec<_> = Vec::new();

    let mut test_program_start = 0;

    let mut instruction_tests: HashMap<i32, Vec<Test>> = HashMap::new();

    for index in (0..lines.len()).step_by(3) {
        let before = &lines[index];
        let input = &lines[index + 1];
        let after = &lines[index + 2];

        if !before.starts_with("Before: ") {
            eprintln!("Broke at {} {}", index, before);
            test_program_start = index;
            break;
        }

        assert!(before.starts_with("Before: "));
        assert!(after.starts_with("After:  "));

        let initial_regs =
            before.replace("Before: ", "").parse::<Registers>()?;
        let result_regs = after.replace("After:  ", "").parse::<Registers>()?;
        let bytecode = input.parse::<ByteCode>()?;

        // eprintln!("before {:?}", initial_regs);
        // eprintln!("input {:?}", bytecode);
        // eprintln!("after {:?}", result_regs);

        tests.push(Test {
            initial: initial_regs,
            expected: result_regs,
            code: bytecode,
        });

        instruction_tests
            .entry(bytecode.instruction)
            .or_insert_with(Vec::new)
            .push(Test {
                initial: initial_regs,
                expected: result_regs,
                code: bytecode,
            });
        //instruction_tests.insert(code.instruction, )
    }

    // eprintln!("tests {:?}", instruction_tests);
    let map = collect_instructions_from_samples(instruction_tests);

    let mut instructions = Vec::new();

    for line in lines.into_iter().skip(test_program_start) {
        let code = line.parse::<ByteCode>()?;

        let func = map.get(&code.instruction).unwrap();

        instructions.push((func)(code.a, code.b, code.c));
    }

    eprintln!("{:#?}", instructions.len());

    let mut machine = Machine::empty();

    for instr in instructions {
        machine.execute(instr).unwrap();
    }

    eprintln!("{:#?}", machine.registers());

    let result = machine.registers()[Reg(0)];

    eprintln!("part2 result {}", result);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let input = r"
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
        ";

        assert_eq!(3, part1(input.trim()).unwrap())
    }
}
