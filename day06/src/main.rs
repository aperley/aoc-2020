use anyhow::{Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Vec<&str>>> {
    let mut groups = vec![];
    let mut group = vec![];

    for person in input.lines() {
        if person == "" {
            groups.push(group);
            group = vec![];
        }
        else {
            group.push(person);
        }
    }
    groups.push(group);

    Ok(groups)
}

fn part1(input: &Vec<Vec<&str>>) {
    let yes_sum: usize = input.iter().map(|g| group_answers_any(g).len()).sum();
    println!("part1 = {}", yes_sum);
}

fn part2(input: &Vec<Vec<&str>>) {
    let all_yes_sum: usize = input.iter().map(|g| group_answers_all(g).len()).sum();
    println!("part2 = {}", all_yes_sum);
}

fn group_answers_any(group: &Vec<&str>) -> HashSet<char> {
    group.iter().flat_map(|p| p.chars()).collect()
}

fn group_answers_all(group: &Vec<&str>) -> HashSet<char> {
    let mut answers = group_answers_any(group);
    for person in group {
        let person_answers: HashSet<char> = person.chars().collect();
        answers = answers.intersection(&person_answers).map(|&c| c).collect();
    } 
    answers
}

