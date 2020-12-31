use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<(RuleSet, Vec<&str>)> {
    let mut parts = s.split("\n\n");
    let rule_set: RuleSet = parts.next().ok_or_else(|| anyhow!("missing rules"))?.parse()?;
    let strs: Vec<&str> = parts.next().ok_or_else(|| anyhow!("missing strings"))?.lines().collect();
    Ok((rule_set, strs))
}

fn part1((rule_set, strs): &(RuleSet, Vec<&str>)) {
    let count = strs.iter().filter(|s| rule_set.matches(s)).count();
    println!("part1 = {}", count);
}

fn part2((rule_set, strs): &(RuleSet, Vec<&str>)) {
    let mut rule_set = rule_set.clone();
    rule_set.rules.insert(8, "42 | 42 8".parse().unwrap());
    rule_set.rules.insert(11, "42 31 | 42 11 31".parse().unwrap());

    let count = strs.iter().filter(|s| rule_set.matches(s)).count();
    println!("part2 = {}", count);
}

#[derive(Clone)]
struct RuleSet {
    rules: HashMap<usize, Rule>
}

impl RuleSet {
    fn matches(&self, s: &str) -> bool {
        let rule_id = 0;
        self.matches_helper(s, rule_id, true).len() > 0
    }

    fn matches_helper(&self, s: &str, rule_id: usize, whole_str: bool) -> Vec<usize> {
        let rule = self.rules.get(&rule_id).unwrap();
        let mut res = vec![];
        for subrule in rule.subrules.iter() {
            for c in self.matches_helper_2(&subrule[..], s) {
                if !whole_str || c == s.len() {
                    res.push(c);
                }
            }
        }

        res
    }

    fn matches_helper_2(&self, subrule: &[RuleItem], s: &str) -> Vec<usize> {
        let mut res: Vec<usize> = vec![];
        if subrule.len() == 0 {
            res.push(0)
        }
        else if s.len() > 0 {
            let item = &subrule[0];
            match item {
                RuleItem::Literal(l) => {
                    if l == &s[0..l.len()] {
                        for c in self.matches_helper_2(&subrule[1..], &s[l.len()..]) {
                            res.push(c + l.len());
                        }
                    }
                },
                RuleItem::Rule(id) => {
                    for c in self.matches_helper(&s, *id, false) {
                        for d in self.matches_helper_2(&subrule[1..], &s[c..]) {
                            res.push(c+d);
                        }
                    }
                },
            }
        }

        res
    }
}

impl FromStr for RuleSet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let rules: HashMap<usize, Rule> = s.lines().map(|l| {
            let mut parts = l.split(':');
            let id: usize = parts.next().ok_or_else(|| anyhow!("missing rule id"))?.parse()?;
            let rule: Rule = parts.next().ok_or_else(|| anyhow!("missing rule body"))?.parse()?;
            Ok((id, rule))
        })
        .collect::<Result<_>>()?;
        Ok(RuleSet{ rules })
    }
}

#[derive(Clone)]
struct Rule {
    subrules: Vec<Vec<RuleItem>>
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let subrules = s.split('|').map(|subrule_str| subrule_str.trim().split(" ").map(|item_str| item_str.parse()).collect()).collect::<Result<_>>()?;
        Ok(Self { subrules })
    }
}

#[derive(Clone)]
enum RuleItem {
    Rule(usize),
    Literal(String),
}

impl FromStr for RuleItem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Some(literal) = s.split('"').nth(1) {
            Ok(RuleItem::Literal(literal.to_string()))
        }
        else {
            let id = s.parse()?;
            Ok(RuleItem::Rule(id))
        }
    }
}
