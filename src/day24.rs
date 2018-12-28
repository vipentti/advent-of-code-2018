
use aoc::{Result, CustomError};

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}


fn part1(s: &str) -> Result<usize> {
    let mut teams: HashMap<Team, Vec<Group>> = HashMap::new();
    let mut current_team = Team::ImmuneSystem;

    let mut groups = Vec::new();

    for (ind, line) in s.lines().enumerate() {
        if line.starts_with("Immune System:") {
            current_team = Team::ImmuneSystem;
            continue;
        }

        if line.starts_with("Infection:") {
            current_team = Team::Infection;
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        let mut group = line.parse::<Group>()
            .map_err(|_| CustomError(format!("Invalid capture at line {}", ind + 1)))?;

        group.team = current_team;
        let len = teams.entry(current_team)
            .or_insert_with(Vec::new)
            .len();

        group.id = len as i64 + 1;

        groups.push(group.clone());

        teams.entry(current_team)
            .or_insert_with(Vec::new)
            .push(group);
    }

    // eprintln!("Groups: {:?}", groups);


    show_groups(&groups);

    while has_both(&groups) {
        run_fight(&mut groups);

        #[cfg(test)]
        show_groups(&groups);
    }

    show_groups(&groups);

    let units: i64 = groups.iter().map(|g| g.units).sum();

    eprintln!("part1: {:?}", units);

    Ok(units as usize)
}

fn part2(s: &str) -> Result<i64> {
    let mut teams: HashMap<Team, Vec<Group>> = HashMap::new();
    let mut current_team = Team::ImmuneSystem;

    let mut groups = Vec::new();

    for (ind, line) in s.lines().enumerate() {
        if line.starts_with("Immune System:") {
            current_team = Team::ImmuneSystem;
            continue;
        }

        if line.starts_with("Infection:") {
            current_team = Team::Infection;
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        let mut group = line.parse::<Group>()
            .map_err(|_| CustomError(format!("Invalid capture at line {}", ind + 1)))?;

        group.team = current_team;
        let len = teams.entry(current_team)
            .or_insert_with(Vec::new)
            .len();

        group.id = len as i64 + 1;

        groups.push(group.clone());

        teams.entry(current_team)
            .or_insert_with(Vec::new)
            .push(group);
    }


    let original_groups = groups.clone();

    /*
    let mut current_boost: i64 = 10000;

    let mut min_boost = i64::max_value();
    let mut max_boost = i64::min_value();

    // Find boost
    let mut prev = false;

    loop {
        let mut groups = original_groups.clone();

        let c = run_with_boost(&mut groups, current_boost);

        if prev && !c {
            max_boost = current_boost + 1000;
            break;
        } else {
            current_boost -= 1000;
            prev = c;
        }
    }

    prev = false;
    current_boost = 0;

    loop {
        let mut groups = original_groups.clone();

        let c = run_with_boost(&mut groups, current_boost);

        if !prev && c {
            min_boost = current_boost - 1000;
            break;
        } else {
            current_boost += 1000;
            prev = c;
        }
    }

    eprintln!("Max {}", max_boost);
    eprintln!("Min {}", min_boost);
    */

    let mut min = 0;
    let mut max = 1_000_000;

    for step in &[100_000, 10_000, 1000, 100, 10, 5] {
        match (find_min_boost(&original_groups, *step, min, max), find_max_boost(&original_groups, *step, min, max) ) {
            (Some(mn), Some(mx)) => {
                eprintln!("Step {} min {} max {}", step, mn, mx);
                min = mn;
                max = mx;
            }

            other => panic!("{} {:?}", step, other),
        }
    }

    eprintln!("Here {} - {}", min, max);

    let mut boost = min;

    let mut prev = false;
    loop {
        let mut g = original_groups.clone();

        let c = run_with_boost(&mut g, boost);

        if !prev && c {
            break;
        } else {
            boost += 1;
            prev = c;
        }
    }

    eprintln!("Boost {}", boost);
    run_with_boost(&mut groups, boost);

    // loop {
    //     let mut groups = original_groups.clone();


    //     // if run_with_boost(&mut groups, current_boost) {
    //     //     current_boost = current_boost / 2;
    //     // } else {
    //     //     current_boost = current_boost * 2;
    //     // }
    // }

    let units: i64 = groups.iter().map(|g| g.units).sum();

    eprintln!("part2: {:?}", units);

    Ok(units)
}

fn find_max_boost(original_groups: &Vec<Group>, step: i64, min: i64, max: i64) -> Option<i64> {
    let mut current_boost: i64 = max;
    let mut max_boost = i64::max_value();
    let mut prev = false;

    loop {
        if current_boost < min {
            return None;
        }
        let mut groups = original_groups.clone();

        let c = run_with_boost(&mut groups, current_boost);

        if prev && !c {
            max_boost = current_boost + step;
            break;
        } else {
            current_boost -= step;
            prev = c;
        }
    }

    Some(max_boost)
}

fn find_min_boost(original_groups: &Vec<Group>, step: i64, min: i64, max: i64) -> Option<i64> {
    let mut current_boost: i64 = min;
    let mut min_boost = i64::max_value();
    let mut prev = false;

    loop {
        if current_boost > max {
            return None;
        }
        let mut groups = original_groups.clone();

        let c = run_with_boost(&mut groups, current_boost);

        if !prev && c {
            min_boost = current_boost - step;
            break;
        } else {
            current_boost += step;
            prev = c;
        }
    }

    Some(min_boost)
}

fn count_units(groups: &[Group]) -> i64 {
    groups.iter().map(|g| g.units).sum()
}

fn run_with_boost(groups: &mut Vec<Group>, boost: i64) -> bool {
    for g in groups.iter_mut() {
        if g.team == Team::ImmuneSystem {
            g.damage += boost;
        }
    }

    let mut prev_counts: Vec<i64> = Vec::new();
    prev_counts.push(count_units(&groups));

    // #[cfg(test)]
    // show_groups(&groups);
    while has_both(&groups) {
        run_fight(groups);

        let count = count_units(&groups);

        if prev_counts.iter().rev().take(10).all(|&c| c == count) {
            eprintln!("Stalemate {}", boost);
            break;
        } else {
            prev_counts.push(count);
        }
    }

    // #[cfg(test)]
    // show_groups(&groups);

    let e1 = groups.iter().any(|g| g.team == Team::ImmuneSystem);
    let e2 = groups.iter().any(|g| g.team == Team::Infection);
    e1 && !e2
}

fn has_both(groups: &[Group]) -> bool {
    let e1 = groups.iter().any(|g| g.team == Team::ImmuneSystem);
    let e2 = groups.iter().any(|g| g.team == Team::Infection);

    e1 && e2
}

fn show_groups(groups: &[Group]) {
    let (immune, infection): (Vec<_>, Vec<_>) = groups.iter()
        .partition(|g| g.team == Team::ImmuneSystem)
        ;
    eprintln!();
    eprintln!("ImmuneSystem: ");
    for g in immune {
        eprintln!("Group {} contains {} units", g.id, g.units);
    }
    eprintln!();
    eprintln!("Infection: ");
    for g in infection {
        eprintln!("Group {} contains {} units", g.id, g.units);
    }
    eprintln!();
}

fn run_fight(groups: &mut Vec<Group>) {
    sort_groups_for_target(groups);

    let mut attack = HashMap::new();
    let mut defend = HashMap::new();

    for group in groups.iter() {
        let e = enemies_for(group, &groups)
            .filter(|e| !defend.contains_key(&(e.team, e.id)))
            ;

        let es: Vec<_> = e.collect();

        if let Some(id) = select_target(group, &es[..]) {
            // #[cfg(test)]
            // eprintln!("{:?} {} attacking {:?}", group.team, group.id, id);
            attack.insert((group.team, group.id), id);
            defend.insert(id, (group.team, group.id));
        }
    }

    sort_groups_for_attack(groups);

    let attackers: Vec<_> = groups.iter()
        .map(|a| (a.team, a.id))
        .collect();


    for att in attackers {
        if let Some(target) = attack.get(&att) {
            // #[cfg(test)]
            // eprintln!("Attacker {:?} attacking {:?}", att, target);

            let target_index = groups.iter()
                .position(|g| (g.team, g.id) == *target)
                .unwrap();

            let attacker_index = groups.iter()
                .position(|g| (g.team, g.id) == att)
                .unwrap();

            let attacker = &groups[attacker_index];

            let damage = groups[target_index].would_take_damage(attacker);

            groups[target_index].take_damage(damage);
        }
    }

    groups.retain(|g| {
        g.units > 0
    });
}

fn sort_groups_for_target(groups: &mut Vec<Group>) {
    use std::cmp::Ordering;
    groups.sort_by(|a, b| {
        match a.effective_power().cmp(&b.effective_power()).reverse() {
            Ordering::Equal => {
                a.initiative.cmp(&b.initiative).reverse()
            }
            other => other,
        }
    })
}

fn sort_groups_for_attack(groups: &mut Vec<Group>) {
    use std::cmp::Ordering;
    groups.sort_by(|a, b| {
        a.initiative.cmp(&b.initiative).reverse()
    })
}

fn enemies_for<'a>(attacker: &'a Group, groups: &'a [Group]) -> impl Iterator<Item=&'a Group> {
    let team = attacker.team;
    groups.iter()
        .filter(move |g| g.team != team)
}

fn select_target(attacker: &Group, possible_enemies: &[&Group]) -> Option<(Team, i64)> {
    use std::cmp::Ordering;

    let mut enemies: Vec<_> = possible_enemies.iter()
        .filter(|a| a.would_take_damage(attacker) > 0)
        .collect();

    enemies.sort_by(|a, b| {
        let dmg_a = a.would_take_damage(attacker);
        let dmg_b = b.would_take_damage(attacker);

        match dmg_a.cmp(&dmg_b).reverse() {
            Ordering::Equal => {

                match a.effective_power().cmp(&b.effective_power()).reverse() {
                    Ordering::Equal => {
                        a.initiative.cmp(&b.initiative).reverse()
                    },
                    other => other,
                }
            },
            other => other,
        }
    });

    // eprintln!("Enemies: {:?}", enemies);
    // #[cfg(test)]
    // for enemy in enemies.iter() {
    //     let damage = enemy.would_take_damage(attacker);
    //     eprintln!("{:?} {} would deal {} damage to {:?} {}", attacker.team, attacker.id, damage, enemy.team, enemy.id);
    // }

    enemies.first().map(|e| (e.team, e.id))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Group {
    team: Team,
    id: i64,
    units: i64,
    hp: i64,
    damage: i64,
    damage_type: DamageType,
    initiative: i64,
    immune_to: Vec<DamageType>,
    weak_to: Vec<DamageType>,
}

impl Group {
    pub fn effective_power(&self) -> i64 {
        self.units * self.damage
    }

    pub fn take_damage(&mut self, damage: i64) {
        if damage <= 0 || damage < self.hp {
            return;
        }

        let units_to_lose = damage / self.hp;

        let units_to_lose = std::cmp::min(self.units, units_to_lose);

        // #[cfg(test)]
        // eprintln!("{:?} {} will lose {} units", self.team, self.id, units_to_lose);

        self.units -= units_to_lose;
    }

    pub fn would_take_damage(&self, attacker: &Group) -> i64 {
        if self.immune_to.contains(&attacker.damage_type) {
            return 0;
        }

        let ap = attacker.effective_power();

        if self.weak_to.contains(&attacker.damage_type) {
            return ap * 2;
        }

        ap
    }
}

impl std::str::FromStr for Group {
    type Err = CustomError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            // 17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
            // (\d+) units each with (\d+) hit points (\(.*\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)
            // static ref RE: Regex = Regex::new(r"(\d+) units each with (\d+) hit points \(.*\)? with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
            // (\d+) units each with (\d+) hit points (\(.*\))?\s*with an attack that does (\d+) (\w+) damage at initiative (\d+)
            // static ref RE: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (\(.*\))*\s*with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
            // (?mi)(\d+) units each with (\d+) hit points (\(.*\))*\s*with an attack that does (\d+) (\w+) damage at initiative (\d+)
            static ref RE: Regex = Regex::new(r"(?i)(\d+) units each with (\d+) hit points\s*(\(.*\))*\s*with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        }
        let caps = RE.captures(s)
            .ok_or_else(|| CustomError(format!("Invalid capture")))?;


        // eprintln!("caps {:?}", caps);
        let units: i64 = aoc::get_value(&caps, 1)?;
        let hp: i64 = aoc::get_value(&caps, 2)?;
        let mut weak = Vec::new();
        let mut immune = Vec::new();
        if let Some(imm) = caps.get(3).map(|v| v.as_str().trim()) {
            let values: Vec<&str> = imm.trim_matches(|p| p == '(' || p == ')' )
                                    .split(';')
                                    .map(|s| s.trim())
                                    .collect();
            // eprintln!("Imm {:?}", values);

            for line in values.iter() {
                if line.starts_with("immune to ") {

                    let tmp: std::result::Result<_, _> = line.replace("immune to ", "")
                            .split(",")
                            .map(|s| s.trim())
                            .map(|s| s.parse::<DamageType>())
                            .collect();

                    let mut tmp = tmp?;

                    immune.append(&mut tmp);

                } else if line.starts_with("weak to ") {
                    let tmp: std::result::Result<_, _> = line.replace("weak to ", "")
                            .split(",")
                            .map(|s| s.trim())
                            .map(|s| s.parse::<DamageType>())
                            .collect();

                    let mut tmp = tmp?;

                    weak.append(&mut tmp);
                }
            }
        }
        let damage: i64 = aoc::get_value(&caps, 4)?;
        let tp: DamageType = aoc::get_value(&caps, 5)?;
        let init: i64 = aoc::get_value(&caps, 6)?;

        Ok(Group {
            units,
            hp,
            weak_to: weak,
            immune_to: immune,
            damage,
            damage_type: tp,
            initiative: init,
            team: Team::ImmuneSystem,
            id: -1,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Team {
    ImmuneSystem,
    Infection,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum DamageType {
    Radiation,
    Bludgeoning,
    Slashing,
    Fire,
    Cold,
}

impl std::str::FromStr for DamageType {
    type Err = CustomError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "fire" => Ok(DamageType::Fire),
            "cold" => Ok(DamageType::Cold),
            "slashing" => Ok(DamageType::Slashing),
            "bludgeoning" => Ok(DamageType::Bludgeoning),
            "radiation" => Ok(DamageType::Radiation),
            other => Err(CustomError(format!("Unknown damagetype {}", other))),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let input = r"
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
        ";

        assert_eq!(782 + 4434, part1(input.trim()).unwrap());
    }

    #[test]
    fn part2_example_input() {
        let input = r"
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
        ";

        assert_eq!(51, part2(input.trim()).unwrap());
    }
}