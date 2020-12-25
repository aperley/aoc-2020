use anyhow::{anyhow, Context, Result};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};
use std::str::FromStr;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Vec<Instruction>> {
    s.lines().map(|l| l.parse().with_context(|| format!("failed to parse instruction '{}'", l))).collect()
}

fn part1(input: &Vec<Instruction>) {
    let mut ship: Ship = Default::default();
    for instruction in input {
        ship.step(instruction);
    }
    println!("part1 = {}", ship.position.manhattan_magnitude());
}

fn part2(input: &Vec<Instruction>) {
    let mut ship: ShipPart2 = Default::default();
    for instruction in input {
        ship.step(instruction);
    }
    println!("part1 = {}", ship.position.manhattan_magnitude());
}

struct Ship {
    position: Vec2<i64>,
    orientation: Vec2<i64>,
}

impl Ship {
    fn step(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::MoveDirection(v) => self.position += *v,
            Instruction::Turn(v) => self.orientation.turn(*v),
            Instruction::MoveForward(v) => self.position += self.orientation * *v,
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            position: Vec2::new(0, 0),
            orientation: Vec2::new(1, 0), // East
        }
    }
}

struct ShipPart2 {
    position: Vec2<i64>,
    waypoint: Vec2<i64>,
}

impl ShipPart2 {
    fn step(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::MoveDirection(v) => self.waypoint += *v,
            Instruction::Turn(v) => self.waypoint.turn(*v),
            Instruction::MoveForward(v) => self.position += self.waypoint * *v,
        }
    }
}

impl Default for ShipPart2 {
    fn default() -> Self {
        ShipPart2 {
            position: Vec2::new(0, 0),
            waypoint: Vec2::new(10, 1),
        }
    }
}

enum Instruction {
    MoveDirection(Vec2<i64>),
    Turn(i64),
    MoveForward(i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        use Instruction::*;
        if s.len() < 1 { return Err(anyhow!("not long enough")); }
        let (action_str, value_str) = s.split_at(1);
        let action = action_str.chars().nth(0).ok_or_else(|| anyhow!("missing action character"))?;
        let value: i64 = value_str.parse().map_err(|e| anyhow!("could not parse value '{}': {}", value_str, e))?;

        Ok(match action {
            'N' => MoveDirection(Vec2::new(0, value)),
            'S' => MoveDirection(Vec2::new(0, -value)),
            'E' => MoveDirection(Vec2::new(value, 0)),
            'W' => MoveDirection(Vec2::new(-value, 0)),

            'L' | 'R' if value % 90 != 0 => return Err(anyhow!("turn not a multiple of 90: {}", value)),
            'L' => Turn(value / 90),
            'R' => Turn(-value / 90),

            'F' => MoveForward(value),

            s => return Err(anyhow!("invalid action '{}'", s)),
        })
    }
}





#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y, 
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Neg<Output = T> + Copy> Vec2<T> {
    fn turn(&mut self, steps: i64) {
        let positive = steps >= 0;
        let s = steps.abs() % 4;

        for _ in 0..s {
            let (x, y) = (self.x, self.y);
            if positive {
                self.x = y.neg();
                self.y = x;
            }
            else {
                self.x = y;
                self.y = x.neg();
            }
        }
    }
}

impl Vec2<i64> {
    fn manhattan_magnitude(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}
