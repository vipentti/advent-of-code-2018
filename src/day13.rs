use aoc::{Result};
use std::collections::HashSet;


fn main() -> Result<()> {
    let s = aoc::read_input_untrimmed()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct Location {
    // Y before X so we can sort these
    pub y: i32,
    pub x: i32,
}

impl PartialEq<(i32, i32)> for Location {
    fn eq(&self, other: &(i32, i32)) -> bool {
        let v: Location = (*other).into();
        *self == v
    }
}

impl std::convert::From<Location> for (i32, i32) {
    fn from(v: Location) -> Self {
        (v.x, v.y)
    }
}

impl std::convert::From<Location> for (usize, usize) {
    fn from(v: Location) -> Self {
        (v.x as usize, v.y as usize)
    }
}

impl std::convert::From<(i32, i32)> for Location {
    fn from(v: (i32, i32)) -> Self {
        Location { x: v.0, y: v.1 }
    }
}

trait AsChar: Sized {
    fn from_char(c: char) -> Option<Self>;
    fn as_char(&self) -> char;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    /// <
    Left,
    /// >
    Right,
    /// ^
    Up,
    /// v
    Down,
}

impl AsChar for Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Down, Turn::Left) => Direction::Right,

            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,

            (d, Turn::Straight) => d,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Turn {
    Left,
    Straight,
    Right
}

impl Turn {
    fn turn(self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

impl Default for Turn {
    fn default() -> Self {
        Turn::Left
    }
}

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct Cart {
    location: Location,
    direction: Direction,
    next_turn: Turn,
    collision: bool,
}
use std::cmp::Ordering;


impl Cart {

    fn turn(&mut self) {
        let next_dir = self.direction.turn(self.next_turn);
        self.next_turn = self.next_turn.turn();
        self.direction = next_dir;
    }

    fn r#move(&mut self) {
        match self.direction {
            Direction::Left => {
                self.location.x -= 1;
            },
            Direction::Right => {
                self.location.x += 1;
            },
            Direction::Up => {
                self.location.y -= 1;
            },
            Direction::Down => {
                self.location.y += 1;
            },
        }
    }

    fn turn_track(&mut self, track: Track) {
        match track {
            Track::Intersection => {
                self.turn();
            },
            Track::StraightHorizontal => { },
            Track::StraightVertical => { },
            Track::CurveLeft => {
                let next_dir = match self.direction {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                };

                self.direction = next_dir;
            },
            Track::CurveRight => {
                let next_dir = match self.direction {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };

                self.direction = next_dir;
            },
            Track::Empty => { },
        }
    }
}


impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.location.cmp(&other.location)
    }
}

impl Eq for Cart { }

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.location == other.location
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Track {
    /// -
    StraightHorizontal,
    /// |
    StraightVertical,
    /// \
    CurveLeft,
    /// /
    CurveRight,
    /// +
    Intersection,
    /// ' '
    Empty,
}

impl Default for Track {
    fn default() -> Self {
        Track::Empty
    }
}

impl AsChar for Track {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+'  => Some(Track::Intersection),
            '\\' => Some(Track::CurveLeft),
            '/'  => Some(Track::CurveRight),
            '-'  => Some(Track::StraightHorizontal),
            '|'  => Some(Track::StraightVertical),
            ' '  => Some(Track::Empty),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Track::StraightHorizontal => '-',
            Track::StraightVertical => '|',
            Track::CurveLeft => '\\',
            Track::CurveRight => '/',
            Track::Intersection => '+',
            Track::Empty => ' ',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Grid {
    tracks: Vec<Track>,
    carts: Vec<Cart>,
    width: usize,
    height: usize,
}

fn to_index(width: usize, v: (usize, usize)) -> usize {
    v.1 * width + v.0
}

impl Grid {
    pub fn new(mut tracks: Vec<Vec<Track>>, mut carts: Vec<Cart>) -> Self {
        let max_width = tracks.iter()
            .map(|v| v.len())
            .max()
            .unwrap()
            ;

        for t in tracks.iter_mut() {
            if t.len() < max_width {
                let diff = max_width - t.len();
                t.extend((0..diff).map(|_| Track::Empty));
            }
        }

        for t in tracks.iter() {
            assert_eq!(max_width, t.len());
        }

        let height = tracks.len();

        carts.sort();

        Grid {
            tracks: tracks.into_iter().flatten().collect(),
            carts,
            width: max_width,
            height,
        }
    }

    fn as_index(&self, v: (usize, usize)) -> usize {
        to_index(self.width, v)
    }

    fn get_colliding_ids(&self) -> HashSet<usize> {
        let mut ids = HashSet::new();

        let len = self.carts.len();
        for i in 0..len {
            let ith = self.carts[i];
            for j in (i + 1)..len {
                let jth = self.carts[j];

                if ith.location == jth.location {
                    ids.insert(i);
                    ids.insert(j);
                }
            }
        }

        ids
    }

    fn get_all_collisions(&self) -> Vec<(i32, i32)> {
        let mut collisions = Vec::new();
        let len = self.carts.len();
        for i in 0..len {
            let ith = self.carts[i];
            for j in (i + 1)..len {
                let jth = self.carts[j];

                if ith.location == jth.location {
                    collisions.push(ith.location.into());
                }
            }
        }

        collisions
    }

    fn check_for_collisions(&mut self) -> Option<(i32, i32)> {

        let len = self.carts.len();

        let mut first = 0;
        let mut second = 0;
        let mut collision = false;

        'outer: for i in 0..len {
            let ith = self.carts[i];
            for j in (i + 1)..len {
                let jth = self.carts[j];

                if ith.location == jth.location {
                    first = i;
                    second = j;
                    collision = true;
                    break 'outer;
                }
            }
        }

