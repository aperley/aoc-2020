use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<i64>> {
    let r: std::result::Result<Vec<_>, _> = input.lines()
         .map(|l| l.parse())
         .collect();

    Ok(r?)
}

fn part1(vals: &Vec<i64>) {
    let val_set: HashSet<i64> = vals.iter().copied().collect();
    assert!(val_set.len() == vals.len());

    let mut pair = None;
    for &v in val_set.iter() {
        let partner = 2020 - v;
        if val_set.contains(&partner) {
            pair = Some((v, partner));
            break;
        }
    }

    let (x, y) = pair.unwrap();
    let answer = x * y;
    println!("part1 = {}", answer);
}

fn part2(vals: &Vec<i64>) {
    let val_set: HashSet<i64> = vals.iter().copied().collect();
    assert!(val_set.len() == vals.len());

    let mut trio = None;
    for &x in val_set.iter() {
        for &y in val_set.iter() {
            if x == y { continue; }

            let z = 2020 - x - y;
            if z != x && z != y && val_set.contains(&z) {
                trio = Some((x, y, z));
                break;
            }
        }
    }

    let (x, y, z) = trio.unwrap();
    let answer = x * y * z;
    println!("part2 = {}", answer);
}
