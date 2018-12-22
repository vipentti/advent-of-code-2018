#![allow(dead_code)]
use aoc::{Result, Vector2, CustomError, ToIndex};

use std::collections::{HashSet, HashMap, VecDeque};
use regex::Regex;
use lazy_static::lazy_static;

use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    // part2(&s)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Direction {
    Down,
    Side,
    Up,
}

impl Default for Direction {
    fn default() -> Self { Direction::Down }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Tile {
    Sand,
    Clay,
    Spring,
    Flow,
    Rest,
    Invalid,
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Clay => '#',
            Tile::Sand => '.',
            Tile::Spring => '+',
            Tile::Flow => '|',
            Tile::Rest => '~',
            Tile::Invalid => 'X',
        }
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Sand }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Grid {
    data: Vec<Tile>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Around {
    left: Tile,
    right: Tile,
    up: Tile,
    down: Tile,
    current: Tile,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            data: vec![Tile::default(); width * height],
            width,
            height,
        }
    }

    fn count_water(&self) -> usize {
        self.data.iter()
            .filter(|&&v| v == Tile::Flow || v == Tile::Rest)
            .count()
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

    fn set<T: ToIndex>(&mut self, idx: T, value: Tile) {
        let i = idx.to_index(self.width);

        if let Some(val) = self.data.get_mut(i) {
            *val = value;
        }
    }

    fn get<T: ToIndex>(&self, idx: T) -> Option<&Tile> {
        let i = idx.to_index(self.width);
        self.data.get(i)
    }

    fn get_copy<T: ToIndex>(&self, idx: T) -> Option<Tile> {
        self.get(idx).map(|p| *p)
    }

    fn get_copy_or_invalid<T: ToIndex>(&self, idx: T) -> Tile {
        self.get(idx).map(|p| *p).unwrap_or(Tile::Invalid)
    }

    fn get_i<T: ToIndex>(&self, idx: T) -> Tile {
        self.get(idx).map(|p| *p).unwrap_or(Tile::Invalid)
    }

    /// [up, right, down, left]
    ///
    /*
    fn get_around(&self, pt: Vector2) -> [Option<Tile>; 4] {
        let mut result = [Default::default(); 4];

        for (i, pt) in pt.around().into_iter().enumerate() {
            result[i] = self.get(pt).map(|p| *p);
        }

        result
    }
    */
    fn get_around(&self, pt: Vector2) -> Around {
        Around {
            left: self.get_copy(pt.left()).unwrap_or(Tile::Invalid),
            right: self.get_copy(pt.right()).unwrap_or(Tile::Invalid),
            down: self.get_copy(pt.down()).unwrap_or(Tile::Invalid),
            up: self.get_copy(pt.up()).unwrap_or(Tile::Invalid),
            current: self.get_copy(pt).unwrap_or(Tile::Invalid),
        }
    }
}

impl<T: ToIndex> Index<T> for Grid {
    type Output = Tile;

    fn index(&self, index: T) -> &Self::Output {
        let i = index.to_index(self.width);
        &self.data[i]
    }
}

impl<T: ToIndex> IndexMut<T> for Grid {
    fn index_mut(&mut self, index: T) -> &mut Tile {
        let i = index.to_index(self.width);
        &mut self.data[i]
    }
}

