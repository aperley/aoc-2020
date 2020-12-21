use anyhow::{anyhow, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Vec<i64>> {
    s.lines()
        .enumerate()
        .map(|(line, l)| l.parse().map_err(|e| anyhow!("could not parse line {} '{}': {}", line + 1, l, e)))
        .collect()
}

fn part1(input: &Vec<i64>) {
    let joltages = sorted_joltages(input);

    let mut diff1_count = 0;
    let mut diff3_count = 0;
    for i in 0..(joltages.len()-1) {
        let diff = joltages[i+1] - joltages[i];
        match diff {
            1 => diff1_count += 1,
            3 => diff3_count += 1,
            _ => panic!("invalid difference: {}", diff),
        }
    }

    println!("part1 = {}", diff1_count * diff3_count);
}

fn part2(input: &Vec<i64>) {
    let joltages = sorted_joltages(input);

    let mut cache = HashMap::new();
    let count = count_ways(&joltages, &mut cache);
    println!("part2 = {}", count);
}

fn count_ways(adapters: &[i64], cache: &mut HashMap<i64, usize>) -> usize {
    if adapters.len() == 1 {
        return 1;
    }

    let current = adapters[0];
    if let Some(&count) = cache.get(&current) {
        return count;
    }

    let mut count = 0;
    for i in 1..adapters.len() {
        let next_adapter = adapters[i];
        if next_adapter > current + 3 {
            break;
        }

        count += count_ways(&adapters[i..], cache);
    }

    cache.insert(current, count);
    count
}

fn sorted_joltages(input: &Vec<i64>) -> Vec<i64> {
    let mut joltages = vec![0];
    joltages.extend(input);
    joltages.sort();
    joltages.push(joltages[..].last().unwrap() + 3);
    joltages
}
