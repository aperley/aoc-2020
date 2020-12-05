use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Password>> {
    let r: std::result::Result<Vec<_>, _> = input.lines()
         .map(Password::parse)
         .collect();

    Ok(r?)
}

fn part1(input: &Vec<Password>) {
    let count = input.iter().filter(|p| p.is_valid_part1()).count();
    println!("part1 = {}", count);
}

fn part2(input: &Vec<Password>) {
    let count = input.iter().filter(|p| p.is_valid_part2()).count();
    println!("part2 = {}", count);
}

struct Password<'a> {
    password: &'a str, 
    policy: Policy,
}

struct Policy {
    letter: char,
    n1: usize,
    n2: usize,
}

impl<'a> Password<'a> {
    fn parse(s: &'a str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([\da]+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        }

        let caps = RE.captures(s).ok_or(anyhow!("regex failed to match '{}'", s))?;
        let n1 = caps.get(1).unwrap().as_str().parse()?;
        let n2 = caps.get(2).unwrap().as_str().parse()?;
        let letter = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = caps.get(4).unwrap().as_str();

        let policy = Policy { letter, n1, n2 };

        Ok(Password { password, policy })
    }

    fn is_valid_part1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.policy.letter).count();
        (self.policy.n1..=self.policy.n2).contains(&count)
    }

    fn is_valid_part2(&self) -> bool {
        let count = [self.policy.n1, self.policy.n2].iter()
            .filter(|&&n| self.password.chars().nth(n - 1).unwrap() == self.policy.letter)
            .count();
        count == 1
    }
}