        if collision {
            {
                let mut f = &mut self.carts[second];

                f.collision = true;
            }
            let l = {
                let mut f = &mut self.carts[first];

                f.collision = true;

                f.location
            };

            return Some(l.into());
        }


        None
    }

    fn tick_part2(&mut self) -> Option<(i32, i32)> {
        let w = self.width;
        let len = self.carts.len();

        for index in 0..len {
            {
                let c = self.carts[index];
                // Skip colliding ones
                if c.collision {
                    continue;
                }
            }
            {
                let cart = &mut self.carts[index];
                cart.r#move();
            }
            {
                let ids = self.get_colliding_ids();

                for id in ids {
                    self.carts[id].collision = true;
                }
            }
            {
                let cart = &mut self.carts[index];
                // Get the track under the new location
                let i = to_index(w, cart.location.into());

                let track = self.tracks[i];

                cart.turn_track(track);
            }
        }

        let carts = std::mem::replace(&mut self.carts, Vec::with_capacity(len));

        self.carts.extend(carts.into_iter()
            .filter(|v| !v.collision)
        );

        self.carts.sort();

        if self.carts.len() == 1 {
            return Some(self.carts.first().unwrap().location.into());
        }

        None
    }

    fn tick(&mut self) {
        let w = self.width;
        let len = self.carts.len();

        for index in 0..len {
            {
                let cart = &mut self.carts[index];
                cart.r#move();
            }
            {
                let collisions = self.get_all_collisions();
                if !collisions.is_empty() {
                    return;
                }
            }
            {
                let cart = &mut self.carts[index];
                // Get the track under the new location
                let i = to_index(w, cart.location.into());

                let track = self.tracks[i];

                cart.turn_track(track);
            }
        }

        self.carts.sort();
    }

    fn draw(&self) -> String {
        let mut output = String::new();

        let mut vec: Vec<_> = self.tracks.iter().map(Track::as_char).collect();

        for c in self.carts.iter() {
            let i = self.as_index(c.location.into());

            if !c.collision {
                vec[i] = c.direction.as_char();
            } else {
                vec[i] = 'X';
            }
        }

        let mut row = String::new();
        // First draw out tracks
        for y in 0..self.height {
            row.clear();
            row.push_str(&format!("{: >3} ", y));
            for x in 0..self.width {
                let i = self.as_index((x, y));
                row.push(vec[i]);
                // row.push(self.tracks[i].as_char());
            }

            output.push_str(&row);
            output.push('\n');
        }

        output
    }
}

fn read_grid(s: &str) -> Result<Grid> {

    eprintln!("{}", s);

    let mut line: i32 = 0;
    let mut column: i32 = 0;

    let mut chars = s.char_indices();

    let mut tracks = Vec::new();
    let mut carts = Vec::new();
    let mut track_row = Vec::new();

    while let Some((_, ch)) = chars.next() {
        match (Track::from_char(ch), Direction::from_char(ch)) {
            (Some(t), None) => {
                track_row.push(t);
            },
            (None, Some(d)) => {
                match d {
                    Direction::Left => track_row.push(Track::StraightHorizontal),
                    Direction::Right => track_row.push(Track::StraightHorizontal),
                    Direction::Up => track_row.push(Track::StraightVertical),
                    Direction::Down => track_row.push(Track::StraightVertical),
                }
                carts.push(Cart {
                    location: (column, line).into(),
                    direction: d,
                    next_turn: Turn::Left,
                    collision: false,
                });
            },
            _ => match ch {
                '\n' => {
                    line += 1;
                    column = 0;
                    tracks.push(track_row);
                    track_row = Vec::new();
                    continue;
                },
                _ => {
                    unreachable!();
                }
            }
        }

        column += 1;
    }

    if !track_row.is_empty() {
        tracks.push(track_row);
    }

    Ok(Grid::new(tracks, carts))
}


fn part1(s: &str) -> Result<(i32, i32)> {
    let mut grid = read_grid(s)?;

    eprintln!("{}", grid.draw());

    let mut tick = 0;
    let res: Option<(i32, i32)> = loop {
        grid.tick();

        if let Some(p) = grid.check_for_collisions() {
            eprintln!("{} Collision {:?}", tick, p);
            break Some(p);
        }

        tick += 1;

        if tick > 1_000_000 {
            break None;
        }
    };

    eprintln!("{}", grid.draw());

    if let Some(p) = res {
        eprintln!("part1: {:?}", p);
        Ok(p)
    } else {
        Ok((0, 0))
    }

}

fn part2(s: &str) -> Result<(i32, i32)> {
    let mut grid = read_grid(s)?;

    eprintln!("{}", grid.draw());

    let mut tick = 0;

    let mut res: Option<(i32, i32)> = loop {
        if let Some(pos) = grid.tick_part2() {
            eprintln!("{} -> {:?}", tick, pos);
            break Some(pos);
        }

        tick += 1;

        if tick > 1_000_000 {
            break None;
        }
    };

    eprintln!("{}", grid.draw());

    if let Some(p) = res {
        eprintln!("part2: {:?}", p);
        Ok(p)
    } else {
        Ok((0, 0))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r"
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
    ";

    const PART2_INPUT: &'static str = r"
/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

    #[test]
    fn part1_example_input() {
        assert_eq!((7, 3), part1(INPUT.trim()).unwrap());
    }

    #[test]
    fn part2_example_input() {
        assert_eq!((6, 4), part2(PART2_INPUT.trim()).unwrap());
    }
}