fn part1(s: &str) -> Result<i32> {
    let mut clay_locations = read_clay_locations(s)?;
    let mut spring: Vector2 = Vector2::new(500, 0);
    let (_, c_min_y, _, _ ) = get_size_from(&clay_locations)?;

    let mut temp_locations = clay_locations.clone();
    temp_locations.push(spring);

    let (mut min_x, min_y, max_x, max_y) = get_size_from(&temp_locations)?;
    min_x -= 1;


    let size_x = (max_x - min_x).abs() as usize + 1;
    let size_y = (max_y - min_y).abs() as usize + 1;

    eprintln!("Size {}x{}", size_x, size_y);
    eprintln!("Min {}x{}", min_x, min_y);
    eprintln!("Max {}x{}", max_x, max_y);
    eprintln!("Clay {}", c_min_y);

    // Normalize the coordinates from -X -> +X to 0..
    for pos in clay_locations.iter_mut() {
        *pos -= (min_x, min_y);
    }

    spring -= (min_x, min_y);

    eprintln!("spring {:?}", spring);
    // eprintln!("clay {:?}", clay_locations);

    let mut grid = Grid::new(size_x, size_y);

    for clay in clay_locations.iter() {
        grid[*clay] = Tile::Clay;
    }

    grid[spring] = Tile::Spring;


    // display_grid(&grid);

    // for i in 0..=1_000_000 {
    //     eprintln!("springs {:?}", springs);

    //     if !produce_water_2(spring, &mut grid, &mut waters, &mut springs) {
    //         eprintln!("Tick {}", i);
    //         // display_grid(&grid);
    //         break;
    //     }
    //     display_grid(&grid);
    // }
    let waters = run_stream(spring, &mut grid);

    display_grid(&grid);

    eprintln!("count {}", grid.count_water());


    let keys = waters.keys()
        .filter(|&&v| v.y >= c_min_y)
        .count();
    eprintln!("Water {}", waters.keys().count());
    eprintln!("Water keys {}", keys);

    let resting = waters.iter()
        .filter(|(v, t)| v.y >= c_min_y && **t == Tile::Rest)
        .count();
    eprintln!("Resting keys {}", resting);

    Ok(0)
}

type Range = std::ops::RangeInclusive<i32>;

fn run_stream(spring: Vector2, grid: &mut Grid) -> HashMap<Vector2, Tile> {
    let mut q = VecDeque::new();
    q.push_back((spring, Direction::Down));
    let mut waters: HashMap<Vector2, Tile> = HashMap::new();

    while !q.is_empty() {
        let (mut pos, dir) = q.pop_front().unwrap();

        // eprintln!("{} {:?}", pos, dir);

        match dir {
            Direction::Down => {

                while pos.y + 1 < grid.height as i32
                    && !is_resting(pos.down(), grid)
                {
                    pos = pos.down();
                    waters.insert(pos, Tile::Flow);
                    // grid[pos] = Tile::Flow;
                    grid.set(pos, Tile::Flow);
                }

                if is_at_edge(pos, grid) {
                    continue;
                }

                if !q.contains(&(pos, Direction::Up)) {
                    q.push_back((pos, Direction::Up));
                }
            }

            Direction::Up => {
                // eprintln!("up {}", pos);
                while let Some(range) = find_walls(pos, grid, floor_below(pos, grid)) {

                    // eprintln!("Container {} {:?}", pos, range );

                    range.clone().for_each(|x| {
                        let tp = (x, pos.y).into();
                        waters.insert(tp, Tile::Rest);
                        // grid[tp] = Tile::Rest;
                        grid.set(tp, Tile::Rest);
                    });

                    pos = pos.up();
                }
                // eprintln!("up {}", pos);

                waters.insert(pos, Tile::Flow);
                grid.set(pos, Tile::Flow);
                //grid[pos] = Tile::Flow;
                //pos = pos.up();

                if !q.contains(&(pos, Direction::Side)) {
                    q.push_back((pos, Direction::Side));
                }
            },
            Direction::Side => {
                let floor = floor_below(pos, grid);

                if floor.is_none() {
                    continue;
                }

                let floor = floor.unwrap();

                // eprintln!("hre;{:?}", floor);

                let mut min_pos = pos;
                let (start, end) = (*floor.start(), *floor.end());

                while min_pos.x - 1 >= start - 1 && is_inside(min_pos, grid)  && !is_resting(min_pos.left(), grid) {
                    min_pos = min_pos.left();
                    waters.insert(min_pos, Tile::Flow);
                    grid.set(min_pos, Tile::Flow);
                }

                // eprintln!("min {}", min_pos);

                let mut max_pos = pos;

                // eprintln!("end {}", end + 1);
                // eprintln!("rr {}", max_pos.x + 1);

                while max_pos.x + 1 <= end + 1 && is_inside(max_pos, grid) && !is_resting(max_pos.right(), grid) {
                    max_pos = max_pos.right();
                    waters.insert(max_pos, Tile::Flow);
                    grid.set(max_pos, Tile::Flow);
                }

                // eprintln!("max {}", max_pos);


                if min_pos.x < start && !q.contains(&(min_pos, Direction::Down)) {
                    q.push_back((min_pos, Direction::Down));
                }
                if max_pos.x > end && !q.contains(&(max_pos, Direction::Down)) {
                    q.push_back((max_pos, Direction::Down));
                }
            },
            _ => {
            }
        }
    }

    waters
}

