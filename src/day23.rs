use aoc::{Result, CustomError};
use std::ops::{Add, Sub};
use std::fmt;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

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

    #[cfg(test)]
    eprintln!("Bots, {:?}", bots);

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

fn part2(s: &str) -> Result<usize> {
    let bots: std::result::Result<Vec<_>, _> = s.lines()
        .map(|s| s.parse::<Bot>())
        .collect();

    let bots = bots?;

    #[cfg(test)]
    eprintln!("Bots, {:?}", bots);

    let min_x = bots.iter().map(|b| b.pos.x).min().unwrap();
    let min_y = bots.iter().map(|b| b.pos.y).min().unwrap();
    let min_z = bots.iter().map(|b| b.pos.z).min().unwrap();
    let max_x = bots.iter().map(|b| b.pos.x).max().unwrap();
    let max_y = bots.iter().map(|b| b.pos.y).max().unwrap();
    let max_z = bots.iter().map(|b| b.pos.z).max().unwrap();


    eprintln!("Min {:?}", (min_x, min_y, min_z));
    eprintln!("Max {:?}", (max_x, max_y, max_z));

    let tmp = [max_x - min_x, max_y - min_y, max_z - min_z];
    let max_size = *tmp.into_iter().max().unwrap();

    let mut size = 1;

    while size < max_size {
        size *= 2;
    }

    eprintln!("Max size {:?} -> size {}", max_size, size);

    let mut spaces = vec![
        Space {
            nr_bots: bots.len(),
            pos: (min_x, min_y, min_z).into(),
            size,
        }
    ];

    use std::cmp::Ordering;

    while !spaces.is_empty() {
        spaces.sort_by(|a, b| {
            match a.nr_bots.cmp(&b.nr_bots) {
                Ordering::Equal => {
                    match a.dist().cmp(&b.dist()).reverse() {
                        Ordering::Equal => {
                            a.size.cmp(&b.size).reverse()
                        },
                        other => other,
                    }
                },
                other => other,
            }
        });

        let current = spaces.pop().unwrap();

        if current.size == 1 {
            eprintln!("Found {:?}", current);
            eprintln!("Part2: {:?}", current.dist());

            return Ok(current.dist());
        }

        let ns = current.size / 2;

        let s1 = {
            let mut s = Space::at(current.pos + (0, 0, 0), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        let s2 = {
            let mut s = Space::at(current.pos + (ns, 0, 0), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        let s3 = {
            let mut s = Space::at(current.pos + (0, ns, 0), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        let s4 = {
            let mut s = Space::at(current.pos + (0, 0, ns), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        let s5 = {
            let mut s = Space::at(current.pos + (ns, ns, 0), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        let s6 = {
            let mut s = Space::at(current.pos + (0, ns, ns), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        let s7 = {
            let mut s = Space::at(current.pos + (ns, 0, ns), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        let s8 = {
            let mut s = Space::at(current.pos + (ns, ns, ns), ns);

            s.nr_bots = bots.iter()
                .filter(|b| b.in_range(s))
                .count();

            s
        };

        if s1.nr_bots > 0 { spaces.push(s1); }
        if s2.nr_bots > 0 { spaces.push(s2); }
        if s3.nr_bots > 0 { spaces.push(s3); }
        if s4.nr_bots > 0 { spaces.push(s4); }
        if s5.nr_bots > 0 { spaces.push(s5); }
        if s6.nr_bots > 0 { spaces.push(s6); }
        if s7.nr_bots > 0 { spaces.push(s7); }
        if s8.nr_bots > 0 { spaces.push(s8); }
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
struct Space {
    nr_bots: usize,
    /// lower left corner
    pos: V3,
    size: i64,
}

impl Space {
    pub fn at(pos: V3, size: i64) -> Self {
        Space {
            nr_bots: 0,
            pos,
            size,
        }
    }

    pub fn dist(&self) -> usize {
        manhattan_distance(self.pos, V3 {
            x: 0,
            y: 0,
            z: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Bot {
    pos: V3,
    radius: i64,
}

impl Bot {
    fn in_range(&self, space: Space) -> bool {
        let min = space.pos;
        let max = space.pos + (space.size - 1, space.size - 1, space.size - 1);

        let mut d = 0;

        if self.pos.x > max.x { d += (self.pos.x - max.x).abs() as i64 }
        if self.pos.x < min.x { d += (min.x - self.pos.x).abs() as i64 }

        if self.pos.y > max.y { d += (self.pos.y - max.y).abs() as i64 }
        if self.pos.y < min.y { d += (min.y - self.pos.y).abs() as i64 }

        if self.pos.z > max.z { d += (self.pos.z - max.z).abs() as i64 }
        if self.pos.z < min.z { d += (min.z - self.pos.z).abs() as i64 }

        d <= self.radius
    }
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

impl std::convert::From<(i64, i64, i64)> for V3 {
    fn from(v: (i64, i64, i64)) -> Self {
        V3 { x: v.0, y: v.1, z: v.2 }
    }
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
        impl Sub<$tp> for V3 {
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
impl_ops!((i64, i64, i64), p => p.0, p.1, p.2);

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

    #[test]
    fn part2_example_input() {
        let input = r"
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
        ";

        assert_eq!(36, part2(input.trim()).unwrap());
    }
}