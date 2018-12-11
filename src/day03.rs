extern crate regex;

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

use aoc::Result;

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
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<usize> {
    let regex = regex::Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)")?;

    let claims: Result<Vec<Claim>> = s
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| parse_claim(s, &regex))
        .collect();

    let claims = claims?;

    let width = claims
        .iter()
        .map(|c| c.rect.right())
        .max()
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid size".to_owned()).into()
        })?;

    let height = claims
        .iter()
        .map(|c| c.rect.bottom())
        .max()
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid size".to_owned()).into()
        })?;

    eprintln!("Size {}x{}", width, height);

    let mut grid = vec![vec![0_i32; width as usize]; height as usize];

    for claim in claims.iter() {
        mark(claim, &mut grid);
    }

    let result = grid
        .iter()
        .map(|row| row.iter().filter(|v| **v >= 2).count())
        .sum();

    eprintln!("total square inches {}", result);

    Ok(result)
}

fn part2(s: &str) -> Result<i32> {
    let regex = regex::Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)")?;

    let claims: Result<Vec<Claim>> = s
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| parse_claim(s, &regex))
        .collect();

    let claims = claims?;

    let width = claims
        .iter()
        .map(|c| c.rect.right())
        .max()
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid size".to_owned()).into()
        })?;

    let height = claims
        .iter()
        .map(|c| c.rect.bottom())
        .max()
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid size".to_owned()).into()
        })?;

    eprintln!("Size {}x{}", width, height);

    let mut grid = vec![vec![0_i32; width as usize]; height as usize];

    for claim in claims.iter() {
        mark(claim, &mut grid);
    }

    for claim in claims.iter() {
        if is_only_claimer(claim, &grid) {
            eprintln!("Claim: {:?}", claim);
            return Ok(claim.id);
        }
    }

    Ok(0)
}

#[allow(dead_code)]
fn display_grid(grid: &[Vec<i32>]) -> Result<()> {
    for row in grid.iter() {
        let mut s = String::new();
        for col in row.iter() {
            if *col == 0 {
                s.push('.');
            } else if *col == 1 {
                s.push('#');
            } else {
                let c = std::char::from_digit(*col as u32, 10)
                    .ok_or_else::<Box<ClaimError>, _>(|| {
                        ClaimError("Invalid value".to_owned()).into()
                    })?;
                s.push(c);
            }
        }
        eprintln!("{}", s);
    }
    Ok(())
}

fn mark(claim: &Claim, grid: &mut Vec<Vec<i32>>) {
    for y in claim.rect.top..claim.rect.bottom() {
        let row = &mut grid[y as usize];
        for x in claim.rect.left..claim.rect.right() {
            row[x as usize] += 1;
        }
    }
}

fn is_only_claimer(claim: &Claim, grid: &[Vec<i32>]) -> bool {
    for y in claim.rect.top..claim.rect.bottom() {
        let row = &grid[y as usize];
        for x in claim.rect.left..claim.rect.right() {
            if row[x as usize] > 1 {
                return false;
            }
        }
    }
    true
}

fn parse_claim(
    s: &str,
    re: &regex::Regex,
) -> StdResult<Claim, Box<std::error::Error>> {
    let caps = re.captures(s).ok_or_else::<Box<ClaimError>, _>(|| {
        ClaimError("Invalid capture".to_owned()).into()
    })?;

    let id = caps
        .get(1)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid id".to_owned()).into()
        })?;
        ;

    let left = caps
        .get(2)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid left".to_owned()).into()
        })?;
        ;

    let top = caps
        .get(3)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid top".to_owned()).into()
        })?;
        ;

    let width = caps
        .get(4)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid width".to_owned()).into()
        })?;
        ;

    let height = caps
        .get(5)
        .and_then(|v| v.as_str().parse::<i32>().ok())
        .ok_or_else::<Box<ClaimError>, _>(|| {
            ClaimError("Invalid height".to_owned()).into()
        })?;
        ;

    let rect = Rect {
        left,
        top,
        width,
        height,
    };

    Ok(Claim { id, rect })
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
        "#;

        assert_eq!(4, part1(input).unwrap());
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
        "#;

        assert_eq!(3, part2(input).unwrap());
    }
}
