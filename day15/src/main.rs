use anyhow::{Context, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Vec<i64>> {
    s.trim().split(',').map(|n| n.parse().with_context(|| format!("could not parse '{}'", n))).collect()
}

fn part1(input: &Vec<i64>) {
    let mut game = MemoryGame::new(input);
    let n = game.run(2020);
    println!("part1 = {}", n);
}

fn part2(input: &Vec<i64>) {
    let mut game = MemoryGame::new(input);
    let n = game.run(30000000);
    println!("part2 = {}", n);
}

struct MemoryGame {
    spoken: HashMap<i64, i64>,
    last: i64,
    turn: i64,
}

impl MemoryGame {
    fn new(starting_nums: &[i64]) -> Self { 
        let turn = starting_nums.len() as i64;
        let (first_nums, last_num) = starting_nums.split_at(starting_nums.len() - 1);
        let spoken = first_nums.iter().map(|&n| n).zip(1..).collect();
        let last = last_num[0];
        Self { spoken, last, turn }
    }

    fn run(&mut self, to_turn: i64) -> i64 {
        while self.turn < to_turn {
            self.run_turn();
        }
        self.last
    }

    fn run_turn(&mut self) {
        let last_spoken_at = self.spoken.insert(self.last, self.turn).unwrap_or(self.turn);
        self.last = self.turn - last_spoken_at;
        self.turn += 1;
    }
}
