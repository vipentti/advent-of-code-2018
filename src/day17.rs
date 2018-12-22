#![allow(dead_code)]
use aoc::{Result, Vector2, CustomError};

use std::collections::{HashSet, VecDeque};
use regex::Regex;
use lazy_static::lazy_static;

use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    // part2(&s)?;

    Ok(())
}

trait ToIndex {
    fn to_index(self, width: usize) -> usize;
}

impl ToIndex for Vector2 {
    fn to_index(self, width: usize) -> usize {
        if self.y < 0 {
            return usize::max_value();
        }
        self.y as usize * width + self.x as usize
    }
}

impl ToIndex for (i32, i32) {
    fn to_index(self, width: usize) -> usize {
        self.1 as usize * width + self.0 as usize
    }
}

impl ToIndex for (usize, usize) {
    fn to_index(self, width: usize) -> usize {
        self.1 * width + self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Direction {
    Down,
    Side,
    Uo,
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

    let mut temp_locations = clay_locations.clone();
    temp_locations.push(spring);

    let (min_x, min_y, max_x, max_y) = get_size_from(&temp_locations)?;

    let size_x = (max_x - min_x).abs() as usize + 1;
    let size_y = (max_y - min_y).abs() as usize + 1;

    eprintln!("Size {}x{}", size_x, size_y);
    eprintln!("Min {}x{}", min_x, min_y);
    eprintln!("Max {}x{}", max_x, max_y);

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

    let mut waters: Vec<Vector2> = Vec::new();

    let mut springs: Vec<Vector2> = Vec::new();

    // display_grid(&grid);

    for i in 0..=1_000_000 {
        eprintln!("springs {:?}", springs);

        if !produce_water_2(spring, &mut grid, &mut waters, &mut springs) {
            eprintln!("Tick {}", i);
            // display_grid(&grid);
            break;
        }
        display_grid(&grid);
    }

    display_grid(&grid);

    let set: HashSet<_> = waters.into_iter().collect();
    eprintln!("Water {}", set.len());

    Ok(0)
}

fn run_stream(spring: Vector2, grid: &mut Grid) {
    let mut vec = VecDeque::new();
    vec.push((spring, Direction::Down));

}

fn is_inside(v: Vector2, grid: &Grid) -> bool {
    (v.x >= 0 && v.x < grid.width as i32)
    && (v.y >= 0 && v.y < grid.height as i32)
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

fn next_free_loc(start: Vector2, spring: Vector2, grid: &Grid) -> Option<(Direction, Vector2)> {

    if let Some(Tile::Sand) = grid.get(start.down()) {
        let above = grid.get_i(start.up());
        let below = grid.get_i(start.down().down());
        if above != Tile::Flow && below == Tile::Sand {
            eprintln!("{} Above {:?} below {:?}", start, above, below);
            return Some((Direction::Down, start.down()));
        }
        // if is_not(start.up(), Tile::Flow, grid) && is(start.down().down(), Tile::Sand, grid)
        // {
        // }
        return Some((Direction::None, start.down()));
    }

    if let Some(Tile::Sand) = grid.get(start.left()) {
        if !is_at_edge(start, grid) {
            return Some((Direction::Left, start.left()));
        }
    }

    if let Some(Tile::Sand) = grid.get(start.right()) {
        if !is_at_edge(start, grid) {
            return Some((Direction::Right, start.right()));
        }
    }

    // Start going down from spring

    let mut loc = spring.down();

    while is_not(loc.down(), Tile::Clay, grid)
        && (is(loc.left(), Tile::Sand, grid) || is(loc.right(), Tile::Sand, grid))
    {
        loc = loc.down();
    }

    loc = loc.up();

    eprintln!("Spring {}", loc);

    if let Some(Tile::Sand) = grid.get(loc.left()) {
        if !is_at_edge(loc, grid) {
            return Some((Direction::Left, loc.left()));
        }
    }

    if let Some(Tile::Sand) = grid.get(loc.right()) {
        if !is_at_edge(loc, grid) {
            return Some((Direction::Right, loc.right()));
        }
    }

    None
}

fn next_free_location(start: Vector2, grid: &Grid) -> Option<Vector2> {
    if let Some(Tile::Sand) = grid.get(start.down()) {
        return Some(start.down());
    }

    // If we cannot go down, we'll try going left first
    // And filling all the locations on that side first

    if let Some(Tile::Sand) = grid.get(start.left()) {
        if !is_at_edge(start, grid) {
            return Some(start.left());
        }
    }

    // Clay on left, we must start backtracking
    // First attempt going right as far as we can
    if let Some(Tile::Clay) = grid.get(start.left()) {

        let mut loc = start;
        // eprintln!("Left clay {}", loc);

        while is_not(loc.right(), Tile::Clay, grid) {
            if let Some(Tile::Sand) = grid.get(loc.right()) {
                if !is_at_edge(loc.right(), grid) {
                    return Some(loc.right());
                }
            }

            loc = loc.right();
        }

        // eprintln!("After left {}", loc);

        // Attempt going up a row
        if let Some(Tile::Flow) = grid.get(loc.up()) {
            return next_free_location(loc.up(), grid);
        }
    }

    if let Some(Tile::Sand) = grid.get(start.right()) {
        if !is_at_edge(start, grid) {
            return Some(start.right());
        }
    }

    if let Some(Tile::Clay) = grid.get(start.right()) {


        let mut loc = start;
        // eprintln!("Right clay {}", loc);

        // We have free space above us, find first
        // free that goes to the left
        if is(loc.up(), Tile::Sand, grid) {
            loc = loc.up();
            while is_not(loc.left(), Tile::Clay, grid) {
                if let Some(Tile::Flow) = grid.get(loc.left()) {
                    if let Some(Tile::Sand) = grid.get(loc.left().left()) {
                        return Some(loc.left().left());
                    }
                    return Some(loc);
                }

                loc = loc.left();
            }

        }

        // eprintln!("Before right {}", loc);


        while is(loc.up(), Tile::Flow, grid) {
            loc = loc.up();
        }

        // eprintln!("After right {}", loc);

        while is(loc.right(), Tile::Flow, grid) {
            loc = loc.right();
        }

        // eprintln!("After right {}", loc);

        if let Some(Tile::Sand) = grid.get(loc.right()) {
            return Some(loc.right());
            // return next_free_location(loc.right(), grid);
        }

        /*
        while is_not(loc.right(), Tile::Clay, grid) {
            if let Some(Tile::Sand) = grid.get(loc.right()) {
                return Some(loc.right());
            }

            loc = loc.right();
        }


        // Attempt going up a row
        if let Some(Tile::Flow) = grid.get(loc.up()) {
            return next_free_location(loc.up(), grid);
        }

        if let Some(Tile::Sand) = grid.get(loc.up()) {
            return next_free_location(loc.up(), grid);
        }
        */
    }

    /*
    if let Some((Tile::Sand, down)) = first_down(start, grid) {
        return Some(down);
    }

    if let Some((Tile::Sand, loc)) = first_left(start, grid) {
        if is_not(loc.down(), Tile::Flow, grid) {
            return Some(loc);
        }
    }

    if let Some((Tile::Sand, loc)) = first_right(start, grid) {
        if is_not(loc.down(), Tile::Flow, grid) {
            return Some(loc);
        }
    }

    if let Some((Tile::Clay, loc)) = first_left(start, grid) {
        if is(loc.down(), Tile::Sand, grid) {
            return Some(loc.down());
        }
    }

    if let Some((Tile::Clay, loc)) = first_right(start, grid) {
        if is(loc.down(), Tile::Sand, grid) {
            return Some(loc.down());
        }

        return next_free_location(loc.left().up(), grid);
    }
    */

    None
}



fn from_spring(springs: &mut Vec<Vector2>, grid: &Grid) -> Option<(Direction, Vector2)> {
    if let Some(_) = springs.pop() {
        // Attempt going down on this spring
        if let Some(sp) = springs.last() {
            let mut pos = *sp;

            while is(pos.down(), Tile::Flow, grid)
                && (is(pos.left(), Tile::Sand, grid) || is(pos.right(), Tile::Sand, grid))
            {
                pos = pos.down();
            }

            pos = pos.up();

            eprintln!("Found {}", pos);
            if let Some(Tile::Sand) = grid.get(pos.left()) {
                if !is_at_edge(pos, grid) {
                    return Some((Direction::Left, pos.left()));
                }
            }

            if let Some(Tile::Sand) = grid.get(pos.right()) {
                if !is_at_edge(pos, grid) {
                    return Some((Direction::Right, pos.right()));
                }
            }
        }
    }
    None
}

fn produce_water_2(spring: Vector2, grid: &mut Grid, waters: &mut Vec<Vector2>, springs: &mut Vec<Vector2>) -> bool {
    let start = waters.last().unwrap_or(&spring);
    let sp = springs.last().unwrap_or(&spring);

    if let Some((dir, loc)) = next_free_loc(*start, *sp, grid) {
        if dir == Direction::Down {
            springs.push(loc);
        }
        waters.push(loc);
        grid[loc] = Tile::Flow;
        true
    } else {

        if let Some((dir, loc)) = from_spring(springs, grid) {
            if dir == Direction::Down {
                springs.push(loc);
            }
            // if dir == Direction::Down {
            //     springs.push(loc);
            // }
            // waters.push(loc);
            // grid[loc] = Tile::Flow;
            // return true;
            waters.push(loc);
            grid[loc] = Tile::Flow;
            return true;
        }
        // eprintln!("No locations available");
        false
    }
}

fn produce_water(spring: Vector2, grid: &mut Grid, waters: &mut Vec<Vector2>) {

    for water in waters.iter_mut() {
        // Default for water is to try and go down
        // until it reaches clay
        let next_pos = *water + (0, 1);

        // Skip water that would move outside
        if !is_inside(next_pos, &grid) {
            continue;
        }

        let around = grid.get_around(*water);

        if around.down == Tile::Sand {
            *water = water.down();
            grid.set(*water, Tile::Flow);
        } else if around.down == Tile::Clay
            && around.current == Tile::Flow
            && around.left == Tile::Sand
        {
            *water = water.left();
            grid.set(*water, Tile::Rest);
        } else if around.down == Tile::Flow
            && around.current == Tile::Flow
        {
            *water = water.down();
            grid.set(*water, Tile::Flow);
        } else if around.down == Tile::Clay
            && around.current == Tile::Rest
            && (around.left == Tile::Sand || around.left == Tile::Rest)
        {
            *water = water.left();
            grid.set(*water, Tile::Rest);
        } else if around.down == Tile::Clay
            && around.current == Tile::Flow
            && around.left == Tile::Rest
            && (around.right == Tile::Rest || around.right == Tile::Clay)
        {
            grid.set(*water, Tile::Rest);
        }

        else {
            eprintln!("here {} with {:?}", *water, grid.get(*water));
        }
        /*
        if around.down == Tile::Sand || around.down == Tile::Flow {
        } else if around.down == Tile::Clay && around.current == Tile::Flow {
            *water = water.left();
            grid.set(*water, Tile::Rest);
            eprintln!("here {} with {:?}", *water, grid.get(*water));
        } else if around.down == Tile::Clay && (around.current == Tile::Rest || around.current == Tile::Flow) && around.left == Tile::Sand {
            *water = water.left();
            grid.set(*water, Tile::Rest);
            eprintln!("here {} with {:?}", *water, grid.get(*water));
        } else if around.down == Tile::Clay
                  && around.current == Tile::Rest
                  && around.left != Tile::Sand
                  && around.right == Tile::Sand
        {
            *water = water.right();
            grid.set(*water, Tile::Rest);
        } else if is_full_of_water(water.down(), grid) {
            eprintln!("FULL {}", water.down());
            grid.set(*water, Tile::Rest);
        } else {
            eprintln!("here {} with {:?}", *water, grid.get(*water));
        }
        */

        // if around[2] == Some(Tile::Sand) {
        //     *water = water.down();
        // } else if around[2] == Some(Tile::Clay) {
        //     grid.set(water, Tile::Rest);
        // }


        /*
        let (new_pos, new_tile) = match grid.get(next_pos).unwrap_or(Tile::Invalid) {
            Tile::Sand => {
                (next_pos, Tile::Flow),
            },
            Tile::Flow => {

            },
            Tile::Rest => {

            },

            Tile::Clay => {

            },

            Tile::Invalid => {
            },
        },
        */

        /*
        if is_tile(next_pos, Tile::Sand, &grid) {
            *water = next_pos;
            // grid[next_pos] = Tile::Flow;

            grid.set(next_pos, Tile::Flow);
        } else if is_tile(next_pos, Tile::Clay, &grid) {

            if is_tile(*water, Tile::Rest, &grid) {
                let left = *water + (-1, 0);
                *water = left;
                grid.set(left, Tile::Rest);
            } else {
                grid.set(water, Tile::Rest);
            }
        } else if is_tile(next_pos, Tile::Rest, &grid) {
            eprintln!("here");
            // push water out of the way
            let left = next_pos + (-1, 0);


            if is_inside(left, &grid) {
                if is_tile(left, Tile::Sand, &grid) {
                    grid[left] = Tile::Rest;
                    *water = left;
                }
            }
        }
        */
    }

    let water = spring + (0, 1);

    waters.push(water);

    grid[water] = Tile::Flow;
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