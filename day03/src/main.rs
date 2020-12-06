use std::str::FromStr;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(input: &str) -> Result<Map> {
    input.parse()
}

fn part1(input: &Map) {
    let slope = (3, 1);
    let count = count_trees(&slope, input);
    println!("part1 = {}", count);
}

fn part2(input: &Map) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let count: usize = slopes.iter().map(|s| count_trees(s, input)).product();
    println!("part2 = {}", count);
}

fn count_trees(slope: &(usize, usize), map: &Map) -> usize {
    let loc_iter = (0..map.rows).step_by(slope.1).zip((0..).step_by(slope.0));
    loc_iter.filter(|&(row, col)| map.tree_at(row, col)).count()
}

struct Map {
    rows: usize,
    cols: usize,
    trees: Vec<bool>,
}

impl Map {
    fn tree_at(&self, row: usize, col: usize) -> bool {
        let col = col % self.cols;
        self.trees[row * self.cols + col]
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let rows = s.lines().count();
        let cols = s.lines().next().ok_or(anyhow!("got 0 lines"))?.chars().count();
        let trees = s.lines()
            .flat_map(|l| l.chars())
            .map(|c| {
                match c {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _   => Err(anyhow!("invalid symbol {}", c)),
                }
            })
            .collect::<std::result::Result<_, _>>()?;
        
        Ok(Map { rows, cols, trees })
    }

}
