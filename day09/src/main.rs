use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    let part1_answer = part1(&input)?;
    part2(&input, part1_answer);

    Ok(())
}

fn parse(s: &str) -> Result<Vec<i64>> {
    s.lines()
        .enumerate()
        .map(|(line, l)| l.parse().map_err(|e| anyhow!("could not parse line {} '{}': {}", line + 1, l, e)))
        .collect()
}

fn part1(input: &Vec<i64>) -> Result<i64> {
    let preamble_len = 25;

    let mut iter = input.iter().enumerate().map(|(i, &v)| (i, v));
    let mut prev: RefCountSet<i64> = iter.by_ref().take(preamble_len).map(|(_, v)| v).collect();

    for (i, v) in iter {
        // is valid
        let mut valid = false;
        for &prev_v in prev.count.keys() {
            let diff = v - prev_v;
            if diff != prev_v && prev.contains(&diff) {
                valid = true;
            }
        }

        if !valid {
            println!("part1 = {}", v);
            return Ok(v);
        }
        
        let remove = input[i-preamble_len];
        prev.remove(&remove);
        prev.add(v);
    }

    Err(anyhow!("did not find valid part1 answer"))
}

fn part2(input: &Vec<i64>, part1_answer: i64) {
    for start_index in 0..input.len() {
        for end_index in (start_index+2)..input.len() {
            let sum: i64 = input[start_index..end_index].iter().sum();
            if sum > part1_answer {
                break;
            }

            if sum == part1_answer {
                let min = input[start_index..end_index].iter().min().unwrap();
                let max = input[start_index..end_index].iter().max().unwrap();
                println!("part2 = {}", min + max);
            }
        }
    }
}


struct RefCountSet<T>
    where T: Eq + Hash
{
    count: HashMap<T, usize>,
}

impl<T> RefCountSet<T>
    where T: Eq + Hash
{
    fn new() -> Self {
        RefCountSet{ count: HashMap::new() }
    }

    fn add(&mut self, elem: T) {
        let c = self.count.entry(elem).or_insert(0);
        *c += 1;
    }

    fn remove(&mut self, elem: &T) {
        let c = self.count.get_mut(elem).unwrap();
        *c -= 1;
        if *c == 0 {
            self.count.remove(elem);
        }
    }

    fn contains(&self, elem: &T) -> bool {
        self.count.contains_key(elem)
    }
}

impl<T> FromIterator<T> for RefCountSet<T>
    where T: Eq + Hash
{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = Self::new();
        for i in iter {
            c.add(i);
        }
        c
    }
}
