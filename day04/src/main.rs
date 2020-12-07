use anyhow::{anyhow, Result};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    part1(&input_str)?;
    part2(&input_str)?;
    
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut valid_count = 0;

    let mut p = PassportBuilder::new();
    for word in input.split(&[' ', '\n'][..]) {
        if word == "" {
            if p.is_valid() {
                valid_count += 1;
            }

            p = PassportBuilder::new();
        }
        else {
            p.add(word)?;
        }
    }

    println!("part1 = {}", valid_count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut valid_count = 0;

    let mut p = PassportBuilder::new();
    for word in input.split(&[' ', '\n'][..]) {
        if word == "" {
            if p.is_valid_part2() {
                valid_count += 1;
            }

            p = PassportBuilder::new();
        }
        else {
            p.add(word)?;
        }
    }

    println!("part2 = {}", valid_count);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl Field {
    fn is_valid(&self, value: &str) -> bool {
        use Field::*;
        match self {
            Byr => value.parse::<i64>().map_or(false, |v| (1920..=2002).contains(&v)),
            Iyr => value.parse::<i64>().map_or(false, |v| (2010..=2020).contains(&v)),
            Eyr => value.parse::<i64>().map_or(false, |v| (2020..=2030).contains(&v)),
            Hgt => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
                }
                RE.captures(value).map_or(false, |caps| {
                    let bounds = match caps.get(2).unwrap().as_str() {
                        "cm" => 150..=193,
                        "in" => 59..=76,
                        _ => panic!("invalid unit"),
                    };
                    let num = caps.get(1).unwrap().as_str().parse().unwrap();
                    bounds.contains(&num)
                })
            },
            Hcl => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
                }
                RE.is_match(value)
            },
            Ecl => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|&s| s == value),
            Pid => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
                }
                RE.is_match(value)
            },
            Cid => true,
        }
    }
}

struct PassportBuilder<'a> {
    fields: HashMap<Field, &'a str>,
}

impl<'a> PassportBuilder<'a> {
    fn new() -> Self {
        let fields: HashMap<_, _> = Default::default();
        PassportBuilder { fields }
    }
    
    fn add(&mut self, word: &'a str) -> Result<()> {
        let mut parts = word.split(':');
        let field = parts.next().ok_or(anyhow!("invalid format '{}'", word))?;
        let value = parts.next().ok_or(anyhow!("invalid format '{}'", word))?;

        use Field::*;
        let field = match field {
            "byr" => Byr,
            "iyr" => Iyr,
            "eyr" => Eyr,
            "hgt" => Hgt,
            "hcl" => Hcl,
            "ecl" => Ecl,
            "pid" => Pid,
            "cid" => Cid,
            _ => return Err(anyhow!("invalid field '{}'", field)),
        };

        self.fields.insert(field, value);
        Ok(())
    }

    fn is_valid(&self) -> bool {
        use Field::*;
        let required_fields = [Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid];
        required_fields.iter().all(|f| self.fields.contains_key(f))
    }

    fn is_valid_part2(&self) -> bool {
        use Field::*;
        let required_fields = [Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid];
        required_fields.iter().all(|f| self.fields.get(f).map_or(false, |v| f.is_valid(v)))
    }
}
