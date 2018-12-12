use aoc::{Result, CustomError};
use std::fmt;
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}

type State = BTreeMap<i64, PotState>;

type Rules = BTreeMap<[PotState; 5], PotState>;

fn read_from(s: &str) -> Result<(State, Rules)> {
    let s = s.replace("initial state: ", "");

    let mut iter = s.lines();

    let mut pots: State = BTreeMap::new();

    if let Some(initial) = iter.next() {
        pots = initial.char_indices()
            .filter(|(_, ch) | match ch {
                '#' => true,
                _ => false,
            })
            .map(|(ind, ch)| match ch {
                '#' => (ind as i64, PotState::Plant),
                _ => unreachable!(),
            })
            .collect();
    }

    // Skip the empty line
    iter.next();

    let mut rules: Rules = BTreeMap::new();

    for rule in iter {
        let mut state_arr: [PotState; 5] = Default::default();
        let states: Vec<_> = rule.chars().take(5)
            .map(|ch| {
                match ch {
                    '#' => PotState::Plant,
                    _ => PotState::Empty,
                }
            })
            .collect();

        state_arr.copy_from_slice(&states[0..5]);

        let after: Vec<_> = rule.chars()
            .skip(9)
            .map(|ch| {
                match ch {
                    '#' => PotState::Plant,
                    _ => PotState::Empty,
                }
            })
            .collect();

        if let Some(rule) = after.first() {
            rules.insert(state_arr, *rule);
        } else {
            return Err(CustomError("Invalid format".to_string()).into());
        }
    }

    Ok((pots, rules))
}

fn get_state(id: i64, state: &State) -> PotState {
    state.get(&id).cloned().unwrap_or_default()
}

fn advance(state: &State, rules: &Rules) -> State {
    let mut new_state: State = BTreeMap::new();

    let orig_min_id = state.keys().min().unwrap();
    let orig_max_id = state.keys().max().unwrap();

    let min_id = *orig_min_id - 3;
    let max_id = *orig_max_id + 3;

    for id in min_id..=max_id {
        let rule_state: [PotState; 5] = [
            get_state(id - 2, &state),
            get_state(id - 1, &state),
            get_state(id, &state),
            get_state(id + 1, &state),
            get_state(id + 2, &state),
        ];

        let next_state = rules.get(&rule_state).cloned().unwrap_or_default();

        if next_state == PotState::Plant {
            new_state.insert(id, next_state);
        }
    }


    new_state
}

fn visualize_generation_with(gen: i64, state: &State, min: i64, max: i64) {
    let mut out = String::new();

    for id in (min - 1)..=(max + 1) {
        let val = state.get(&id).cloned().unwrap_or_default();

        out.push(val.as_char());
    }

    eprintln!("{: >3} {}", gen, out);
}

fn show_rule(rule: [PotState; 5], after: PotState) -> String {
    let mut out = String::new();

    for r in rule.iter() {
        out.push(r.as_char());
    }
    out.push_str(" => ");

    out.push(after.as_char());

    out
}

fn visualize_rules(rules: &Rules) {
    for (rule, next) in rules.iter() {
        let r = show_rule(*rule, *next);
        eprintln!("{}", r);
    }
}

fn count_ids(state: &State) -> i64 {
    let mut ids = 0;

    for (id, pot) in state.iter() {
        if *pot == PotState::Plant {
            ids += id;
        }
    }

    ids
}

fn diff_ids(current: &State, prev: &State) -> i64 {
    let mut diff = 0;

    for (cur, prv) in current.keys().zip(prev.keys()) {
        if *cur != *prv + 1 {
            diff += 1
        }
    }

    diff
}

fn part1(s: &str) -> Result<i64> {
    let (orig_state, rules) = read_from(s)?;
    let mut state = orig_state;

    visualize_rules(&rules);

    let mut states: Vec<State> = Vec::new();
    states.push(state.clone());

    let mut min = 0;
    let mut max = 0;
    let min_id = state.keys().min().unwrap();
    let max_id = state.keys().max().unwrap();
    min = std::cmp::min(min, *min_id);
    max = std::cmp::max(max, *max_id);

    for _ in 0..20 {
        // visualize_generation(i, &state);
        state = advance(&state, &rules);
        let min_id = state.keys().min().unwrap();
        let max_id = state.keys().max().unwrap();

        min = std::cmp::min(min, *min_id);
        max = std::cmp::max(max, *max_id);

        states.push(state.clone());
    }

    for (id, state) in states.iter().enumerate() {
        visualize_generation_with(id as i64, state, min, max);
    }

    let total_id = count_ids(&state);

    eprintln!("part1: {}", total_id);

    Ok(total_id)
}

fn part2(s: &str) -> Result<i64> {
    let (orig_state, rules) = read_from(s)?;
    let mut state = orig_state;

    visualize_rules(&rules);

    let mut states: Vec<State> = Vec::new();
    states.push(state.clone());

    let mut min = 0;
    let mut max = 0;
    let min_id = state.keys().min().unwrap();
    let max_id = state.keys().max().unwrap();
    min = std::cmp::min(min, *min_id);
    max = std::cmp::max(max, *max_id);

    let mut found = false;

    let mut increment: i64 = 0;

    let mut last_id: i64 = 0;

    for id in 0..50_000_000_000i64 {
        // visualize_generation(i, &state);
        state = advance(&state, &rules);
        let min_id = state.keys().min().unwrap();
        let max_id = state.keys().max().unwrap();

        min = std::cmp::min(min, *min_id);
        max = std::cmp::max(max, *max_id);

        let diff_ids = diff_ids(&state, states.last().unwrap());

        increment = count_ids(&state) - count_ids(states.last().unwrap());

        states.push(state.clone());

        last_id = id + 1;


        if found {
            break;
        }

        // Iterates one more time
        if diff_ids == 0 {
            eprintln!("Stopped {}", id);
            found = true;
        }

    }

    for (id, state) in states.iter().enumerate() {
        visualize_generation_with(id as i64, state, min, max);
    }

    eprintln!("incr {} @ {}", increment, last_id);

    let mut total_id: i64 = count_ids(&state);

    total_id += (50_000_000_000i64 - last_id) * increment;

    eprintln!("part2: {}", total_id);

    Ok(total_id)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum PotState {
    Empty,
    Plant
}

impl PotState {
    fn as_char(self) -> char {
        match self {
            PotState::Empty => '.',
            PotState::Plant => '#',
        }
    }
}

impl fmt::Display for PotState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PotState::Empty => write!(f, "."),
            PotState::Plant => write!(f, "#"),
        }
    }
}

impl Default for PotState {
    fn default() -> Self {
        PotState::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
    ";
    #[test]
    fn part1_example_input() {
        assert_eq!(325, part1(INPUT.trim()).unwrap());
    }
}