fn find_walls(v: Vector2, grid: &Grid, floor: Option<Range>) -> Option<Range> {
    // eprintln!("Here {} {:?} {:?}", v, grid.get(v), floor);
    // if !is(v.down(), Tile::Clay, grid) {
    if !is_resting(v.down(), grid) {
        return None;
    }

    if floor.is_none() {
        return None;
    }
    // eprintln!("Here2 {} {:?} {:?}", v, grid.get(v), floor);

    let floor = floor.unwrap();

    let mut min_pos = v;

    while !is_resting(min_pos.left(), grid) && min_pos.x >= 0 {
        min_pos = min_pos.left();
    }
    // eprintln!("Here3 {} {:?} {:?}", v, grid.get(v), floor);

    let mut max_pos = v;

    while !is_resting(max_pos.right(), grid) && max_pos.x < grid.width as i32 {
        max_pos = max_pos.right();
    }

    // eprintln!("{} {}", min_pos, max_pos);

    //Some(floor)
    if min_pos.x >= *floor.start() && max_pos.x <= *floor.end() {
            return Some(min_pos.x..=max_pos.x);
        }

    None
}


fn floor_below(v: Vector2, grid: &Grid) -> Option<Range>{
    let below = v.down();

    if is_resting(below, grid) {

        let mut min_pos = below;

        while is_resting(min_pos.left(), grid) {
            min_pos = min_pos.left();
        }

        let mut max_pos = below;

        while is_resting(max_pos.right(), grid) {
            max_pos = max_pos.right();
        }


        return Some(min_pos.x..=max_pos.x);
    }

    None
}

fn is_inside(v: Vector2, grid: &Grid) -> bool {
    (v.x >= 0 && v.x < grid.width as i32)
    && (v.y >= 0 && v.y < grid.height as i32)
}

fn is_resting(v: Vector2, grid: &Grid) -> bool {
    grid.get(v) == Some(&Tile::Rest)
    || grid.get(v) == Some(&Tile::Clay)
}

fn is_tile(v: Vector2, expected: Tile, grid: &Grid) -> bool {
    grid.get_copy_or_invalid(v) == expected
}

fn is(v: Vector2, expected: Tile, grid: &Grid) -> bool {
    grid.get_copy_or_invalid(v) == expected
}

fn is_not(v: Vector2, expected: Tile, grid: &Grid) -> bool {
    grid.get_copy_or_invalid(v) != expected
    // && grid.get_copy_or_invalid(v) != Tile::Invalid
}

fn is_full_of_water(mut row: Vector2, grid: &Grid) -> bool {

    if is_tile(row, Tile::Clay, grid) {
        return false;
    }

    while is_not(row, Tile::Clay, grid) {
        if is_not(row, Tile::Rest, grid) {
            return false;
        }

        row = row.left();
    }

    while is_not(row, Tile::Clay, grid) {
        if is_not(row, Tile::Rest, grid) {
            return false;
        }

        row = row.right();
    }

    true
}

fn first_down(start: Vector2, grid: &Grid) -> Option<(Tile, Vector2)> {
    for y in (start.y + 1)..grid.height as i32 {
        let loc: Vector2 = (start.x, y).into();
        if let Some(Tile::Sand) = grid.get(loc) {
            return Some((Tile::Sand, loc));
        } else if let Some(Tile::Clay) = grid.get(loc) {
            return Some((Tile::Clay, loc));
        }
    }

    None
}

fn first_left(start: Vector2, grid: &Grid) -> Option<(Tile, Vector2)> {
    if start.y == grid.height as i32 - 1 {
        return None;
    }

    for x in (0..=(start.x - 1)).rev() {
        let c: Vector2 = (x, start.y).into();
        if let Some(Tile::Sand) = grid.get(c) {
            return Some((Tile::Sand, c));
        } else if let Some(Tile::Clay) = grid.get(c) {
            return Some((Tile::Clay, c));
        }
    }
    None
}

