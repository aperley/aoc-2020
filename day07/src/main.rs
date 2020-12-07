use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use lazy_static::lazy_static;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let (parent_rules, child_rules) = parse(&input_str)?;

    part1(&parent_rules);
    part2(&child_rules);

    Ok(())
}

type Rules<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

fn parse(input: &str) -> Result<(Rules, Rules)> {
    let mut parent_rules: Rules = Default::default();

    for line in input.lines() {
        let (parent_kind, children) = parse_rule(line)?;
        for (count, child_kind) in children {
            parent_rules.entry(child_kind).or_insert(vec![])
                .push((count, parent_kind));
        }
    }

    let child_rules: Rules = input.lines()
        .map(parse_rule)
        .collect::<Result<Rules>>()?;

    Ok((parent_rules, child_rules))
}

fn part1(parent_rules: &Rules) {
    let kind = "shiny gold";
    let count = allowed_containers(kind, parent_rules).len();
    println!("part1 = {}", count);
}

fn allowed_containers<'a>(kind: &'a str, rules: &'a Rules) -> HashSet<&'a str> {
    let empty = vec![];
    let parents = rules.get(kind).unwrap_or(&empty);
    let recursive_iter = parents.iter()
        .map(|(_, parent_kind)| allowed_containers(parent_kind, rules))
        .flatten();
    
    parents.iter()
        .map(|&(_, parent_kind)| parent_kind)
        .chain(recursive_iter)
        .collect()
}

fn part2(child_rules: &Rules) {
    let kind = "shiny gold";
    let count = child_count(kind, child_rules);
    println!("part2 = {}", count);
}

fn child_count<'a>(kind: &'a str, rules: &'a Rules) -> usize {
    let children = rules.get(kind).unwrap();
    children.iter()
        .map(|(count, child_kind)| count * (1 + child_count(child_kind, rules)))
        .sum()
}


fn parse_rule(s: &str) -> Result<(&str, Vec<(usize, &str)>)> {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^(.*?) bags contain (.*?)\.$").unwrap();
        static ref CHILD_RE: Regex = Regex::new(r"^(\d+) (.*?) bags?$").unwrap();
    }

    let caps = RULE_RE.captures(s).ok_or_else(|| anyhow!("rule regex did not match '{}'", s))?;
    let kind = caps.get(1).unwrap().as_str();
    let children_str = caps.get(2).unwrap().as_str();

    let children = if children_str == "no other bags" {
        vec![]
    }
    else {
        children_str.split(", ").map(|child_str| {
            let caps = CHILD_RE.captures(child_str).ok_or_else(|| anyhow!("child regex did not match '{}'", child_str))?;
            let count: usize = caps.get(1).unwrap().as_str().parse()?;
            let kind = caps.get(2).unwrap().as_str();
            Ok((count, kind))
        })
        .collect::<Result<Vec<_>>>()?
    };

    Ok((kind, children))
}
