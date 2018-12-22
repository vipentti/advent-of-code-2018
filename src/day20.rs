
#![allow(dead_code)]
use aoc::{Result, CustomError, Vector2, ToIndex};

use std::ops::{Index, IndexMut};

use std::collections::{VecDeque, BTreeMap, HashMap};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PathRegex {
    Blank,
    Step(Dir),
    Branch(Box<PathRegex>, Box<PathRegex>),
    List(Vec<PathRegex>),
}

impl PathRegex {
    fn is_optional_branch(&self) -> bool {
        match self {
            PathRegex::Branch(_, p) if **p == PathRegex::Blank => true,
            _ => false,
        }
    }
}

struct Parser {
    input: Vec<char>,
    current: usize,
}

impl Parser {
    pub fn new(input: Vec<char>) -> Self {
        Parser {
            input,
            current: 0,
        }
    }

    fn parse_all(&mut self) -> Result<PathRegex> {
        let mut results = Vec::new();

        while self.has_more() {
            let r = self.parse()?;
            results.push(r);
        }

        Ok(PathRegex::List(results))
    }

    fn parse(&mut self) -> Result<PathRegex> {
        match self.peek() {
            '(' => {
                self.eat('(')?;

                let inner = self.regex()?;

                self.eat(')')?;

                Ok(inner)
            },

            'N' => { self.eat('N')?; Ok(PathRegex::Step(Dir::North)) }
            'E' => { self.eat('E')?; Ok(PathRegex::Step(Dir::East)) }
            'S' => { self.eat('S')?; Ok(PathRegex::Step(Dir::South)) }
            'W' => { self.eat('W')?; Ok(PathRegex::Step(Dir::West)) }

            _ => unreachable!(),
        }
    }

    fn regex(&mut self) -> Result<PathRegex> {
        let term = self.term()?;

        if self.has_more() && self.peek() == '|' {
            // New option
            self.eat('|')?;
            let alt = self.regex()?;
            Ok(PathRegex::Branch(Box::new(term), Box::new(alt)))
        } else {
            Ok(term)
        }
    }

    fn term(&mut self) -> Result<PathRegex> {
        let blank = PathRegex::Blank;

        let mut list: Vec<PathRegex> = Vec::new();

        while self.has_more() && self.peek() != ')' && self.peek() != '|' {
            let next = self.factor()?;

            // Factor = sequence
            list.push(next)
        }

        if list.is_empty() {
            Ok(blank)
        } else {
            Ok(PathRegex::List(list))
        }
    }

    fn factor(&mut self) -> Result<PathRegex> {
        let base = self.parse()?;

        Ok(base)
    }

    fn has_more(&self) -> bool {
        self.current < self.input.len()
    }

    fn peek(&self) -> char {
        self.input[self.current]
    }

    fn next(&mut self) -> Result<char> {
        let c = self.peek();
        self.eat(c)?;
        Ok(c)
    }

    fn eat(&mut self, c: char) -> Result<()> {
        if self.peek() == c {
            // Move from current
            self.current += 1;
            return Ok(());
        }
        Err(CustomError(format!("Invalid character {} at {} expected {}", self.peek(), self.current, c)).into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Tile {
    Wall,
    Open,
    DoorUp,
    DoorSide,
    Start,
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Open => '.',
            Tile::DoorSide => '|',
            Tile::DoorUp => '-',
            Tile::Start => 'X',
        }
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Wall }
}

type TileMap = BTreeMap<Vector2, Tile>;

fn part1(s: &str) -> Result<usize> {
    let path_chars: Vec<char> = s.chars()
        .filter(|&c| c != '^' && c != '$')
        .collect();

    // eprintln!("C {:?}", path_chars);

    let mut parser = Parser::new(path_chars);

    let path = parser.parse_all()?;

    // eprintln!("Path {:?}", path);

    let mut coords: Vec<Vector2> = Vec::new();

    let mut start = (0, 0).into();

    coords.push(start);
    let mut tilemap: TileMap = BTreeMap::new();
    tilemap.insert(start, Tile::Start);

    traverse(&path, &mut start, &mut coords, &mut tilemap);

    // eprintln!("Coords {:?}", coords);

    let mut points: Vec<_> = tilemap.keys().cloned().collect();

    let (mut min_x, mut min_y, mut max_x, mut max_y) = get_size_from(&points)?;
    // min_x -= 1;
    // min_y -= 1;
    // max_x += 1;
    // max_y += 1;

    let size_x = (max_x - min_x).abs() as usize + 1;
    let size_y = (max_y - min_y).abs() as usize + 1;
    eprintln!("Size {}x{}", size_x, size_y);
    eprintln!("Min {}x{}", min_x, min_y);
    eprintln!("Max {}x{}", max_x, max_y);
    // eprintln!("{:?}", tilemap);

    // for pt in points.iter_mut() {
    //     *pt -= (min_x, min_y);
    // }

    // eprintln!("Coords {:?}", points);

    let mut grid = Grid::new(size_x, size_y);

    for (key, val) in tilemap.iter() {
        let coord = *key - (min_x, min_y);
        grid[coord] = *val;
    }

    grid.display();

    let start_x = (size_x - 1) / 2;
    let start_y = (size_y - 1) / 2;

    let s = start - (min_x, min_y);
    eprintln!("s {}", s);

    if let Some(dist) =  grid.find_path(s) {
        eprintln!("part1 {}", dist);
        return Ok(dist);
    }

    // grid.find_path(start + (max_x, max_y));



    Ok(0)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Grid {
    data: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            data: vec![Default::default(); width * height],
            width,
            height
        }
    }

