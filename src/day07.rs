use aoc::{Result, CustomError};
use std::collections::{HashMap, BTreeMap, BTreeSet};

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s, 60, 5)?;

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

fn pop_front(tree: &mut BTreeSet<String>) -> Option<String> {

    let first = tree.iter().next().map(|v| v.to_string());

    if let Some(fst) = first {
        tree.remove(&fst);
        return Some(fst);
    }
    None


    // let first = tree.iter()
    //     .next()
    //     .unwrap()
    //     .to_string();

    // tree.remove(&first);

    // first
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

    let mut work_queue: BTreeSet<_> = firsts.difference(&seconds).cloned().collect();

    eprintln!("Starts {:?}", work_queue);

    let mut completed: Vec<String> = Vec::new();

    while !work_queue.is_empty() {
        let node = pop_front(&mut work_queue).unwrap();
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
                                work_queue.insert(aft.to_string());
                            }
                        }
                        None => {
                            work_queue.insert(aft.to_string());
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

type WorkMap = BTreeMap<String, Vec<String>>;

fn part2(s: &str, min_time: usize, nr_workers: usize) -> Result<usize> {

    let times: HashMap<String, usize> = (b'A'..=b'Z').enumerate()
        .map(|(i, v)| {
            ((v as char).to_string(), (i + 1) + min_time)
        })
        .collect()
        ;

    // eprintln!("Times {:?}", times);

    let steps = get_steps(s)?;

    let mut firsts  = BTreeSet::new();
    let mut seconds = BTreeSet::new();
    let mut map: WorkMap = BTreeMap::new();
    let mut prereqs: WorkMap = BTreeMap::new();

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

    #[derive(Debug, Clone, Default)]
    struct Work {
        id: usize,
        target: Option<String>,
        duration: usize,
    }

    // eprintln!("Maps {:?}", map);
    // eprintln!("Reqs {:?}", prereqs);

    let mut work_queue: BTreeSet<_> = firsts.difference(&seconds).cloned().collect();

    // eprintln!("Starts {:?}", work_queue);

    let mut completed: Vec<String> = Vec::new();

    let mut tick = 0;

    let mut workers = (0..nr_workers)
        .into_iter()
        .map(|i| Work { id: i, ..Default::default() })
        .collect::<Vec<_>>();

    fn workers_working(workers: &[Work]) -> bool {
        workers.iter().any(|v| {
            if let Some(_) = v.target {
                return true;
            }
            false
        })
    }

    fn add_work(step: &str, work_queue: &mut BTreeSet<String>, steps: &WorkMap, prereqs: &WorkMap, completed: &mut Vec<String>) {
        completed.push(step.to_string());

        if let Some(after) = steps.get(step) {
            for aft in after.iter() {
                match prereqs.get(aft) {
                    Some(reqs) => {
                        let all_completed =
                            reqs.iter().all(|v| completed.contains(v));

                        if all_completed {
                            work_queue.insert(aft.to_string());
                        }
                    }
                    None => {
                        work_queue.insert(aft.to_string());
                    }
                }
            }
        }
    }

    fn maybe_take_work(worker: &mut Work, work_queue: &mut BTreeSet<String>, times: &HashMap<String, usize>) -> bool {
        if let Some(work) = pop_front(work_queue) {
            worker.duration = *times.get(&work).unwrap();
            worker.target = Some(work);
            return true;
        } else {
            worker.target = None;
            false
        }
    }

    while !work_queue.is_empty() || workers_working(&workers) {
        // eprintln!("{} Queue {:?}", tick, work_queue);

        let mut output = format!("{: <6}", tick);
        // let mut output = String::new()

        for worker in workers.iter_mut() {
            match worker.target {
                // Worker is working on something
                Some(ref t) => {
                    if worker.duration > 0 {
                        worker.duration -= 1;
                    }

                    // Worker completed current task
                    if worker.duration == 0 {
                        add_work(t, &mut work_queue, &map, &prereqs, &mut completed);
                        maybe_take_work(worker, &mut work_queue, &times);
                    }
                },

                // Worker is waiting for work
                None => {
                    maybe_take_work(worker, &mut work_queue, &times);
                }
            }
        }

        // After all workers have given up their work
        // attempt one final time to get actual work
        for worker in workers.iter_mut() {
            match worker.target {
                // Worker is working on something
                Some(_) => {},

                // Worker is waiting for work
                None => {
                    maybe_take_work(worker, &mut work_queue, &times);
                }
            }
        }

        for worker in workers.iter() {
            match worker.target {
                Some(ref t) => {
                    output.push_str(&format!("{: <2}", t));
                }
                None => {
                    output.push_str(&format!("{: <2}", '.'));
                }
            }
        }

        output.push_str(&format!("   {}", completed.join("")));

        eprintln!("{}", output);
        tick += 1;
    }

    // Remove last tick that gets added when the loop goes around one final time
    if tick > 0 {
        tick -= 1;
    }

    let res = completed.join("");

    eprintln!("part1: {}", res);

    eprintln!("part2: {}", tick);

    Ok(0)
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

    #[test]
    fn part2_example_input() {
        assert_eq!(15, part2(INPUT.trim(), 0, 2).unwrap());
    }
}
