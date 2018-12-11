use aoc::{Result, Vector2};
use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    let serial_number: i32 = s.parse::<i32>()
        .map_err::<Box<std::num::ParseIntError>, _>(|e| e.into())?;

    part1(serial_number)?;
    part2(serial_number)?;

    Ok(())
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Grid {
    data: Vec<i32>,
    offset: usize,
    width: usize,
    height: usize,
}

impl Index<Vector2> for Grid {
    type Output = i32;

    fn index(&self, index: Vector2) -> &Self::Output {
        &self.data[self.to_index(index)]
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = i32;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.data[self.to_index(index.into())]
    }
}

impl IndexMut<Vector2> for Grid {
    fn index_mut(&mut self, index: Vector2) -> &mut i32 {
        let i = self.to_index(index);
        &mut self.data[i]
    }
}

impl IndexMut<(i32, i32)> for Grid {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut i32 {
        let i = self.to_index(index.into());
        &mut self.data[i]
    }
}

impl Grid {
    fn new_with(serial: i32) -> Self {
        let mut grid = Grid {
            data: vec![0; 300 * 300],
            offset: 1,
            width: 300,
            height: 300,
        };

        for y in (grid.offset as i32)..=(grid.height as i32) {
            for x in (grid.offset as i32)..=(grid.width as i32) {
                let rack_id = x + 10;
                let mut power_level = rack_id * y;
                power_level += serial;

                power_level *= rack_id;

                power_level = hundred_digit(power_level);
                power_level -= 5;

                grid[(x, y)] = power_level;
            }
        }

        grid
    }

    fn to_index(&self, vec: Vector2) -> usize {
        (vec.y as usize - self.offset) * self.width + (vec.x as usize - self.offset)
    }

    #[allow(dead_code)]
    fn as_string(&self) -> String {
        let mut out = String::new();

        for y in (self.offset as i32)..=(self.height as i32) {
            let mut row = String::new();
            for x in (self.offset as i32)..=(self.width as i32) {
                let val = self[(x, y)];
                row.push_str(&format!("{:^3}", val));
            }

            out.push_str(&row);
            out.push('\n');
        }

        out
    }

    fn find_region(&self, size: i32) -> Option<(Vector2, i32)> {
        let off = self.offset as i32;
        let height = self.height as i32;
        let width = self.width as i32;

        let mut current_max = i32::min_value();
        let mut current_point: Vector2 = (0, 0).into();

        // Region has to be fully inside
        for y in off..=(height - size) {
            for x in off..=(width - size) {

                let mut total = 0;

                for y_inc in 0..size {
                    for x_inc in 0..size {
                        let val = self[(x + x_inc, y + y_inc)];

                        total += val;
                    }
                }

                if total >= current_max {
                    current_max = total;
                    current_point = (x, y).into();
                }
            }
        }

        if current_max != i32::min_value() {
            Some((current_point, current_max))
        } else {
            None
        }
    }

    fn find_sized_region(&self) -> Option<(Vector2, i32, i32)> {
        let mut current_size = i32::min_value();
        let mut current_max = i32::min_value();
        let mut current_pt: Vector2 = (0, 0).into();

        for size in 1..=300 {
            if let Some((pt, max)) = self.find_region(size) {
                if max >= current_max {
                    current_max = max;
                    current_pt = pt;
                    current_size = size;
                }
            }
        }

        if current_max != i32::min_value() {
            Some((current_pt, current_max, current_size))
        } else {
            None
        }
    }
}

fn part1(serial: i32) -> Result<Vector2> {
    let grid = Grid::new_with(serial);


    if let Some((pt, max)) = grid.find_region(3) {
        eprintln!("part1_max: {}", max);
        eprintln!("part1_point: {:?}", pt);
        return Ok(pt);
    }

    Ok((0, 0).into())
}

fn part2(serial: i32) -> Result<(i32, i32, i32)> {

    let grid = Grid::new_with(serial);

    if let Some((pt, max, size)) = grid.find_sized_region() {
        eprintln!("part2_max: {}", max);
        eprintln!("part2_point: {:?}", pt);
        eprintln!("part2_size: {:?}", size);
        return Ok((pt.x, pt.y, size));
    }

    Ok((0, 0, 0))
}

fn hundred_digit(value: i32) -> i32 {
    (value / 100) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        assert_eq!(Vector2::from((33, 45)), part1(18).unwrap());
        assert_eq!(Vector2::from((21, 61)), part1(42).unwrap());
    }

    #[test]
    fn part2_example_input() {
        assert_eq!((90, 269, 16), part2(18).unwrap());
        assert_eq!((232, 251, 12), part2(42).unwrap());
    }
}