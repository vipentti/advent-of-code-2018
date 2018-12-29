use aoc::{CustomError, Result, ToIndex, Vector2};

use std::ops::Index;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s, 10)?;
    part2(&s)?;

    Ok(())
}

fn part1(s: &str, minutes: usize) -> Result<usize> {
    let mut items = Vec::new();

    for line in s.lines() {
        let mut row = Vec::new();

        for ch in line.chars() {
            match ch {
                '.' => {
                    row.push(Acre::Open);
                }
                '#' => {
                    row.push(Acre::Lumber);
                }
                '|' => {
                    row.push(Acre::Tree);
                }

                _ => {
                    return Err(CustomError("Invalid format".to_string()).into());
                }
            }
        }

        items.push(row);
    }

    assert!(!items.is_empty());

    let mut grid = Grid::new_with(items);

    grid.display();

    let mut last_tick = None;

    for tick in 1..=minutes {
        if !grid.tick() {
            eprintln!("After {} minutes", tick);
            last_tick = Some(tick);
            break;
        }
        // grid.display();
    }

    eprintln!("After {} minutes", minutes);
    grid.display();

    if let Some(mut tick) = last_tick {
        let position = grid.prev.iter().position(|p| p == &grid.data);

        if let Some(pos) = position {
            let cycle_len = grid.prev.len() - pos;
            eprintln!("cycle {} minutes {:?}", cycle_len, pos);

            while tick + cycle_len <= minutes {
                tick += cycle_len
            }

            eprintln!("Continuing at {}", tick);

            for _ in tick..minutes {
                grid.tick();
            }
        }

        // advance iteration one cycle at a time as long as possible
        // while tick + cycle_len <= 1000000000 {
        //     tick += cycle_len;
        // }
    }
    // eprintln!("{:?}", grid.counts);

    let tree_count = grid.tree_count();
    let lumber_count = grid.lumber_count();

    let total = lumber_count * tree_count;

    eprintln!(
        "part1: trees {} lumber {} total = {}",
        tree_count, lumber_count, total
    );

    Ok(0)
}

fn part2(s: &str) -> Result<usize> {
    part1(s, 1_000_000_000)
    // part1(s, 1_000)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Acre {
    Open,
    Tree,
    Lumber,
}

impl Acre {
    fn as_char(&self) -> char {
        match self {
            Acre::Open => '.',
            Acre::Tree => '|',
            Acre::Lumber => '#',
        }
    }
}

impl Default for Acre {
    fn default() -> Self {
        Acre::Open
    }
}

fn adjacent(pt: Vector2) -> [Vector2; 8] {
    [
        pt.left(),
        pt.left().up(),
        pt.up(),
        pt.up().right(),
        pt.right(),
        pt.right().down(),
        pt.down(),
        pt.down().left(),
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Grid {
    data: Vec<Acre>,
    width: usize,
    height: usize,
    counts: Vec<(usize, usize)>,
    prev: Vec<Vec<Acre>>,
}

impl Grid {
    fn new_with(items: Vec<Vec<Acre>>) -> Self {
        let width = items[0].len();
        let height = items.len();

        let data: Vec<_> = items.into_iter().flatten().collect();

        Grid {
            width,
            height,
            counts: Vec::new(),
            prev: vec![data.clone()],
            data,
        }
    }

    fn tree_count(&self) -> usize {
        self.data.iter().filter(|&&p| p == Acre::Tree).count()
    }

    fn lumber_count(&self) -> usize {
        self.data.iter().filter(|&&p| p == Acre::Lumber).count()
    }

    fn tick(&mut self) -> bool {
        let mut next_data = self.data.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let pt: Vector2 = (x, y).into();

                let current = self[pt];

                let adjacent = adjacent(pt);

                match current {
                    Acre::Open => {
                        let count_trees = adjacent
                            .into_iter()
                            .filter_map(|p| self.get(*p))
                            .filter(|&&p| p == Acre::Tree)
                            .count();

                        if count_trees >= 3 {
                            // eprintln!("{} trees {}", pt, count_trees);

                            next_data[pt.to_index(self.width)] = Acre::Tree;
                        }
                    }
                    Acre::Tree => {
                        let count = adjacent
                            .into_iter()
                            .filter_map(|p| self.get(*p))
                            .filter(|&&p| p == Acre::Lumber)
                            .count();

                        if count >= 3 {
                            // eprintln!("{} tree into lumber {}", pt, count);
                            next_data[pt.to_index(self.width)] = Acre::Lumber;
                        }
                    }
                    Acre::Lumber => {
                        let count_trees = adjacent
                            .into_iter()
                            .filter_map(|p| self.get(*p))
                            .filter(|&&p| p == Acre::Tree)
                            .count();
                        let count_lumber = adjacent
                            .into_iter()
                            .filter_map(|p| self.get(*p))
                            .filter(|&&p| p == Acre::Lumber)
                            .count();

                        if count_trees >= 1 && count_lumber >= 1 {
                            // eprintln!("{} lumber {} {}", pt, count_trees, count_lumber);
                            next_data[pt.to_index(self.width)] = Acre::Lumber;
                        } else {
                            // eprintln!("{} open {}", pt, count_trees);
                            next_data[pt.to_index(self.width)] = Acre::Open;
                        }
                    }
                }
            }
        }

        let any_same = self.prev.iter().filter(|&p| p == &next_data).count();

        if any_same >= 1 {
            std::mem::replace(&mut self.data, next_data);
            eprintln!("Repeat");
            return false;
        }

        self.prev.push(next_data.clone());

        std::mem::replace(&mut self.data, next_data);

        true
    }

    fn get<T: ToIndex>(&self, idx: T) -> Option<&Acre> {
        let i = idx.to_index(self.width);
        self.data.get(i)
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

impl<T: ToIndex> Index<T> for Grid {
    type Output = Acre;

    fn index(&self, index: T) -> &Self::Output {
        let i = index.to_index(self.width);
        &self.data[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let input = r"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
        ";

        assert_eq!(1147, part1(input.trim(), 10).unwrap());
    }
}