    fn find_path(&self, start: Vector2) -> Option<usize> {
        let mut q: VecDeque<Vector2> = VecDeque::new();
        q.push_back(start);

        let mut distances: BTreeMap<Vector2, usize> = BTreeMap::new();
        distances.insert(start, 0);

        let mut doors: BTreeMap<Vector2, usize> = BTreeMap::new();
        doors.insert(start, 0);

        let mut came_from: BTreeMap<Vector2, Vector2> = BTreeMap::new();

        while !q.is_empty() {
            let current = q.pop_front().unwrap();

            for nbr in current.around().into_iter() {
                // Skip unwalkable
                if !self.can_walk(*nbr) {
                    continue;
                }

                if !distances.contains_key(nbr) {
                    q.push_back(*nbr);
                    distances.insert(*nbr, 1 + distances.get(&current).unwrap());
                    if self.is_door(*nbr) {
                        doors.insert(*nbr, 1 + doors.get(&current).unwrap());
                    } else {
                        doors.insert(*nbr, *doors.get(&current).unwrap());
                    }
                    came_from.insert(*nbr, current);
                } else {
                    let old_door = *doors.get(nbr).unwrap();
                    let current_doors = *doors.get(&current).unwrap();
                    let old_dist = *distances.get(nbr).unwrap();
                    let cur_dist = *distances.get(&current).unwrap();

                    if current_doors + 1 > old_door && cur_dist + 1 < old_dist {
                        eprintln!("Updating!! {}", nbr);
                        doors.insert(*nbr, current_doors + 1);
                        distances.insert(*nbr, cur_dist);
                        came_from.insert(*nbr, current);
                    }
                }
            }
        }

        // eprintln!("{:?}", distances);

        let max = distances.iter()
            .max_by_key(|(_, v)| *v)
            ;
        if let Some(m) = max {
            let d = doors.get(&m.0);
            eprintln!("Max {:?} {:?}", m, d);
            if let Some(dist) = d {
                return Some(*dist);
            }
        }
        None
    }

    fn is_door(&self, pt: Vector2) -> bool {
        match self.get(pt) {
            Some(Tile::DoorSide) => true,
            Some(Tile::DoorUp) => true,
            _ => false,
        }
    }

    fn is_open(&self, pt: Vector2) -> bool {
        match self.get(pt) {
            Some(Tile::Open) => true,
            _ => false,
        }
    }

    fn can_walk(&self, pt: Vector2) -> bool {
        match self.get(pt) {
            Some(Tile::Open) => true,
            Some(Tile::DoorSide) => true,
            Some(Tile::DoorUp) => true,

            _ => false,
        }
    }

    fn get<T: ToIndex>(&self, p: T) -> Option<&Tile> {
        let i = p.to_index(self.width);
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

fn adjacent(pt: Vector2) -> [Vector2; 8] {
    [ pt.left()
    , pt.left().up()
    , pt.up()
    , pt.up().right()
    , pt.right()
    , pt.right().down()
    , pt.down()
    , pt.down().left()
    ]
}

fn insert_adjacent(c: Vector2, tile: Tile, chars: &mut TileMap) {
    for pt in adjacent(c).iter() {
        if !chars.contains_key(pt) {
            chars.insert(*pt, Tile::default());
        }
    }
    chars.insert(c, tile);
}

fn traverse(path: &PathRegex, current: &mut Vector2, coords: &mut Vec<Vector2>, chars: &mut TileMap) {
    use self::PathRegex::*;

    // eprintln!("Step {:?} {}", path, current);
    // if path.is_optional_branch() {
    //     eprintln!("Optional {:?}", path);
    //     return;
    // }

    match path {
        Blank => { }
        Step(Dir::North) => {
            *current = current.up();
            insert_adjacent(*current, Tile::DoorUp, chars);
            *current = current.up();
            insert_adjacent(*current, Tile::Open, chars);
            coords.push(*current);
        },
        Step(Dir::East) => {
            *current = current.right();
            insert_adjacent(*current, Tile::DoorSide, chars);
            *current = current.right();
            insert_adjacent(*current, Tile::Open, chars);
            coords.push(*current);
        },
        Step(Dir::West) => {
            *current = current.left();
            insert_adjacent(*current, Tile::DoorSide, chars);
            *current = current.left();
            insert_adjacent(*current, Tile::Open, chars);
            coords.push(*current);
        },
        Step(Dir::South) => {
            *current = current.down();
            insert_adjacent(*current, Tile::DoorUp, chars);
            *current = current.down();
            insert_adjacent(*current, Tile::Open, chars);
            coords.push(*current);
        },

        Branch(left, right) => {

            let mut r = *current;
            let mut l = *current;

            traverse(&left, &mut l, coords, chars);
            traverse(&right, &mut r, coords, chars);
        }

        List(points) => {
            let mut c = *current;

            for pt in points.iter() {
                let mut p = c;
                traverse(pt, &mut p, coords, chars);
                c = p;
            }
        },
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Dir {
    North,
    West,
    East,
    South,
}

impl Dir {
    fn as_char(&self) -> char {
        match self {
            Dir::North => 'N',
            Dir::West => 'W',
            Dir::East => 'E',
            Dir::South => 'S',
        }
    }
}

impl Default for Dir {
    fn default() -> Self { Dir::North }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {

//         assert_eq!(3, part1(r"
// #####
// #.|.#
// #-###
// #.|X#
// #####
//         ".trim()).unwrap());
        assert_eq!(3, part1(r"^WNE$".trim()).unwrap());
        assert_eq!(18, part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").unwrap());
        assert_eq!(10, part1("^ENWWW(NEEE|SSE(EE|N))$").unwrap());

    }
}