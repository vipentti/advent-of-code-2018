#![allow(dead_code)]
use aoc::{Result, Vector2, ToIndex};

use std::collections::{HashSet, HashMap};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

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

    #[cfg(test)]
    grid.display();

    eprintln!("part1: Target {} @ {} - Risk {}", target, depth, risk);

    Ok(risk)
}

fn part2(s: &str) -> Result<usize> {
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


    let mut grid = Grid::new(400, target.y as usize * 2, depth as usize);

    let risk = grid.calculate(target);

    #[cfg(test)]
    grid.display();

    grid.find_path((0, 0).into(), target);

    Ok(risk)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Tile {
    Rocky,
    Wet,
    Narrow,
    Mouth,
    Target
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Gear {
    Torch,
    Climbing,
    Neither,
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

fn manhattan_distance(a: &Vector2, b: &Vector2) -> usize {
    ((b.x - a.x).abs() + (b.y - a.y).abs()) as usize
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

    fn is_walkable(&self, pos: Vector2) -> bool {
        let index = pos.to_index(self.width);

        self.data.get(index).is_some()
    }

    fn can_move(&self, gear: Gear, from: Vector2, to: Vector2) -> bool {
        !self.next_gear(gear, from, to).is_empty()
    }

    fn heuristic(&self, gear: Gear, from: Vector2, to: Vector2) -> usize {
        manhattan_distance(&from, &to)
    }

    fn gear_is_valid(&self, gear: Gear, pt: Vector2) -> bool {
        let i = pt.to_index(self.width);

        if let Some(tile) = self.data.get(i) {
            match (tile, gear) {
                (Tile::Mouth, Gear::Torch) => true,
                (Tile::Mouth, Gear::Climbing) => true,
                (Tile::Target, Gear::Torch) => true,
                (Tile::Rocky, Gear::Torch) => true,
                (Tile::Rocky, Gear::Climbing) => true,
                (Tile::Wet, Gear::Climbing) => true,
                (Tile::Wet, Gear::Neither) => true,
                (Tile::Narrow, Gear::Neither) => true,
                (Tile::Narrow, Gear::Torch) => true,

                _ => false,
            }

        } else {
            false
        }
    }

    fn next_gear(&self, gear: Gear, from: Vector2, to: Vector2) -> Vec<Gear> {
        let mut gears = Vec::new();

        for g in &[Gear::Torch, Gear::Climbing, Gear::Neither] {
            if self.gear_is_valid(*g, from)
                && self.gear_is_valid(*g, to)
            {
                gears.push(*g);
            }
        }

        gears
    }

    fn find_path(&self, start: Vector2, end: Vector2) {

        fn reconstruct_path(came_from: &HashMap<(Vector2, Gear), (Vector2, Gear)>, mut current: (Vector2, Gear)) -> Vec<(Vector2, Gear)> {
            let mut path = vec![current];

            while let Some(cur) = came_from.get(&current) {
                current = *cur;
                path.push(current);
            }

            // path.pop();

            path.reverse();

            path
        }

        fn count_path_duration(path: &[(Vector2, Gear)]) -> usize {
            let mut duration = 0;


            for index in 0..(path.len() - 1) {

                let (_, g1) = path[index];
                let (_, g2) = path[index + 1];

                if g1 == g2 {
                    duration += 1;
                } else {
                    duration += 7;
                    duration += 1;
                }

            }

            duration
        }

        let start_pair = (start, Gear::Torch);

        let mut g_score: HashMap<(Vector2, Gear), usize> = HashMap::new();
        g_score.insert(start_pair, 0);

        let mut f_score: HashMap<(Vector2, Gear), usize> = HashMap::new();
        f_score.insert(start_pair, self.heuristic(Gear::Torch, start, end));

        let mut closed_set: HashSet<(Vector2, Gear)> = HashSet::new();
        let mut open_set = vec![(start, Gear::Torch)];

        let mut came_from: HashMap<(Vector2, Gear), (Vector2, Gear)> = HashMap::new();

        eprintln!("{:?}", g_score);

        while !open_set.is_empty() {
            open_set.sort_by(|a, b| {
                let ascore = f_score.get(a).unwrap_or(&usize::max_value());
                let bscore = f_score.get(b).unwrap_or(&usize::max_value());

                match ascore.cmp(&bscore).reverse() {
                    std::cmp::Ordering::Equal => {
                        a.cmp(b).reverse()
                    },
                    other => other,
                }
            });

            let (current, gear) = open_set.pop().unwrap();

            if current == end {
                eprintln!("{:?}", (current, gear));
                eprintln!("part2 {:?}", g_score.get(&(current, gear)));
                break;
            }

            closed_set.insert((current, gear));


            let current_score = if let Some(score) = g_score.get(&(current, gear)) {
                *score
            } else {
                eprintln!("{:?}", g_score);
                panic!("Missing score");
            };

            for nbr in current.around().into_iter() {
                if !self.is_walkable(*nbr) {
                    continue;
                }
                let nbr = *nbr;

                if closed_set.contains(&(nbr, gear)) {
                    continue;
                }

                if !self.gear_is_valid(gear, nbr) {
                    continue;
                }

                let dist = current_score + 1;

                if let Some(g) = g_score.get(&(nbr, gear)) {
                    if dist >= *g {
                        continue;
                    }
                }

                if !open_set.contains(&(nbr, gear)) {
                    open_set.push((nbr, gear));
                }

                came_from.insert((nbr, gear), (current, gear));
                g_score.insert((nbr, gear), dist);
                f_score.insert((nbr, gear), dist + manhattan_distance(&nbr, &end));
            }

            for ngear in self.next_gear(gear, current, current) {
                if ngear == gear {
                    continue;
                }

                if closed_set.contains(&(current, ngear)) {
                    continue;
                }

                let dist = current_score + 7;

                if let Some(g) = g_score.get(&(current, ngear)) {
                    if dist >= *g {
                        continue;
                    }
                }

                if !open_set.contains(&(current, ngear)) {
                    open_set.push((current, ngear));
                }

                came_from.insert((current, ngear), (current, gear));
                g_score.insert((current, ngear), dist);
                f_score.insert((current, ngear), dist + manhattan_distance(&current, &end));
            }
        }

        eprintln!("HRE");
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
    fn part2_example_input() {
        let input = r"
depth: 510
target: 10,10
        ";

        assert_eq!(45, part2(input.trim()).unwrap());
    }

    #[test]
    fn part1_example_input() {
        let input = r"
depth: 510
target: 10,10
        ";

        assert_eq!(114, part1(input.trim()).unwrap());
    }
}