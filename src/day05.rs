use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();

    let mut file = File::open("input/day05.txt")?;

    file.read_to_string(&mut s)?;

    part1(&s.trim())?;
    part2(&s.trim())?;

    Ok(())
}

fn flip_char(a: &char) -> char {
    if a.is_ascii_lowercase() {
        a.to_ascii_uppercase()
    } else {
        a.to_ascii_lowercase()
    }
}

fn react(s: &str) -> Vec<char> {
    let bytes = s.trim().chars().collect::<Vec<_>>();

    eprintln!("bytes: {:?}", bytes.len());

    let mut last_length = usize::max_value();

    let mut output = bytes;
    let mut was_last = false;
    loop {
        let mut index = 0;
        let mut result: Vec<char> = Vec::new();
        while index < output.len() {
            let a = &output[index];

            if index < output.len() - 1 {
                let b = &output[index + 1];

                if flip_char(a) == *b {
                    index += 1;
                } else {
                    result.push(*a);
                }
            } else {
                result.push(*a);
            }
            index += 1;
        }

        if last_length == result.len() {
            if was_last {
                output = result;
                break;
            } else {
                last_length = result.len();
                output = result;
                was_last = true;
            }
        } else {
            last_length = result.len();
            output = result;
        }
    }
    output
}

fn part1(s: &str) -> Result<usize> {
    let output = react(s);

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
        let bytes = s
            .chars()
            .filter(|c| {
                *c != *polymer && flip_char(c) != *polymer
            })
            .collect::<String>();

        let chars = react(&bytes);

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