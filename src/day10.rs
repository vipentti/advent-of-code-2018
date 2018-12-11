use aoc::{Result, CustomError};
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, Index, IndexMut};
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::From;
use std::str::FromStr;
use std::collections::{HashMap, BTreeMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Light {
    position: Vector2,
    velocity: Vector2,
}

impl FromStr for Light {
    type Err = CustomError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"position=<([\d\s\-]+), ([\d\s\-]+)> velocity=<([\d\s\-]+), ([\d\s\-]+)>"
            ).unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| CustomError("Invalid captures".to_owned()))?;

        let pos_x = aoc::get_value(&caps, 1)?;
        let pos_y = aoc::get_value(&caps, 2)?;

        let vel_x = aoc::get_value(&caps, 3)?;
        let vel_y = aoc::get_value(&caps, 4)?;

        let position = (pos_x, pos_y).into();
        let velocity = (vel_x, vel_y).into();

        Ok(Light {
            position,
            velocity,
        })
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from(v: (i32, i32)) -> Self {
        Vector2 {
            x: v.0,
            y: v.1,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: i32) -> Self {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s, 20000)?;
    // part2(&s)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Grid<T: fmt::Display + Clone + Default> {
    // grid: Vec<Vec<T>>,
    grid: Vec<T>,
    init: T,
    width: usize,
    height: usize,
}

struct VirtualGrid {
    values: BTreeMap<Vector2, char>,
    width: usize,
    height: usize,
}

impl VirtualGrid {
    pub fn new_with(width: usize, height: usize) -> Self {
        VirtualGrid {
            values: BTreeMap::new(),
            width,
            height,
        }
    }

    pub fn reset(&mut self) {
        self.values.clear();
    }

    pub fn set(&mut self, point: Vector2) {
        self.values.insert(point, '#');
    }

    pub fn get(&self, index: &Vector2) -> Option<&char> {
        self.values.get(index)
    }

    pub fn get_keys(&self) -> Vec<Vector2> {
        self.values.keys().cloned().collect()
    }

    pub fn as_string(&self) -> String {

        let mut keys: Vec<_> = self.values.keys().cloned().collect();

        let (min_x, min_y, max_x, max_y) = get_size_from(&keys[..]).unwrap();
        let size_x = (max_x - min_x).abs() as usize + 1;
        let size_y = (max_y - min_y).abs() as usize + 1;

        eprintln!("Size {}x{}", size_x, size_y);
        eprintln!("Min {}x{}", min_x, min_y);
        eprintln!("Max {}x{}", max_x, max_y);

        for key in keys.iter_mut() {
            *key -= (min_x, min_y).into();
        }

        let mut grid = Grid::new_with('.', size_x, size_y);

        for key in keys.iter() {
            grid[*key] = '#';
        }

        grid.as_string()
    }
}

impl<T: fmt::Display + Clone + Default> Grid<T> {
    pub fn new_with(init: T, width: usize, height: usize) -> Self {
        Grid {
            init: init.clone(),
            grid: vec![init; width * height],
            width,
            height,
            // grid: vec![vec![init; width]; height],
        }
    }

    pub fn reset(&mut self) {
        for row in self.grid.iter_mut() {
            *row = self.init.clone();
            // for col in row.iter_mut() {
            // }
        }
    }

    pub fn as_string(&self) -> String {
        let mut fin = String::new();

        fin.push(' ');
        fin.push(' ');
        fin.push(' ');

        for x in 0..self.width {
            fin.push_str(&format!("{}", x % 10))
        }
        fin.push('\n');

        for y in 0..self.height {
            fin.push_str(&format!("{:^3}", y));
            let mut output = String::new();

            for x in 0..self.width {
                output.push_str(&format!("{}", self.grid[y * self.width + x]))
            }
            fin.push_str(&output);
            fin.push('\n');
        }

        fin
    }

    pub fn get(&self, index: Vector2) -> Option<&T> {
        let i = index.y as usize * self.width + index.x as usize;
        self.grid.get(i)
    }

    pub fn get_mut(&mut self, index: Vector2) -> Option<&mut T> {
        let i = index.y as usize * self.width + index.x as usize;
        self.grid.get_mut(i)
    }
}

fn debug_grid(orig: &[Vector2]) {
    if orig.is_empty() {
        return;
    }

    let mut points: Vec<_> = orig.iter().cloned().collect();

    let (min_x, min_y, max_x, max_y) = get_size_from(&points[..]).unwrap();
    let size_x = (max_x - min_x).abs() as usize + 1;
    let size_y = (max_y - min_y).abs() as usize + 1;

    eprintln!("Size {}x{}", size_x, size_y);
    eprintln!("Min {}x{}", min_x, min_y);
    eprintln!("Max {}x{}", max_x, max_y);

    for key in points.iter_mut() {
        *key -= (min_x, min_y).into();
    }

    let mut grid = Grid::new_with('.', size_x, size_y);

    for key in points.iter() {
        grid[*key] = '#';
    }

    let s = grid.as_string();

    eprintln!("{}", s);
}

