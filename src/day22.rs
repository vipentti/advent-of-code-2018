#![allow(dead_code)]
use aoc::{Result, Vector2, CustomError, ToIndex};

use std::io::{self, Write};

use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::fmt;
use std::collections::{BTreeSet, BTreeMap};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    // part2(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<usize> {
    let mut depth = i32::min_value();
    let mut target_x = i32::min_value();
    let mut target_y = i32::min_value();

    for (ind, line) in s.lines().enumerate() {
        if ind == 0 {
            let input = line.replace("depth: ", "");
            depth = input.parse::<i32>()?;
        } else if ind == 1 {
            let input = line.replace("target: ", "");
            let parts: Vec<&str> = input.split(",")
                        .map(|s| s.trim())
                        .collect();

            target_x = parts[0].parse::<i32>()?;
            target_y = parts[1].parse::<i32>()?;
        }
    }

    let target: Vector2 = (target_x, target_y).into();


    let mut grid = Grid::new(target.x as usize + 1, target.y as usize + 1, depth as usize);

    let risk = grid.calculate(target);

    grid.display();

    eprintln!("part1: Target {} @ {} - Risk {}", target, depth, risk);
    Ok(0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Tile {
    Rocky,
    Wet,
    Narrow,
    Mouth,
    Target
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Rocky => '.',
            Tile::Wet => '=',
            Tile::Narrow => '|',
            Tile::Mouth => 'M',
            Tile::Target => 'T',
        }
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Rocky }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Grid {
    data: Vec<Tile>,
    geologic: Vec<usize>,
    erosion: Vec<usize>,
    width: usize,
    height: usize,
    depth: usize,
}

impl Grid {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        Grid {
            data: vec![Default::default(); width * height],
            geologic: vec![Default::default(); width * height],
            erosion: vec![Default::default(); width * height],
            width,
            height,
            depth,
        }
    }

    fn calculate(&mut self, target: Vector2) -> usize {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (x, y).to_index(self.width);
                let g_index = {
                    if (x == 0 && y == 0)
                        || target == (x, y) {
                        0
                    } else if y == 0 {
                        x * 16807
                    } else if x == 0 {
                        y * 48271
                    } else {
                        let e1 = (x - 1, y).to_index(self.width);
                        let e2 = (x, y - 1).to_index(self.width);

                        self.erosion[e1] * self.erosion[e2]
                    }
                };
                let erosion = {
                    (g_index + self.depth) % 20183
                };

                self.geologic[index] = g_index;
                self.erosion[index] = erosion;

                let tp = {
                    if erosion % 3 == 0 {
                        Tile::Rocky
                    } else if erosion % 3 == 1 {
                        Tile::Wet
                    } else {
                        Tile::Narrow
                    }
                };
                self.data[index] = tp;
            }
        }

        self.data[0] = Tile::Mouth;
        self.data[target.to_index(self.width)] = Tile::Target;

        let mut risk_level = 0;

        for y in 0..=target.y {
            for x in 0..=target.x {
                let index = (x, y).to_index(self.width);

                let risk = match self.data[index] {
                    Tile::Rocky |
                    Tile::Mouth |
                    Tile::Target => {
                        0
                    },
                    Tile::Wet => {
                        1
                    },
                    Tile::Narrow => {
                        2
                    },
                };

                risk_level += risk;

            }
        }

        risk_level
    }

    fn display(&self) {
        eprintln!("{}", self.render_to_string());
    }

    fn render_to_string(&self) -> String {
        assert!(!self.data.is_empty());

        let mut buf = String::new();

        buf.push(' ');
        buf.push(' ');
        for ind in 0..self.width {
            buf.push(std::char::from_digit(ind as u32 % 10, 10).unwrap());
        }

        buf.push('\n');

        for y in 0..self.height {
            buf.push(std::char::from_digit(y as u32 % 10, 10).unwrap());
            buf.push(' ');
            for x in 0..self.width {
                buf.push(self[(x, y)].as_char());
            }
            buf.push('\n');
        }

        buf
    }
}

impl<T: ToIndex> std::ops::Index<T> for Grid {
    type Output = Tile;

    fn index(&self, index: T) -> &Self::Output {
        let i = index.to_index(self.width);
        &self.data[i]
    }
}

impl<T: ToIndex> std::ops::IndexMut<T> for Grid {
    fn index_mut(&mut self, index: T) -> &mut Tile {
        let i = index.to_index(self.width);
        &mut self.data[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let input = r"
depth: 510
target: 10,10
        ";

        assert_eq!(114, part1(input.trim()).unwrap());
    }
}