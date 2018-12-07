use std::collections::HashSet;

use aoc::{read_input, Result};

fn main() -> Result<()> {
    let s = read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}

fn reacts(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn react(s: &str, ignore: Option<char>) -> Vec<char> {
    let mut output = Vec::new();

    for polymer in s.chars() {
        if Some(polymer.to_ascii_lowercase()) == ignore {
            // skip
        } else if reacts(polymer, output.last().cloned().unwrap_or_default()) {
            output.pop();
        } else {
            output.push(polymer);
        }
    }

    output
}

fn part1(s: &str) -> Result<usize> {
    let output = react(s, None);

    eprintln!("part1 {}", output.len());

    Ok(output.len())
}

fn part2(s: &str) -> Result<usize> {
    let polymers = s
        .chars()
        .map(|v| v.to_ascii_lowercase())
        .collect::<HashSet<char>>();

    let mut length = usize::max_value();

    for polymer in polymers.iter() {
        let chars = react(s, Some(*polymer));

        if chars.len() < length {
            length = chars.len();
        }
    }
    eprintln!("part2 {}", length);

    Ok(length)
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = r"dabAcCaCBAcCcaDA";

        assert_eq!(10, part1(input.trim()).unwrap());
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    #[test]
    fn example_input() {
        let input = r"dabAcCaCBAcCcaDA";

        assert_eq!(4, part2(input.trim()).unwrap());
    }
}