fn first_right(start: Vector2, grid: &Grid) -> Option<(Tile, Vector2)> {
    if start.y == grid.height as i32 - 1 {
        return None;
    }
    for x in (start.x + 1)..grid.width as i32 {
        let c: Vector2 = (x, start.y).into();
        if let Some(Tile::Sand) = grid.get(c) {
            return Some((Tile::Sand, c));
        } else if let Some(Tile::Clay) = grid.get(c) {
            return Some((Tile::Clay, c));
        }
    }
    None
}

fn last_non_clay_right(start: Vector2, grid: &Grid) -> Option<(Tile, Vector2)> {

    if start.y == grid.height as i32 - 1 {
        return None;
    }

    for x in (start.x + 1)..grid.width as i32 {
        let c: Vector2 = (x, start.y).into();
        if let Some(Tile::Clay) = grid.get(c) {
            let actual = c.left();
            if let Some(t) = grid.get(actual) {
                return Some((*t, actual));
            }
        }
    }

    None
}

fn is_at_edge(start: Vector2, grid: &Grid) -> bool {
    if start.y == 0 {
        return true;
    }

    if start.y == grid.height as i32 - 1 {
        return true;
    }

    if start.x == 0 {
        return true;
    }

    if start.x == grid.width as i32 - 1 {
        return true;
    }

    false
}


fn display_grid(grid: &Grid) {
    eprintln!("{}", grid.render_to_string());
}

fn get_size_from(points: &[Vector2]) -> Result<(i32, i32, i32, i32)> {
    let min_x = points
        .iter()
        .map(|c| c.x)
        .min()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing min_x".to_string()).into()
        })?;

    let min_y = points
        .iter()
        .map(|c| c.y)
        .min()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing min_y".to_string()).into()
        })?;

    let max_x = points
        .iter()
        .map(|c| c.x)
        .max()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing max_x".to_string()).into()
        })?;

    let max_y = points
        .iter()
        .map(|c| c.y)
        .max()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing max_y".to_string()).into()
        })?;

    Ok((min_x, min_y, max_x, max_y))
}

fn read_clay_locations(s: &str) -> Result<Vec<Vector2>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(x|y)=(\d+)\.?\.?(\d+)?").unwrap();
    }

    let mut clay_locations: Vec<Vector2> = Vec::new();

    for (_ind, line) in s.lines().enumerate() {
        let mut x_values = Vec::new();
        let mut y_values = Vec::new();

        for caps in RE.captures_iter(line) {
            // eprintln!("{} {} {} -> {:?}", ind, line, caps.len(), caps);

            let name = caps.get(1).map_or("", |m| m.as_str());
            let range_start = caps.get(2).map_or("", |m| m.as_str());
            if let Some(end) = caps.get(3).map(|m| m.as_str()) {
                let start = range_start.parse::<i32>()
                    .map_err(|e| Box::new(e))?;
                let end = end.parse::<i32>()
                    .map_err(|e| Box::new(e))?;
                match name {
                    "x" => {
                        x_values.extend(start..=end);
                    },
                    "y" => {
                        y_values.extend(start..=end);
                    },
                    v => {
                        return Err(CustomError(format!("Unknown field {}", v)).into());
                    }
                }
            } else {
                match name {
                    "x" => {
                        let x = range_start.parse::<i32>()
                            .map_err(|e| Box::new(e))?;

                        x_values.push(x);
                    },
                    "y" => {
                        let y = range_start.parse::<i32>()
                            .map_err(|e| Box::new(e))?;

                        y_values.push(y);
                    },
                    v => {
                        return Err(CustomError(format!("Unknown field {}", v)).into());
                    }
                }
            }
        }

        // eprintln!("x: {:?} y: {:?}", x_values, y_values);

        if x_values.len() == 1 {
            let x = *x_values.first().unwrap();

            for y in y_values {
                clay_locations.push((x, y).into());
            }

        } else if y_values.len() == 1 {
            let y = *y_values.first().unwrap();

            for x in x_values {
                clay_locations.push((x, y).into());
            }

        } else {
            return Err(CustomError("invalid format".to_string()).into());
        }
    }

    Ok(clay_locations)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let input = r"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
        ";

        assert_eq!(57, part1(input.trim()).unwrap());
    }
}