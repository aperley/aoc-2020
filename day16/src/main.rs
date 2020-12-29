use anyhow::{anyhow, Context, Result};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Notes> {
    s.parse()
}

fn part1(notes: &Notes) {
    let invalid_sum: i64 = notes.nearby.iter()
        .flat_map(|ticket| {
            ticket.iter().filter(|value| !notes.rules.iter().any(|rule| rule.is_valid(value)))
        })
        .sum();
    
    println!("part1 = {}", invalid_sum);
}


fn part2(notes: &Notes) {
    let valid_tickets: Vec<&Ticket> = notes.nearby.iter()
        .filter(|ticket| {
            ticket.iter().all(|value| notes.rules.iter().any(|rule| rule.is_valid(value)))
        })
        .collect();
    
    let field_count = notes.rules.len();
    assert_eq!(field_count, notes.ticket.len());

    // valid[rule][field] = true if the rule is valid for that field
    let mut valid = vec![vec![true; field_count]; field_count];
    for ticket in valid_tickets.iter() {
        for (field_idx, value) in ticket.iter().enumerate() {
            for (rule_idx, rule) in notes.rules.iter().enumerate() {
                valid[rule_idx][field_idx] &= rule.is_valid(value);
            }
        }
    }

    // map rule_idx -> field_idx
    let mut mapping = HashMap::new();
    assert!(find_idx_mapping(&mut (0..field_count).collect(), &mut (0..field_count).collect(), &valid, &mut mapping));

    // sanity check
    for ticket in valid_tickets.iter() {
        for (rule_idx, rule) in notes.rules.iter().enumerate() {
            let field_idx = *mapping.get(&rule_idx).unwrap();
            assert!(rule.is_valid(&ticket[field_idx]));
        }
    }

    let product: i64 = notes.rules.iter()
        .enumerate()
        .filter(|(_, r)| r.name.starts_with("departure"))
        .map(|(i, r)| {
            let field_idx = *mapping.get(&i).unwrap() as usize;
            assert!(r.is_valid(&notes.ticket[field_idx]));
            notes.ticket[field_idx]
        })
        .product();
    
    println!("part2 = {}", product);

}

fn find_idx_mapping(rules: &mut HashSet<usize>, fields: &mut HashSet<usize>, valid: &Vec<Vec<bool>>, mapping: &mut HashMap<usize, usize>) -> bool {
    if rules.len() == 0 {
        return true;
    }

    // find most constrained rule
    let (rule, valid_fields) = rules.iter()
        .map(|&rule| {
            (rule, fields.iter().filter(|&&field| valid[rule][field]).map(|&f| f).collect::<Vec<usize>>())
        })
        .min_by_key(|(_, v)| v.len()).unwrap();
    
    // no valid fields for this rule, backtrack
    if valid_fields.len() == 0 {
        return false;
    }

    rules.remove(&rule);
    for field in valid_fields {
        fields.remove(&field);
        if find_idx_mapping(rules, fields, valid, mapping) {
            mapping.insert(rule, field);
            return true;
        }

        fields.insert(field);
    }

    rules.insert(rule);
    return false; // backtrack
}

struct Notes {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby: Vec<Ticket>,
}

impl FromStr for Notes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let rules = lines.by_ref().take_while(|&l| l != "")
            .map(|l| l.parse().with_context(|| format!("error parsing '{}'", l)))
            .collect::<Result<_>>()?;
        
        expect_line("your ticket:", &mut lines)?; 

        let ticket = lines.next().ok_or_else(|| anyhow!("expected ticket but got EOF"))
            .and_then(|l| parse_ticket(l).with_context(|| format!("error parsing '{}'", l)))?;
        
        expect_line("", &mut lines)?;
        expect_line("nearby tickets:", &mut lines)?;
        
        let nearby = lines.map(|l| parse_ticket(l).with_context(|| format!("error parsing '{}", l))).collect::<Result<_>>()?;

        Ok(Self { rules, ticket, nearby })
    }
}

type Ticket = Vec<i64>;

fn parse_ticket(s: &str) -> Result<Ticket> {
    let ticket = s.split(',').map(|n| n.parse()).collect::<Result<_, _>>()?;
    Ok(ticket)
}

fn expect_line(e: &str, lines: &mut std::str::Lines) -> Result<()> {
    lines.next().ok_or_else(|| "EOF")
        .and_then(|l| if l == e { Ok(()) } else { Err(l) })
        .map_err(|s| anyhow!("expected '{}' but got '{}'", e, s))
}

struct Rule {
    name: String,
    ranges: Vec<(i64, i64)>,
}

impl Rule {
    fn is_valid(&self, value: &i64) -> bool {
        self.ranges.iter().any(|&(low, high)| (low..=high).contains(value))
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"^(.*?): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }

        let caps = RULE_RE.captures(s).ok_or_else(|| anyhow!("rule did not match expected format"))?;
        let name = caps.get(1).unwrap().as_str().to_owned();

        let r0_low = caps.get(2).unwrap().as_str().parse()?;
        let r0_high = caps.get(3).unwrap().as_str().parse()?;
        let r1_low = caps.get(4).unwrap().as_str().parse()?;
        let r1_high = caps.get(5).unwrap().as_str().parse()?;
        let ranges = vec![(r0_low, r0_high), (r1_low, r1_high)];

        Ok(Self { name, ranges })
    }
}

