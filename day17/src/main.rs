use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Vec<(i64, i64)>> {
    s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    match c {
                        '#' => Some(Ok((x as i64, y as i64))),
                        '.' => None,
                        c   => Some(Err(anyhow!("invalid character '{}' at ({}, {}", c, x, y))),
                    }
                })
        })
        .collect()
}

fn part1(input: &Vec<(i64, i64)>) {
    let mut dimension: PocketDimension<Point3> = input.iter().copied().collect();
    for _ in 0..6 {
        dimension.step();
    }

    let active_count = dimension.active.iter().count();
    println!("part1 = {}", active_count);
}

fn part2(input: &Vec<(i64, i64)>) {
    let mut dimension: PocketDimension<Point4> = input.iter().copied().collect();
    for _ in 0..6 {
        dimension.step();
    }

    let active_count = dimension.active.iter().count();
    println!("part2 = {}", active_count);
}

struct PocketDimension<T: Point>
{
    active: HashSet<T>,
}

impl<T: Point> PocketDimension<T>
{
    fn step(&mut self) {
        let mut active_neighbors: HashMap<T, (bool, usize)> = self.active.iter().map(|&p| (p, (true, 0))).collect();
        for point in self.active.iter() {
            for neighbor in point.neighbors() {
                let e = active_neighbors.entry(neighbor).or_insert((false, 0));
                let (active, count) = *e;
                *e = (active, count + 1);
            }
        }

        self.active = active_neighbors.iter().filter_map(|(&p, &(active, count))| {
            if (active && (count == 2 || count == 3)) || (!active && count == 3) {
                Some(p)
            }
            else {
                None
            }
        }).collect();
    }
}

impl<T: Point> FromIterator<(i64, i64)> for PocketDimension<T> {
    fn from_iter<I: IntoIterator<Item=(i64, i64)>>(iter: I) -> Self {
        let active = iter.into_iter().map(T::from_xy).collect();
        Self { active }
    }
}

trait Point: Copy + Eq + Hash + Sized {
    fn from_xy(xy: (i64, i64)) -> Self;
    fn neighbors(&self) -> Vec<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point for Point3 {
    fn from_xy((x, y): (i64, i64)) -> Self {
        Self { x, y, z: 0 }
    }

    fn neighbors(&self) -> Vec<Self> {
        (self.x-1..=self.x+1).flat_map(move |x| {
            (self.y-1..=self.y+1).flat_map(move |y| {
                (self.z-1..=self.z+1).map(move |z| Self { x, y, z })
            })
        })
        .filter(|p| p != self)
        .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point for Point4 {
    fn from_xy((x, y): (i64, i64)) -> Self {
        Self { x, y, z: 0, w: 0 }
    }

    fn neighbors(&self) -> Vec<Self> {
        (self.x-1..=self.x+1).flat_map(move |x| {
            (self.y-1..=self.y+1).flat_map(move |y| {
                (self.z-1..=self.z+1).flat_map(move |z| {
                    (self.w-1..=self.w+1).map(move |w| Self { x, y, z, w })
                })
            })
        })
        .filter(|p| p != self)
        .collect()
    }
}
