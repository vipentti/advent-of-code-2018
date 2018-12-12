use aoc::{read_input, CustomError, Result};
use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::*;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
    id: i32,
}

impl Point {
    fn distance_from(&self, other: &Self) -> usize {
        manhattan_distance(self, other)
    }
}

impl FromStr for Point {
    type Err = CustomError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),\s*(\d+)").unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| CustomError("Invalid captures".to_owned()))?;

        let x = aoc::get_value(&caps, 1)?;
        let y = aoc::get_value(&caps, 2)?;

        Ok(Point { x, y, id: 0 })
    }
}

impl std::convert::From<(usize, usize)> for Point {
    fn from(v: (usize, usize)) -> Self {
        Point {
            x: v.0 as i32,
            y: v.1 as i32,
            id: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Cell {
    last_distance: usize,
    visits: Vec<i32>,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            last_distance: usize::max_value(),
            visits: Vec::new(),
        }
    }

    pub fn id(&self) -> Option<i32> {
        if self.visits.len() == 1 {
            Some(self.visits[0])
        } else {
            None
        }
    }

    pub fn value(&self) -> String {
        if self.visits.len() == 1 {
            self.visits[0].to_string()
        } else {
            '.'.to_string()
        }
    }

    fn add(&mut self, id: i32) {
        self.visits.push(id);
    }

    fn set(&mut self, id: i32) {
        self.visits.clear();
        self.visits.push(id);
    }
}

type Grid<T> = Vec<Vec<T>>;

fn main() -> Result<()> {
    let s = read_input()?;

    part1(&s)?;
    part2(&s, 10000)?;

    Ok(())
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    ((b.x - a.x).abs() + (b.y - a.y).abs()) as usize
}

fn get_points(s: &str) -> Result<(Vec<Point>, usize, usize)> {
    let coords: std::result::Result<Vec<_>, _> =
        s.lines().map(|v| v.parse::<Point>()).collect();

    let mut coords = coords?;

    let min_x = coords
        .iter()
        .map(|c| c.x)
        .min()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing min_x".to_string()).into()
        })?;
    let max_x = coords
        .iter()
        .map(|c| c.x)
        .max()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing max_x".to_string()).into()
        })?;
    let min_y = coords
        .iter()
        .map(|c| c.y)
        .min()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing min_y".to_string()).into()
        })?;
    let max_y = coords
        .iter()
        .map(|c| c.y)
        .max()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing max_y".to_string()).into()
        })?;

    let size_x = (max_x - min_x).abs() as usize + 1;
    let size_y = (max_y - min_y).abs() as usize + 1;

    for (i, mut co) in coords.iter_mut().enumerate() {
        co.x -= min_x;
        co.y -= min_y;
        co.id = i as i32;
    }

    Ok((coords, size_x, size_y))
}

fn part1(s: &str) -> Result<usize> {
    let (coords, size_x, size_y) = get_points(s)?;

    let mut grid = vec![vec![Cell::new(); size_x]; size_y];

    update_grid(&mut grid, &coords)?;

    let r = count_grid1(&grid, (size_x - 1) as usize, (size_y - 1) as usize)?;

    eprintln!("part1: {:?}", r);

    Ok(r)
}

fn part2(s: &str, limit: usize) -> Result<usize> {
    let (coords, size_x, size_y) = get_points(s)?;

    let grid = vec![vec![Cell::new(); size_x]; size_y];

    let res = count_grid2(&grid, &coords, limit)?;

    eprintln!("part2: {:?}", res);

    Ok(res)
}

fn update_grid(grid: &mut Grid<Cell>, coords: &[Point]) -> Result<()> {
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, mut col) in row.iter_mut().enumerate() {
            let pt: Point = (x, y).into();
            for coord in coords.iter() {
                let dist = manhattan_distance(&pt, &coord) as usize;
                if dist == 0 {
                    col.last_distance = dist;
                    col.set(coord.id);
                } else if dist == col.last_distance {
                    col.last_distance = dist;
                    col.add(coord.id);
                } else if dist < col.last_distance {
                    col.last_distance = dist;
                    col.set(coord.id);
                }
            }
        }
    }

    Ok(())
}

fn count_grid2<T>(grid: &[T], coords: &[Point], limit: usize) -> Result<usize>
where
    T: AsRef<[Cell]>,
{
    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.as_ref().iter().enumerate() {
            let pt: Point = (x, y).into();

            let all: usize = coords.iter().map(|c| pt.distance_from(c)).sum();

            if all < limit {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn count_grid1<T>(grid: &[T], max_x: usize, max_y: usize) -> Result<usize>
where
    T: AsRef<[Cell]>,
{
    let mut map = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.as_ref().iter().enumerate() {
            if let Some(id) = col.id() {
                if x == 0 || y == 0 || x == max_x || y == max_y {
                    *map.entry(id).or_insert_with(|| 0) = i32::min_value();
                } else {
                    let entry = map.entry(id).or_insert_with(|| 0);

                    if let Some(res) = entry.checked_add(1) {
                        *entry = res;
                    }
                }
            }
        }
    }

    let result = map
        .iter()
        .filter(|(_, v)| **v >= 0)
        .map(|(_, v)| v)
        .max()
        .unwrap();

    Ok(*result as usize)
}

#[allow(dead_code)]
fn display_grid(grid: &[Vec<Cell>]) -> Result<()> {
    for row in grid.iter() {
        let mut s = String::new();
        for col in row.iter() {
            s.push_str(&col.value());
        }

        eprintln!("{}", s);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
    ";

    #[test]
    fn part1_example_input() {
        assert_eq!(17, part1(INPUT.trim()).unwrap());
    }

    #[test]
    fn part2_example_input() {
        assert_eq!(16, part2(INPUT.trim(), 32).unwrap());
    }
}
