use aoc::{Result, CustomError};
use std::ops::{Add, Sub};
use std::fmt;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;

    Ok(())
}


fn part1(s: &str) -> Result<usize> {

    let bots: std::result::Result<Vec<_>, _> = s.lines()
        .map(|s| s.parse::<Bot>())
        .collect();

    let bots = bots?;


    let max = bots.iter()
        .max_by_key(|b| b.radius)
        ;

    // eprintln!("Bots, {:?}", bots);

    if let Some(bot) = max {
        eprintln!("Largest {:?}", bot);


        let bots_in_range = bots.iter()
            .map(|b| manhattan_distance(bot.pos, b.pos))
            .filter(|d| *d <= bot.radius as usize)
            .count();

        eprintln!("part1: Bots in range {:?}", bots_in_range);

        return Ok(bots_in_range);
    }


    Ok(0)
}

fn manhattan_distance(a: V3, b: V3) -> usize {
    ((b.x - a.x).abs()
     + (b.y - a.y).abs()
     + (b.z - a.z).abs()
     ) as usize
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct Bot {
    pos: V3,
    radius: i64,
}

impl FromStr for Bot {
    type Err = CustomError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            // pos=<0,0,0>, r=4
            static ref RE: Regex = Regex::new(r"pos=<([\d\-]+),([\d\-]+),([\d\-]+)>, r=(\d+)").unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| CustomError("Invalid captures".to_owned()))?;


        let x = aoc::get_value(&caps, 1)?;
        let y = aoc::get_value(&caps, 2)?;
        let z = aoc::get_value(&caps, 3)?;
        let r = aoc::get_value(&caps, 4)?;

        Ok(Bot {
            pos: V3 {
                x,
                y,
                z,
            },
            radius: r,
        })

    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct V3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl fmt::Debug for V3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for V3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! impl_ops {
    ($tp: ty, $p: pat => $x:expr, $y:expr, $z:expr) => {
        impl Add<$tp> for V3 {
            type Output = V3;

            fn add(self, $p: $tp) -> Self::Output {
                V3 {
                    x: self.x + $x,
                    y: self.y + $y,
                    z: self.z + $z,
                }
            }
        }
        impl Sub for V3 {
            type Output = V3;

            fn sub(self, $p: $tp) -> Self::Output {
                V3 {
                    x: self.x - $x,
                    y: self.y - $y,
                    z: self.z - $z,
                }
            }
        }
    };
}

impl_ops!(V3, p => p.x, p.y, p.z);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let input = r"
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
        ";

        assert_eq!(7, part1(input.trim()).unwrap());
    }
}