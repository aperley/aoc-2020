use anyhow::{Result};
use std::iter::Peekable;
use std::fmt;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Vec<Program>> {
    s.lines().map(tokenize).collect()
}

fn part1(input: &Vec<Program>) {
    let sum: Literal = input.iter()
        .map(|p| {
            let mut iter = p.iter().peekable();
            let ast = parse_expression(&mut iter);
            eval_ast(&ast)
        })
        .sum();

    println!("part1 = {}", sum);
}


type Token = char;
type Program = Vec<Token>;

fn tokenize(s: &str) -> Result<Program> {
    Ok(s.chars().filter(|&c| c != ' ').collect())
}

fn parse_primary<'a, I>(tokens: &mut Peekable<I>) -> Box<Primary>
where I: Iterator<Item = &'a Token>
{
    let c = tokens.next().unwrap();
    match c {
        '(' => parse_expression(tokens),
        c   => parse_literal(c),
    }
}


fn parse_expression<'a, I>(tokens: &mut Peekable<I>) -> Box<Primary>
where I: Iterator<Item = &'a Token>
{
    let lhs = parse_primary(tokens);
    let e = parse_expression_1(lhs, tokens, 0);
    if tokens.peek() == None || tokens.peek() == Some(&&')') { tokens.next(); }
    e
}

fn parse_expression_1<'a, I>(mut lhs: Box<Primary>, tokens: &mut Peekable<I>, min_precedence: usize) -> Box<Primary>
where I: Iterator<Item = &'a Token>
{
    loop {
        match tokens.peek() {
            None | Some(')') => {break;},
            Some(c) => {
                let op = parse_op(c);
                if op.precedence() < min_precedence { break; }
            }
        }

        let op = parse_op(tokens.next().unwrap());

        let mut rhs = parse_primary(tokens);
        loop {
            match tokens.peek() {
                None | Some(')') => { break; },
                Some(c) => {
                    let next_op = parse_op(c);
                    if next_op.precedence() <= op.precedence() { break; }
                    rhs = parse_expression_1(rhs, tokens, next_op.precedence());
                }
            }
        }

        lhs = Box::new(Primary::Expression { op, lhs, rhs });
    }

    lhs
}

fn parse_literal(c: &Token) -> Box<Primary> {
    let v = c.to_string().parse().unwrap();
    Box::new(Primary::Literal(v))
}


fn parse_op(c: &Token) -> Operation {
    match c {
        '+' => Operation::Add,
        '*' => Operation::Mul,
        _   => panic!("invalid op {}", c),
    }
}


enum Primary {
    Expression { op: Operation, lhs: Box<Primary>, rhs: Box<Primary> },
    Literal(Literal),
}

type Literal = i64;

enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn eval(&self, lhs: Literal, rhs: Literal) -> Literal {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
        }
    }

    fn precedence(&self) -> usize {
        match self {
            Operation::Add => 1,
            Operation::Mul => 0,
        }
    }
}


fn eval_ast(primary: &Box<Primary>) -> Literal {
    match &**primary {
        Primary::Expression { op, lhs, rhs } => {
            let lhs_val = eval_ast(&lhs);
            let rhs_val = eval_ast(&rhs);
            op.eval(lhs_val, rhs_val)
        },
        Primary::Literal(l) => *l,
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Operation::Add => '+',
            Operation::Mul => '*',
        };
        write!(f, "{}", c)
    }
}

fn _print_ast(primary: &Box<Primary>, level: usize) {
    let indent = "  ".repeat(level);
    match &**primary {
        Primary::Expression { op, lhs, rhs } => {
            println!("{}Op: {}", indent, op);
            println!("{}lhs:", indent);
            _print_ast(&lhs, level+1);
            println!("{}rhs:", indent);
            _print_ast(&rhs, level+1);
        },

        Primary::Literal(l) => {
            println!("{}{}", indent, l);
        }
    }

}
