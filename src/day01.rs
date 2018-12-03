use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();

    let mut file = File::open("input/day01.txt")?;

    file.read_to_string(&mut s)?;

    let numbers: std::result::Result<Vec<_>, _> = s.lines()
        .map(|v| v.parse::<i32>())
        .collect();

    let numbers = numbers?;

    part1(&numbers)?;
    part2(&numbers)?;

    Ok(())
}

fn part1(nr: &[i32]) -> Result<()> {
    let mut freq = 0;

    for change in &nr[..] {
        freq += change;
    }

    eprintln!("freq: {}", freq);

    Ok(())
}

fn part2(nr: &[i32]) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for change in &nr[..] {
            freq += change;
            if !seen.insert(freq) {
                eprintln!("dup: {}", freq);
                return Ok(());
            }
        }
    }
}