// Just for convenience, so that we can type `self[i]` instead of `self.nodes[i]`.
impl<T: fmt::Display + Clone + Default> Index<Vector2> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {
        // &self.grid[index.y as usize][index.x as usize]
        &self.grid[index.y as usize * self.width + index.x as usize]
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.nodes[i]`.
impl<T: fmt::Display + Clone + Default> IndexMut<Vector2> for Grid<T> {
    fn index_mut(&mut self, index: Vector2) -> &mut T {
        //&mut self.grid[index.y as usize][index.x as usize]
        &mut self.grid[index.y as usize * self.width + index.x as usize]
    }
}

fn get_size_from(points: &[Vector2]) -> Result<(i32, i32, i32, i32)>
{
    let min_x = points.iter().map(|c| c.x).min()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing min_x".to_string()).into()
        })?;

    let min_y = points.iter().map(|c| c.y).min()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing min_y".to_string()).into()
        })?;

    let max_x = points.iter().map(|c| c.x).max()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing max_x".to_string()).into()
        })?;

    let max_y = points.iter().map(|c| c.y).max()
        .ok_or_else::<Box<CustomError>, _>(|| {
            CustomError("Missing max_y".to_string()).into()
        })?;

    Ok((min_x, min_y, max_x, max_y))
}

fn get_size(lights: &[Light], max_ticks: i32) -> Result<(i32, i32, i32, i32)> {
    let mut points: Vec<Vector2> = lights
        .iter()
        .map(|c| c.position)
        .collect();

    points.extend(lights
        .iter()
        .map(|c| c.position + (c.velocity * max_ticks))
    );

    get_size_from(&points[..])
}


#[derive(Debug, Clone)]
struct LetterMap {
    data: BTreeMap<char, Vec<Vec<Vector2>>>,
}

impl LetterMap {
    pub fn new_from(data: BTreeMap<char, Vec<Vec<Vector2>>>) -> Self {
        LetterMap {
            data,
        }
    }

    pub fn contains(&self, grid: &VirtualGrid) -> bool {
        let mut positions = grid.get_keys();

        let mut out = String::new();

        let mut map = BTreeMap::new();

        'outer: loop {
            let mut any_found = false;
            let mut possible_match = BTreeMap::new();

            if let Some((first, rest)) = positions.split_first().map(|(v, r)| (v.clone(), r.to_vec())) {

                for (k, vs) in self.data.iter() {
                    for list in vs.iter() {
                        let transformed: Vec<_> = list.iter()
                            .cloned()
                            .map(|c| c + first.clone())
                            .collect();

                        let all_chars = transformed.iter()
                            .map(|pos| grid.get(&pos))
                            .all(|c| match c {
                                Some(ch) => *ch == '#',
                                None => false,
                            });

                        if all_chars {
                            eprintln!("Found possible match {} {:?}", k, first);

                            possible_match.insert(*k, transformed.clone());
                            break;
                        }
                    }
                }

                let max = possible_match
                    .iter()
                    .max_by_key(|(_, v)| v.len());

                if let Some((k, values)) = max {
                    eprintln!("Found match {} {:?}", k, values);
                    map.insert(first.clone(), *k);
                    positions = rest.iter()
                        .filter(|&v| !values.contains(v))
                        .cloned()
                        .collect();

                    out.push(*k);
                    // break 'outer;
                } else {
                    positions = rest;
                }

            } else {
                break;
            }

            /*

            for (k, vs) in self.data.iter() {
                for list in vs.iter() {

                    let new_pos = {
                        let mut new_stuff = positions.clone();
                        for pos in positions.iter() {
                            let transformed: Vec<_> = list.iter()
                                .cloned()
                                .map(|c| c + pos.clone())
                                .collect();

                            let all_chars = transformed.iter()
                                .map(|pos| grid.get(&pos))
                                .all(|c| match c {
                                    Some(ch) => *ch == '#',
                                    None => false,
                                });

                            if all_chars {
                                any_found = true;
                                eprintln!("Found match {} {:?} {:?}", k, pos, list);

                                map.insert(*pos, *k);

                                possible_match.insert(*k, transformed.clone());

                                out.push(*k);
                                let before = positions.len();

                                new_stuff = positions.iter()
                                    .filter(|v| !transformed.contains(v))
                                    .cloned()
                                    .collect();

                                let after = new_stuff.len();

                                eprintln!("{} -> {}", before, after);

                                break;
                            }
                        }
                        new_stuff
                    };

                    positions = new_pos;
                }
            }

            if !any_found {
                break;
            }
            */
        }

        if !out.is_empty() {
            debug_grid(&positions[..]);
            eprintln!("{:?}", map);
            eprintln!("{}", out);
            true
        } else {
            false
        }
    }

    pub fn matches(&self, pos: &Vector2, grid: &VirtualGrid) -> Option<char> {

        for (k, vs) in self.data.iter() {
            for list in vs.iter() {
                let all_chars = list.iter()
                    .cloned()
                    .map(|c| c + pos.clone())
                    .map(|pos| grid.get(&pos))
                    .all(|c| match c {
                        Some(ch) => *ch == '#',
                        None => false,
                    });

                if all_chars {
                    eprintln!("Found match {} {:?} {:?}", k, pos, list);
                    return Some(*k);
                }
            }
        }

        None
    }
}

fn read_letters() -> Result<LetterMap> {
    let letters = aoc::read_file("input/day10_letters.txt")?;

    let mut chars = letters.char_indices().peekable();
    let mut line = 0;
    let mut current = 0;

    let mut letter_index = 0;

    let letters = vec![
        'A',
        'B',
        'C',
        'E',
        'F',
        'G',
        'H',
        'H',
        'I',
        'J',
        'K',
        'L',
        'N',
        'P',
        'R',
        'X',
        'Z',
    ];

    let mut map: BTreeMap<char, Vec<Vec<Vector2>>> = BTreeMap::new();

    while let Some((_ind, ch)) = chars.next() {
        match ch {
            '#' => {
                let vec: Vector2 = (current as i32, line as i32).into();
                let letter = letters[letter_index];
                // eprintln!("vec {} {:?}", letters[letter_index], vec);
                current += 1;

                let e = map.entry(letter)
                   .or_insert_with(|| vec![Vec::new()]);

                if let Some(ref mut last) = e.last_mut() {
                   last.push(vec);
                }
            }

            '\n' => {
                match chars.peek() {
                    Some((_, '\n')) => {
                        if letters[letter_index] == letters[letter_index + 1] {
                            map.entry(letters[letter_index])
                                .and_modify(|v| v.push(Vec::new()));
                        }

                        letter_index += 1;
                        line += 1;
                        current = 0;
                        chars.next();
                    }
                    _ => {
                        line += 1;
                        current = 0;
                    }
                }
            }

            _ => {
                current += 1;
            }
        }
    }

    // Normalize the values from absolute units
    // to relative units to the first point
    for (k, values) in map.iter_mut() {
        for list in values.iter_mut() {
            let first = list.first().cloned().unwrap();

            for val in list.iter_mut() {
                *val -= first;
            }
        }
    }

    // eprintln!("Map {:#?}", map);

    Ok(LetterMap::new_from(map))
}

fn part1(s: &str, max_ticks: i32) -> Result<String> {
    let letters = read_letters()?;

    let lights: Result<Vec<_>> = s.lines()
        .map(|v| v.parse::<Light>().map_err(|e| e.into()))
        .collect();

    let mut lights = lights?;

    let (min_x, min_y, max_x, max_y) = get_size(&lights, max_ticks)?;

    let size_x = (max_x - min_x).abs() as usize + 1;
    let size_y = (max_y - min_y).abs() as usize + 1;

    eprintln!("{}x{}", size_x, size_y);
    eprintln!("{}x{}", min_x, min_y);
    eprintln!("{}x{}", max_x, max_y);

    // let mut grid = vec![vec!['.'; size_x]; size_y];
    // let mut grid = Grid::new_with('.', size_x, size_y);
    let mut grid = VirtualGrid::new_with(size_x, size_y);

    // Normalize the coordinates from -X -> +X to 0..
    for light in lights.iter_mut() {
        light.position -= (min_x, min_y).into();
    }

    for light in &lights {
        grid.set(light.position);
        // grid[light.position] = '#';
    }

    // let fin = grid.as_string();

    // eprintln!("{}", fin);

    let mut out = String::new();

    let mut last_tick = 0;

    for tick in 0..max_ticks {
        grid.reset();

        for light in lights.iter_mut() {
            light.position += light.velocity;
            // grid[light.position] = '#';
            grid.set(light.position);
        }

        let mut found = false;

        if letters.contains(&grid) {
            found = true;
        }

        // for light in lights.iter() {
        //     if let Some(letter) = letters.matches(&light.position, &grid) {
        //         out.push(letter);
        //         found = true;
        //     }
        // }

        if found {
            eprintln!("TICK: {}", tick);
            let fin = grid.as_string();

            // Seconds start from 1
            last_tick = tick + 1;

            eprintln!("{}", fin);

            break;
        }
    }

    eprintln!("part1: {}", out);
    eprintln!("part2: {}", last_tick);

    Ok(out)
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
    ";

    #[test]
    fn part1_example_input() {
        assert_eq!("HI", part1(INPUT.trim(), 5).unwrap());
    }
}