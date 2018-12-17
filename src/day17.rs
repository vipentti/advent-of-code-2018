use aoc::{Result, Vector2, CustomError};

use regex::Regex;
use lazy_static::lazy_static;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    // part2(&s)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Tile {
    Sand,
    Clay,
    Spring,
    FlowingWater,
    RestingWater,
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Clay => '#',
            Tile::Sand => '.',
            Tile::Spring => '+',
            Tile::FlowingWater => '|',
            Tile::RestingWater => '~',
        }
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Sand }
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
    eprintln!("clay {:?}", clay_locations);

    let mut grid = vec![vec![Tile::default(); size_x]; size_y];

    for clay in clay_locations.iter() {
        grid[clay.y as usize][clay.x as usize] = Tile::Clay;
    }

    grid[spring.y as usize][spring.x as usize] = Tile::Spring;

    display_grid(&grid);

    let mut waters = Vec::new();

    produce_water(spring, &mut grid, &mut waters);
    produce_water(spring, &mut grid, &mut waters);
    produce_water(spring, &mut grid, &mut waters);
    produce_water(spring, &mut grid, &mut waters);
    produce_water(spring, &mut grid, &mut waters);
    produce_water(spring, &mut grid, &mut waters);

    display_grid(&grid);

    produce_water(spring, &mut grid, &mut waters);

    display_grid(&grid);
    produce_water(spring, &mut grid, &mut waters);
    display_grid(&grid);

    Ok(0)
}

fn is_inside(v: Vector2, grid: &[Vec<Tile>]) -> bool {
    (v.x >= 0 && v.x < grid[0].len() as i32)
    && (v.y >= 0 && v.y < grid.len() as i32)
}

fn is_tile(v: Vector2, expected: Tile, grid: &[Vec<Tile>]) -> bool {
    grid[v.y as usize][v.x as usize] == expected
}

fn produce_water(spring: Vector2, grid: &mut Vec<Vec<Tile>>, waters: &mut Vec<Vector2>) {

    for water in waters.iter_mut() {
        let next_pos = *water + (0, 1);

        // Skip water that would move outside
        if !is_inside(next_pos, &grid) {
            continue;
        }

        if is_tile(next_pos, Tile::Sand, &grid) {
            *water = next_pos;
            grid[next_pos.y as  usize][next_pos.x as usize] = Tile::FlowingWater;
        } else if is_tile(next_pos, Tile::Clay, &grid) {
            grid[water.y as  usize][water.x as usize] = Tile::RestingWater;
        } else if is_tile(next_pos, Tile::RestingWater, &grid) {
            // push water out of the way
            let left = *water + (-1, 0);


            if is_inside(left, &grid) {
                if is_tile(left, Tile::Sand, &grid) {
                    grid[left.y as  usize][left.x as usize] = Tile::RestingWater;
                    *water = left;
                }
            }
        }
    }

    let water = spring + (0, 1);

    waters.push(water);

    grid[water.y as  usize][water.x as usize] = Tile::FlowingWater;
}

fn display_grid(grid: &[Vec<Tile>]) {
    let mut buf = String::new();

    for row in grid.iter() {
        for col in row.iter() {
            buf.push(col.as_char());
        }
        buf.push('\n');
    }

    eprintln!("{}", buf);

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

    for (ind, line) in s.lines().enumerate() {
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