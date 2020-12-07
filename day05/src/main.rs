use std::str::FromStr;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Seat>> {
    input.lines().map(|p| p.parse()).collect()
}

fn part1(input: &Vec<Seat>) {
    let highest = input.iter().map(|s| s.id()).max().unwrap();
    println!("part1 = {}", highest);
}

fn part2(input: &Vec<Seat>) {
    let mut ids: Vec<usize> = input.iter().map(|s| s.id()).collect();
    ids.sort();

    let missing = ids.iter()
        .zip(ids.iter().skip(1))
        .find_map(|(&id, &next_id)| {
            if id + 1 == next_id {
                None
            }
            else {
                Some(id + 1)
            }
        })
        .expect("no missing id");
    
    println!("part2 = {}", missing);
}

struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let row = binary_partition(&s[0..7], 0, 128)?;
        let col = binary_partition(&s[7..7+3], 0, 8)?;
        Ok(Seat { row, col })
    }
}


fn binary_partition(steps: &str, mut low: usize, mut high: usize) -> Result<usize> {
    for c in steps.chars() {
        let mid = (low + high) / 2;
        match c {
            'F' | 'L' => high = mid,
            'B' | 'R' => low = mid,
            _   => return Err(anyhow!("invalid step '{}'", c)),
        };
    }

    if low + 1 == high {
        Ok(low)
    }
    else {
        Err(anyhow!("steps do not converge to a single value {} {} {}", low, high, steps))
    }
}
