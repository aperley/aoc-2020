use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(s: &str) -> Result<Vec<Instruction>> {
    s.lines().map(|l| l.parse().with_context(|| format!("could not parse '{}'", l))).collect()
}

fn part1(input: &Vec<Instruction>) -> Result<()> {
    let mut vm: VM = Default::default();
    vm.run(input)?;
    let sum: u64 = vm.mem.values().sum();
    println!("part1 = {}", sum);
    Ok(())
}

fn part2(input: &Vec<Instruction>) -> Result<()> {
    let mut vm: VMPart2 = Default::default();
    vm.run(input)?;
    let sum: u64 = vm.mem.values().sum();
    println!("part2 = {}", sum);
    Ok(())
}

#[derive(Default)]
struct VM {
    mask: Option<Bitmask>,
    mem: HashMap<usize, u64>,
}

impl VM {
    fn run(&mut self, program: &Vec<Instruction>) -> Result<()> {
        for instruction in program.iter() {
            self.execute(instruction)?;
        }
        Ok(())
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        match *instruction {
            Instruction::Mask { mask } => self.mask = Some(mask),
            Instruction::Mem { addr, value } => {
                let mask = self.mask.ok_or_else(|| anyhow!("got a mem instruction before a mask instruction"))?;
                let calc_val = (value & !mask.mask) | (mask.value & mask.mask);
                self.mem.insert(addr, calc_val);
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct VMPart2 {
    mask: Option<Vec<Bitmask>>,
    mem: HashMap<usize, u64>,
}

impl VMPart2 {
    fn run(&mut self, program: &Vec<Instruction>) -> Result<()> {
        for instruction in program.iter() {
            self.execute(instruction)?;
        }
        Ok(())
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        match *instruction {
            Instruction::Mask { mask } => self.mask = Some(all_masks(&mask)),
            Instruction::Mem { addr, value } => {
                let masks = self.mask.as_ref().ok_or_else(|| anyhow!("got a mem instruction before a mask instruction"))?;
                for mask in masks {
                    let calc_addr = (addr & !(mask.mask as usize)) | ((mask.value & mask.mask) as usize);
                    self.mem.insert(calc_addr, value);

                }
            }
        }
        Ok(())
    }
}

fn all_masks(mask: &Bitmask) -> Vec<Bitmask> {
    let mut masks = vec![];
    all_masks_helper(*mask, 0, &mut masks);
    masks
}

fn all_masks_helper(mut mask: Bitmask, start_bit: u64, masks: &mut Vec<Bitmask>) {
    let mut any_floating = false;
    for bit in start_bit..36 {
        if (mask.mask >> bit) & 1 == 0 {
            any_floating = true;
            let new_mask = mask.mask | (1 << bit);
            for bit_value in 0..=1 {
                let new_value = mask.value | (bit_value << bit);
                all_masks_helper(Bitmask { mask: new_mask, value: new_value }, bit + 1, masks);
            }
            break
        }
        else if (mask.value >> bit) & 1 == 0 {
            mask.mask &= !(1 << bit);
        }
    }

    if !any_floating {
        masks.push(mask);
    }
}

#[derive(Clone, Copy)]
struct Bitmask {
    mask: u64,
    value: u64,
}

enum Instruction {
    Mask { mask: Bitmask },
    Mem { addr: usize, value: u64 },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
            static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }

        if let Some(caps) = MASK_RE.captures(s) {
            let mut value = 0;
            let mut mask = 0;
            for c in caps.get(1).unwrap().as_str().chars() {
                value <<= 1;
                mask <<= 1;
                match c {
                    '0' => {
                        value |= 0;
                        mask |= 1;
                    },
                    '1' => {
                        value |= 1;
                        mask |= 1;
                    },
                    'X' => (),
                    _ => panic!("invalid character in mask"),
                }
            }

            Ok(Instruction::Mask { mask: Bitmask { mask, value } })
        }
        else if let Some(caps) = MEM_RE.captures(s) {
            let addr = caps.get(1).unwrap().as_str().parse()?;
            let value = caps.get(2).unwrap().as_str().parse()?;
            Ok(Instruction::Mem { addr, value })
        }
        else {
            Err(anyhow!("invalid instruction"))
        }
    }
}
