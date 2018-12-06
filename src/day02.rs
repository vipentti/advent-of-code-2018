use std::collections::{HashMap};
use aoc::Result;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<usize> {

    let mut twices = 0;
    let mut thrices = 0;

    for line in s.lines() {
        let (twice, thrice) = count_duplicates(&line)?;

        twices += twice;
        thrices += thrice;
    }

    let out = twices * thrices;

    eprintln!("part1 {}", out);

    Ok(out)
}

fn part2(s: &str) -> Result<String> {
    for (i, line) in s.lines().enumerate() {
        for alt in s.lines().skip(i) {
            if let Some(s) = diff_by_one(&line, &alt) {
                eprintln!("part2 {}", s);
                return Ok(s);
            }
        }
    }

    Ok("".into())
}

fn diff_by_one(lhs: &str, rhs: &str) -> Option<String> {
    if lhs.len() != rhs.len() {
        return None;
    }

    let mut diff = 0;
    let mut buffer = Vec::with_capacity(lhs.len());

    for (cha, chb) in lhs.chars().zip(rhs.chars()) {

        if cha != chb {
            diff += 1;
        } else {
            buffer.push(cha);
        }

        if diff > 1 {
            return None;
        }

    }

    if diff == 0 {
        return None;
    }

    let s = buffer.into_iter()
        .collect::<String>();

    Some(s)
}

fn count_duplicates(s: &str) -> Result<(usize, usize)> {

    let mut twice = 0;
    let mut thrice = 0;

    // let mut seen = HashSet::new();
    let mut twices = HashMap::new();
    let mut thrices = HashMap::new();

    for ch in s.chars() {
        {
            let counter = twices.entry(ch).or_insert(0);
            *counter += 1;
        }
        {
            let counter = thrices.entry(ch).or_insert(0);
            *counter += 1;
        }
    }

    for (_, v) in twices {
        if v == 2 {
            twice = 1;
            break;
        }
    }


    for (_, v) in thrices {
        if v == 3 {
            thrice = 1;
            break;
        }
    }

    Ok((twice, thrice))
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!((1, 1), count_duplicates("bababc").unwrap());
        assert_eq!((1, 0), count_duplicates("abbcde").unwrap());
        assert_eq!((0, 1), count_duplicates("ababab").unwrap());
    }

    #[test]
    fn test_example() {
        let input = r#"
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
        "#;

        assert_eq!(12, part1(input).unwrap());
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(Some("fgij".to_owned()), diff_by_one("fghij", "fguij"));
    }

    #[test]
    fn test_example() {
        let input = r#"
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
        "#;

        assert_eq!("fgij", part2(input).unwrap());
    }
}