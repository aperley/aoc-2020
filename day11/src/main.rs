use anyhow::{anyhow, Result};
use std::str::FromStr;

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let input = parse(&input_str)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse(s: &str) -> Result<Map> {
    s.parse()
}

fn part1(input: &Map) {
    let mut map = input.clone();
    for _step in 0.. {
        if !map.step() { break; }
    }
    let occupied_count = map.iter().filter(|&(_, &e)| e == MapElement::Occupied).count();
    println!("part1 = {}", occupied_count);
}

fn part2(input: &Map) {
    let mut map = input.clone();
    for _step in 0.. {
        if !map.step_part2() { break; }
    }
    let occupied_count = map.iter().filter(|&(_, &e)| e == MapElement::Occupied).count();
    println!("part2 = {}", occupied_count);
}


#[derive(Clone)]
struct Map {
    rows: i64,
    cols: i64,
    maps: [Vec<MapElement>; 2],
    current_map: usize,
}

impl Map {
    fn step(&mut self) -> bool {
        use MapElement::*;

        let mut any_change: bool = false;
        let new_idx = (self.current_map + 1) % 2;
        for coord in self.iter_coords() {
            let idx = self.idx(&coord);
            let map = &self.maps[self.current_map];
            let current_element = map[idx];
            let new_element = match current_element {
                Floor => Floor,
                Empty => if self.adjacent(&coord).map(|c| map[self.idx(&c)]).any(|e| e == Occupied) { Empty } else { Occupied },
                Occupied => if self.adjacent(&coord).map(|c| map[self.idx(&c)]).filter(|&e| e == Occupied).count() >= 4 { Empty } else { Occupied },
            };

            if new_element != current_element {
                any_change = true;
            }

            let new_map = &mut self.maps[new_idx];
            new_map[idx] = new_element;
        }
        
        self.current_map = new_idx;
        any_change
    }

    fn step_part2(&mut self) -> bool {
        use MapElement::*;

        let mut any_change: bool = false;
        let new_idx = (self.current_map + 1) % 2;
        for coord in self.iter_coords() {
            let idx = self.idx(&coord);
            let map = &self.maps[self.current_map];
            let current_element = map[idx];
            let new_element = match current_element {
                Floor => Floor,
                Empty => if self.part2_adjacent_seats(&coord).any(|&e| e == Occupied) { Empty } else { Occupied },
                Occupied => if self.part2_adjacent_seats(&coord).filter(|&&e| e == Occupied).count() >= 5 { Empty } else { Occupied },
            };

            if new_element != current_element {
                any_change = true;
            }

            let new_map = &mut self.maps[new_idx];
            new_map[idx] = new_element;
        }
        
        self.current_map = new_idx;
        any_change
    }

    fn iter_coords(&self) -> impl Iterator<Item = (i64, i64)> {
        let rows = self.rows;
        let cols = self.cols;
        (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c)))
    }

    fn adjacent(&self, &(row, col): &(i64, i64)) -> impl Iterator<Item = (i64, i64)> {
        let rows = self.rows;
        let cols = self.cols;
        (row-1..=row+1).flat_map(move |r| (col-1..=col+1).map(move |c| (r, c)))
            .filter(move |&(r, c)| r >= 0 && r < rows && c >= 0 && c < cols && (r, c) != (row, col))
    }

    fn part2_adjacent_seats<'a>(&'a self, coord: &'a (i64, i64)) -> impl Iterator<Item = &'a MapElement> {
        (-1..=1).flat_map(move |dr| (-1..=1).map(move |dc| (dr, dc)))
            .filter(|&(r, c)| !(r == 0 && c == 0))
            .filter_map(move |(r, c)| self.seat_in_direction(coord, &(r, c)))
    }

    fn seat_in_direction(&self, coord: &(i64, i64), delta: &(i64, i64)) -> Option<&MapElement> {
        let mut row = coord.0 + delta.0;
        let mut col = coord.1 + delta.1;
        while row >= 0 && row < self.rows && col >= 0 && col < self.cols {
            let e = &self.maps[self.current_map][self.idx(&(row, col))];
            if *e != MapElement::Floor {
                return Some(e);
            }

            row += delta.0;
            col += delta.1;
        }

        None
    }

    fn iter(&self) -> impl Iterator<Item = ((i64, i64), &MapElement)> {
       self.iter_coords().map(move |c| (c, &self.maps[self.current_map][self.idx(&c)])) 
    }

    fn idx(&self, &(row, col): &(i64, i64)) -> usize {
        (row * self.cols + col) as usize
    }
}


impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let rows = s.lines().count() as i64;
        let cols = s.lines().nth(0).ok_or_else(|| anyhow!("map does not have any rows"))?.chars().count() as i64;
        let map0: Vec<MapElement> = s.lines().flat_map(|l| l.chars()).map(|c| c.parse()).collect::<Result<_, _>>()?;
        let map1 = map0.clone();
        let maps = [map0, map1];
        let current_map = 0;

        Ok(Self { rows, cols, maps, current_map })
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MapElement {
    Floor,
    Empty,
    Occupied,
}

impl FromChar for MapElement {
    type Err = anyhow::Error;

    fn from_char(c: &char) -> Result<Self> {
        use MapElement::*;
        match c {
            '.' => Ok(Floor),
            'L' => Ok(Empty),
            '#' => Ok(Occupied),
            _   => Err(anyhow!("unknown character '{}'", c)),
        }
    }
}



trait CharParseExt<T: FromChar> {
    fn parse(&self) -> std::result::Result<T, T::Err>;
}

impl<T: FromChar> CharParseExt<T> for char {
    fn parse(&self) -> std::result::Result<T, T::Err> {
        T::from_char(&self)
    }
}

trait FromChar: Sized {
    type Err;
    fn from_char(c: &char) -> std::result::Result<Self, Self::Err>;
}
