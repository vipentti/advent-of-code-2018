extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::error;
use std::fmt;

use std::result::Result as StdResult;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClaimError(String);

impl fmt::Display for ClaimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}


// This is important for other errors to wrap this one.
impl error::Error for ClaimError {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

type Grid = Vec<Vec<i32>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Claim {
    id: i32,
    rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rect {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn right(&self) -> i32 {
        self.left + self.width
    }

    fn bottom(&self) -> i32 {
        self.top + self.height
    }
}

fn main() -> Result<()> {
    let mut s = String::new();

    let mut file = File::open("input/day03.txt")?;

    file.read_to_string(&mut s)?;

    part1(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<()> {
    let regex = regex::Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)")?;

    let claims: Result<Vec<Claim>> = s.lines()
        .map(|s| parse_claim(s, &regex))
        .collect()
        ;

    let claims = claims?;

    let width = claims.iter().map(|c| c.rect.right())
        .max()
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid size".to_owned()).into())?
        ;

    let height = claims.iter().map(|c| c.rect.bottom())
        .max()
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid size".to_owned()).into())?
        ;

    eprintln!("Size {}x{}", width, height);

    for c in &claims[..10] {
        eprintln!("c: {:?}", c);
    }

    let mut grid = vec![vec![0_i32; width as usize]; height as usize];

    for claim in claims.iter() {
        mark(claim, &mut grid);
    }

    for row in grid.iter() {
        let mut s = String::new();
        for col in row.iter() {
            if *col == 0 {
                s.push('.');
            } else if *col == 1 {
                s.push('#');
            } else {
                let c = std::char::from_digit(*col as u32, 10)
                    .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid value".to_owned()).into())?
                    ;
                s.push(c);
            }
        }
        eprintln!("{}", s);
    }


    let results = grid.iter_mut()
        .map(|mut row| {
            let filtered = row.iter_mut()
                .filter(|v| **v < 2)
                .collect();
            filtered
        });


    Ok(())
}

fn mark(claim: &Claim, grid: &mut Grid) {

    for y in claim.rect.top..claim.rect.bottom() {
        let row = grid.get_mut(y as usize).expect(&format!("row: {}", y));
        for x in claim.rect.left..claim.rect.right() {
            row[x as usize] += 1;
        }
    }

}

fn parse_claim(s: &str, re: &regex::Regex) -> StdResult<Claim, Box<std::error::Error>> {
    let caps = re.captures(s)
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid capture".to_owned()).into())?;

    let id = caps
        .get(1)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid id".to_owned()).into())?;
        ;

    let left = caps
        .get(2)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid left".to_owned()).into())?;
        ;

    let top = caps
        .get(3)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid top".to_owned()).into())?;
        ;

    let width = caps
        .get(4)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid width".to_owned()).into())?;
        ;

    let height = caps
        .get(5)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| ClaimError("Invalid height".to_owned()).into())?;
        ;

    let rect = Rect {
        left, top, width, height
    };

    Ok(Claim {
        id,
        rect,
    })
}