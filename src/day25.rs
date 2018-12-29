use aoc::Result;

use std::collections::{BTreeSet, VecDeque};
use std::fmt;
use std::ops::{Add, Sub};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;

    Ok(())
}

fn read_points(s: &str) -> Result<Vec<V4>> {
    let mut points = Vec::new();

    for line in s.lines() {
        let parts: std::result::Result<Vec<i64>, _> = line
            .split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<i64>())
            .collect();

        let parts = parts?;

        points.push(V4 {
            x: parts[0],
            y: parts[1],
            z: parts[2],
            w: parts[3],
        });
    }

    Ok(points)
}

fn part1(s: &str) -> Result<i64> {
    let points = read_points(s)?;
    // eprintln!("{:?}", points);

    eprintln!("Points: {:?}", points.len());
    let mut graph = Graph::new(points);

    graph.setup();

    eprintln!("Finished setup {}", graph.edges.len());

    // eprintln!("{:?}", graph);

    #[cfg(test)]
    graph.display();

    let count = graph.visit_all();

    eprintln!("part1: {:?}", count);

    Ok(count as i64)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Graph {
    rows: usize,
    columns: usize,
    vertices: Vec<V4>,
    edges: Vec<Vec<i64>>,
}

impl Graph {
    fn new(vertices: Vec<V4>) -> Self {
        let rows = vertices.len();
        let columns = vertices.len();

        Graph {
            vertices,
            rows,
            columns,
            edges: vec![vec![Default::default(); columns]; rows],
        }
    }

    fn visit_all(&self) -> usize {
        let mut visited = BTreeSet::new();

        let mut count = 0;

        loop {
            if visited.len() == self.vertices.len() {
                break;
            }

            for ind in 0..self.rows {
                if !visited.contains(&ind) {
                    if self.visit(ind, &mut visited) {
                        count += 1;
                    }
                    break;
                }
            }
        }

        count
    }

    fn visit(&self, start: usize, visited: &mut BTreeSet<usize>) -> bool {
        if start > self.edges.len() - 1 {
            return false;
        }
        let mut q = VecDeque::new();
        q.push_back(start);

        // let mut visited: BTreeSet<usize> = BTreeSet::new();

        while !q.is_empty() {
            let current_index = q.pop_front().unwrap();

            visited.insert(current_index);

            for (edge, value) in self.edges[current_index].iter().enumerate() {
                if *value == 1 {
                    if !visited.contains(&edge) {
                        q.push_back(edge)
                    }
                }
            }
        }

        true
    }

    fn setup(&mut self) {
        let points = self.vertices.clone();
        let points2 = self.vertices.clone();

        for index in 0..points.len() {
            let me = points[index];
            for alt in (index + 1)..points.len() {
                let other = points2[alt];

                if manhattan_distance(me, other) <= 3 {
                    self.add_edge(me, other);
                    self.add_edge(other, me);
                }
            }
        }
    }

    fn display(&self) {
        let mut buf = String::new();
        buf.push('\n');

        for (ind, v) in self.vertices.iter().enumerate() {
            buf.push(std::char::from_digit(ind as u32 % 10, 10).unwrap());
            buf.push(':');
            buf.push(' ');
            buf.push_str(&format!("{}", v));

            buf.push('\n');
        }
        buf.push('\n');

        buf.push(' ');
        buf.push(' ');
        for ind in 0..self.columns {
            buf.push(std::char::from_digit(ind as u32 % 10, 10).unwrap());
        }
        buf.push('\n');

        for y in 0..self.rows {
            buf.push(std::char::from_digit(y as u32 % 10, 10).unwrap());
            buf.push(' ');
            for x in 0..self.columns {
                buf.push(
                    std::char::from_digit(self.edges[y][x] as u32 % 10, 10)
                        .unwrap(),
                );
            }
            buf.push('\n');
        }

        buf.push('\n');

        eprintln!("{}", buf);
    }

    fn index(&self, v: V4) -> Option<usize> {
        self.vertices.iter().position(|&p| p == v)
    }

    fn add_edge(&mut self, a: V4, b: V4) {
        let row = self.index(a).unwrap();
        let column = self.index(b).unwrap();

        self.edges[row][column] = 1;
    }
}

fn manhattan_distance(a: V4, b: V4) -> i64 {
    ((b.x - a.x).abs()
        + (b.y - a.y).abs()
        + (b.z - a.z).abs()
        + (b.w - a.w).abs()) as i64
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct V4 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

impl std::convert::From<(i64, i64, i64, i64)> for V4 {
    fn from(v: (i64, i64, i64, i64)) -> Self {
        V4 {
            x: v.0,
            y: v.1,
            z: v.2,
            w: v.3,
        }
    }
}

impl fmt::Debug for V4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Display for V4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! impl_ops {
    ($tp: ty, $p: pat => $x:expr, $y:expr, $z:expr, $w:expr) => {
        impl Add<$tp> for V4 {
            type Output = V4;

            fn add(self, $p: $tp) -> Self::Output {
                V4 {
                    x: self.x + $x,
                    y: self.y + $y,
                    z: self.z + $z,
                    w: self.w + $w,
                }
            }
        }
        impl Sub<$tp> for V4 {
            type Output = V4;

            fn sub(self, $p: $tp) -> Self::Output {
                V4 {
                    x: self.x - $x,
                    y: self.y - $y,
                    z: self.z - $z,
                    w: self.w - $w,
                }
            }
        }
    };
}

impl_ops!(V4, p => p.x, p.y, p.z, p.w);
impl_ops!((i64, i64, i64, i64), p => p.0, p.1, p.2, p.3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let sample = r"
0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0
        ";

        assert_eq!(2, part1(sample.trim()).unwrap());

        let sample = r"
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
        ";

        assert_eq!(4, part1(sample.trim()).unwrap());

        let sample = r"
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
        ";

        assert_eq!(3, part1(sample.trim()).unwrap());

        let sample = r"
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
        ";

        assert_eq!(8, part1(sample.trim()).unwrap());
    }
}
