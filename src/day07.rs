use aoc::{Result, CustomError};
use std::collections::{HashMap, BTreeSet};

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;

    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Step {
    first: String,
    second: String,
}

fn get_steps(s: &str) -> Result<Vec<Step>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Step (\w+) must be finished before step (\w+) can begin\."
        ).unwrap();
    }

    let steps = s.lines()
        .map(|line| {
            let caps = RE.captures(line)
                .ok_or_else(|| CustomError("Invalid captures".to_owned()))?;

            let first: String = aoc::get_value(&caps, 1)?;
            let second: String = aoc::get_value(&caps, 2)?;
            Ok(Step {
                first,
                second,
            })
        })
        .collect()
        ;

    steps
}

fn pop_front(tree: &mut BTreeSet<String>) -> String {
    let first = tree.iter()
        .next()
        .unwrap()
        .to_string();

    tree.remove(&first);

    first
}

fn part1(s: &str) -> Result<String> {

    let steps = get_steps(s)?;

    let mut firsts  = BTreeSet::new();
    let mut seconds = BTreeSet::new();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut prereqs: HashMap<String, Vec<String>> = HashMap::new();

    for step in steps.iter() {
        firsts.insert(step.first.clone());
        seconds.insert(step.second.clone());
        {
            let entry = map.entry(step.first.clone())
                .or_insert_with(|| Vec::new());
            entry.push(step.second.clone());
        }
        {
            let entry = prereqs.entry(step.second.clone())
                .or_insert_with(|| Vec::new());
            entry.push(step.first.clone());
        }
    }

    // eprintln!("Maps {:?}", map);
    // eprintln!("Reqs {:?}", prereqs);

    let mut stack: BTreeSet<_> = firsts.difference(&seconds).cloned().collect();

    // eprintln!("Starts {:?}", stack);

    let mut completed: Vec<String> = Vec::new();

    while !stack.is_empty() {
        let node = pop_front(&mut stack);
        // eprintln!("Visiting {}", node);

        completed.push(node.clone());

        match map.get(&node) {
            Some(after) => {
                for aft in after.iter() {
                    match prereqs.get(aft) {
                        Some(reqs) => {
                            let all_completed =
                                reqs.iter().all(|v| completed.contains(v));

                            if all_completed {
                                stack.insert(aft.to_string());
                            }
                        }
                        None => {
                            stack.insert(aft.to_string());
                        }
                    }
                }
            }
            None => {}
        }
    }

    let res = completed.join("");

    eprintln!("part1: {}", res);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
    ";

    const INPUT_UNSORTED: &'static str = r"
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
    ";

    #[test]
    fn part1_example_input() {
        assert_eq!("CABDFE", part1(INPUT.trim()).unwrap());
    }

    #[test]
    fn part1_example_input_unsorted() {
        assert_eq!("CABDFE", part1(INPUT_UNSORTED.trim()).unwrap());
    }
}