use anyhow::{anyhow, Result};
use std::str::FromStr;


fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Vec<Instruction>> {
    s.lines().map(|l| l.parse()).collect()
}

fn part1(input: &Vec<Instruction>) {
    let mut vm: VM = Default::default();
    vm.run(input);
    println!("part1 = {}", vm.accumulator)
}

fn part2(input: &Vec<Instruction>) {
    use Instruction::*;
    for i in 0..input.len() {
        let mut program = (*input).clone();
        let new_instruction = match program[i] {
            Acc { .. } => continue,
            Jmp { offset } => Nop { value: offset },
            Nop { value } => Jmp { offset: value },
        };
        program[i] = new_instruction;

        let mut vm: VM = Default::default();
        if vm.run(&program) {
            println!("part2 = {}", vm.accumulator);
        }
    }
}

#[derive(Default)]
struct VM {
    pc: i64,
    accumulator: i64,
}

impl VM {
    fn run(&mut self, program: &Vec<Instruction>) -> bool {
        let mut seen: Vec<bool> = program.iter().map(|_| false).collect();

        while (0..(program.len() as i64)).contains(&self.pc) && !seen[self.pc as usize] {
            seen[self.pc as usize] = true;
            self.execute(&program[self.pc as usize]);
        }

        self.pc == (program.len() as i64)
    }

    fn execute(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Acc { value } => {
                self.accumulator += value;
                self.pc += 1
            },
            Jmp { offset } => self.pc += offset,
            Nop { .. } => self.pc += 1,
        }
    }

}

#[derive(Clone)]
enum Instruction {
    Acc { value: i64 },
    Jmp { offset: i64 },
    Nop { value: i64 },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        use Instruction::*;

        let mut parts = s.split_whitespace();
        let opcode = parts.next().ok_or(anyhow!("missing opcode"))?;
        let arg = parts.next().ok_or(anyhow!("missing arg"))?.parse()?;

        match opcode {
            "acc" => Ok( Acc { value: arg } ),
            "jmp" => Ok( Jmp { offset: arg } ),
            "nop" => Ok( Nop { value: arg } ),
            _ => Err(anyhow!("invalid opcode {}", opcode)),
        }
    }
}
