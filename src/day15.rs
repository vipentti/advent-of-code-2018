#![allow(dead_code)]
use aoc::{Result, CustomError};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

type GenId = u32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct Vector2 {
    // Y before X so we can sort these
    pub y: i32,
    pub x: i32,
}

use std::fmt;

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vector2 {
    fn around(&self) -> [Vector2; 4] {
        [
            self.left(),
            self.right(),
            self.down(),
            self.up(),
        ]
    }

    fn up(&self) -> Self {
        *self + (0, -1)
    }

    fn down(&self) -> Self {
        *self + (0, 1)
    }

    fn left(&self) -> Self {
        *self + (-1, 0)
    }

    fn right(&self) -> Self {
        *self + (1, 0)
    }
}

impl Add<(i32, i32)> for Vector2 {
    type Output = Vector2;

    fn add(self, other: (i32, i32)) -> Self::Output {
        Vector2 {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: i32) -> Self {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<(i32, i32)> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: (i32, i32)) -> Self::Output {
        Vector2 {
            x: self.x - other.0,
            y: self.y - other.1,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}


impl std::convert::From<(i32, i32)> for Vector2 {
    fn from(v: (i32, i32)) -> Self {
        Vector2 { x: v.0, y: v.1 }
    }
}

impl std::convert::From<(usize, usize)> for Vector2 {
    fn from(v: (usize, usize)) -> Self {
        Vector2 { x: v.0 as i32, y: v.1 as i32 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Index {
    index: usize,
    generation: GenId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct AllocEntry {
    is_live: bool,
    generation: GenId,
}

#[derive(Debug, Clone, Default)]
struct IndexAllocator {
    entries: Vec<AllocEntry>,
    free: Vec<usize>,
}

impl IndexAllocator {
    fn allocate(&mut self) -> Index {
        // No free slots available
        // Create new entry
        if self.free.is_empty() {
            let index = self.entries.len();

            self.entries.push(AllocEntry {
                is_live: true,
                generation: 0,
            });

            Index { index, generation: 0, }
        } else {
            // Take the last index
            let index = self.free.pop().unwrap();

            self.entries[index].is_live = true;

            let next_gen = self.entries[index].generation;

            Index {
                index,
                generation: next_gen,
            }
        }
    }

    fn deallocate(&mut self, index: Index) -> bool {
        let entry = self.entries[index.index];
        if entry.is_live && entry.generation == index.generation {
            self.entries[index.index].is_live = false;
            self.entries[index.index].generation += 1;

            self.free.push(index.index);

            true
        } else {
            false
        }
    }

    fn is_live(&self, index: Index) -> bool {
        self.entries.get(index.index)
            .filter(|i| i.generation == index.generation)
            .map(|v| v.is_live)
            .unwrap_or(false)

    }
}

struct Entry<T> {
    value: T,
    generation: GenId,
}

#[derive(Default)]
struct GenerationArray<T> {
    entries: Vec<Option<Entry<T>>>,
}

impl<T> GenerationArray<T> {
    // Set the value for some generational index.  May overwrite past generation
    // values.
    pub fn set(&mut self, index: Index, value: T) {
        for _ in self.entries.len()..=index.index {
            self.entries.push(None);
        }

        self.entries[index.index] = Some(Entry {
            generation: index.generation,
            value,
        });
    }

    pub fn remove(&mut self, index: Index) {
        if let Some(node) = self.entries.get_mut(index.index) {
            if let Some(ref mut inner) = node {
                if inner.generation == index.generation {
                    self.entries[index.index] = None;
                }
            }
        }
    }

    // Gets the value for some generational index, the generation must match.
    pub fn get(&self, index: Index) -> Option<&T> {
        self.entries.get(index.index)
            .filter(|entry| entry.is_some())
            .map(|entry| entry.as_ref().unwrap())
            .filter(|i| i.generation == index.generation)
            .map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        self.entries.get_mut(index.index)
            .filter(|entry| entry.is_some())
            .map(|entry| entry.as_mut().unwrap())
            .filter(|i| i.generation == index.generation)
            .map(|entry| &mut entry.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries.iter()
            .filter(|entry| entry.is_some())
            .map(|entry| entry.as_ref().unwrap())
            .map(|entry| &entry.value)
    }
}

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    // part2(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<i32> {
    let mut map = read_map(s)?;

    map.sort_entities();
    map.render();


    let mut round = 1;

    loop {
        eprintln!("after of: {} round(s)", round - 1);

        map.render();

        if map.update() || round >= 1000 {
            break;
        }

        map.sort_entities();


        round += 1;
    }

    eprintln!("End after {}", round - 1);

    map.render();

    let last_round = round as i32 - 1;

    let total_hp = map.total_hp();

    eprintln!("Round {} hp {} total {}", last_round, total_hp, last_round * total_hp);


    Ok(last_round * total_hp)
}

fn manhattan_distance(a: &Vector2, b: &Vector2) -> usize {
    ((b.x - a.x).abs() + (b.y - a.y).abs()) as usize
}

fn read_map(s: &str) -> Result<World> {

    let mut world = World::empty();

    let mut grid = Vec::new();

    for (linenr, line) in s.trim().lines().enumerate() {
        let mut row = Vec::new();
        for (ind, ch) in line.char_indices() {
            match ch {
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Empty),
                'G' => {
                    // TODO Goblin
                    row.push(Tile::Empty);

                    world.add_goblin_at((ind, linenr).into());
                },
                'E' => {
                    // TODO Elf
                    row.push(Tile::Empty);

                    world.add_elf_at((ind, linenr).into());
                },
                _ => return Err(CustomError("Invalid map".into()).into()),
            }
        }

        grid.push(row);
    }

    assert!(!grid.is_empty());

    let height = grid.len();
    let width = grid[0].len();

    world.tiles = grid.into_iter().flatten().collect();
    world.width = width;
    world.height = height;

    Ok(world)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum EntityType {
    Elf,
    Goblin,
}

impl Default for EntityType {
    fn default() -> Self { EntityType::Elf }
}

impl EntityType {
    fn as_char(&self) -> char {
        match self {
            EntityType::Elf => 'E',
            EntityType::Goblin => 'G',
        }
    }

    fn enemy_type(&self) -> Self {
        match self {
            EntityType::Elf => EntityType::Goblin,
            EntityType::Goblin => EntityType::Elf,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Tile {
    Wall,
    Empty,
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Empty => '.',
        }
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Empty }
}

fn reconstruct_path(came_from: HashMap<Vector2, Vector2>, mut current: Vector2) -> Vec<Vector2> {
    let mut path = vec![current];

    while let Some(cur) = came_from.get(&current) {
        current = *cur;
        path.push(current);
    }

    path.pop();

    path.reverse();

    path
}

type EntityMap<T> = GenerationArray<T>;
type Entity = Index;

struct World {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    allocator: IndexAllocator,

    position_components: EntityMap<Vector2>,
    health_components: EntityMap<i32>,
    type_components: EntityMap<EntityType>,
    attack_components: EntityMap<i32>,

    entities: Vec<Entity>,
}

fn index(w: usize, v: Vector2) -> usize {
    v.y as usize * w + v.x as usize
}

fn is_inside(w: usize, h: usize, v: Vector2) -> bool {
    (v.x >= 0 && v.x < w as i32)
    && (v.y >= 0 && v.y < h as i32)
}

impl World {
    fn empty() -> Self {
        World {
            tiles: Default::default(),
            width: Default::default(),
            height: Default::default(),
            allocator: Default::default(),
            position_components: Default::default(),
            type_components: Default::default(),
            health_components: Default::default(),
            attack_components: Default::default(),
            entities: Default::default(),
        }
    }

    fn render(&self) {
        eprintln!("{}", self.render_to_string());
        eprintln!("{}", self.debug_entities());
    }

    fn is_free(&self, pos: Vector2) -> bool {
        if is_inside(self.width, self.height, pos) {
            let index = index(self.width, pos);
            if self.tiles[index] == Tile::Empty {
                let any_pos = self.position_components.iter()
                    .any(|&p| p == pos);

                !any_pos
            } else {
                false
            }
        } else {
            false
        }
    }

    fn total_hp(&self) -> i32 {
        self.health_components
            .iter()
            .filter(|hp| **hp > 0)
            .sum()
    }


    fn update(&mut self) -> bool {

        let entities: Vec<_> = self.entities.iter().cloned().collect();

        let mut empty = true;

        for entity in entities {
            if !self.is_alive(entity) {
                continue;
            }

            if !self.any_enemies_alive(entity) {
                eprintln!("{} found no enemies", entity.index);
                return true;
            }

            if let Some(target) = self.find_target_in_range(entity) {
                self.entity_attack(entity, target);
                empty = false;
            } else {
                let paths = self.find_target_paths(entity);
                if !paths.is_empty() {
                    empty = false;
                }

                for (target, path) in paths.iter() {
                    eprintln!("{:?} -> {:?} {:?}", entity.index, target.index, path);
                }

                if let Some((target, path)) = paths.first() {
                    // Next to an enemy
                    if !path.is_empty() {
                        // self.entity_move_towards(attacker, *target);
                        self.entity_move_on_path(entity, &path);

                        if let Some(new_target) = self.find_target_in_range(entity) {
                            self.entity_attack(entity, new_target);
                            empty = false;
                        }
                    }
                }
            }

        }

        if empty {
            return true;
        }

        false
    }

    fn is_alive(&self, entity: Entity) -> bool {
        self.health_components.get(entity)
            .map(|hp| *hp > 0)
            .unwrap_or(false)
    }

    fn any_enemies_alive(&self, entity: Entity) -> bool {
        if let Some(tp) = self.type_components.get(entity) {
            let enemy_type = tp.enemy_type();

            let possible_targets_count = self.entities.iter()
                .filter(|&e| self.type_components.get(*e) == Some(&enemy_type))
                .filter(|&e| self.position_components.get(*e).is_some())
                .filter(|&e| self.health_components.get(*e).map(|hp| *hp > 0).unwrap_or(false))
                .count();

            possible_targets_count > 0
        } else {
            false
        }
    }

    fn remove_entity(&mut self, entity: Entity) {

        if let Some(index) = self.entities.iter().position(|x| *x == entity) {
            eprintln!("Removing {:?}", entity);

            self.allocator.deallocate(entity);

            self.position_components.remove(entity);
            self.type_components.remove(entity);
            self.attack_components.remove(entity);
            self.health_components.remove(entity);
            self.entities.remove(index);
        }
    }

    fn add_goblin_at(&mut self, pos: Vector2) {
        let entity = self.allocator.allocate();

        self.position_components.set(entity, pos);
        self.health_components.set(entity, 200);
        self.attack_components.set(entity, 3);
        self.type_components.set(entity, EntityType::Goblin);

        self.entities.push(entity);
    }

    fn add_elf_at(&mut self, pos: Vector2) {
        let entity = self.allocator.allocate();

        self.position_components.set(entity, pos);
        self.attack_components.set(entity, 3);
        self.health_components.set(entity, 200);
        self.type_components.set(entity, EntityType::Elf);

        self.entities.push(entity);
    }

    fn debug_entity(&self, pref: &str, entity: Entity) {
        eprintln!("{}: {}", pref, self.entity_to_string(entity));
    }

    fn bfs(&self, start: Vector2, end: Vector2) -> Option<Vec<Vector2>> {
        let mut q: VecDeque<Vector2> = VecDeque::new();
        q.push_back(start);

        let mut distances: HashMap<Vector2, usize> = HashMap::new();
        distances.insert(start, 0);

        let mut came_from: HashMap<Vector2, Vector2> = HashMap::new();

        // came_from.insert(start, start);

        while !q.is_empty() {
            let current = q.pop_front().unwrap();

            for neighbour in current.around().iter().filter(|p| self.is_free(**p) || **p == end) {
                if !distances.contains_key(&neighbour) {
                    q.push_back(*neighbour);
                    distances.insert(*neighbour, 1 + distances.get(&current).unwrap());
                    came_from.insert(*neighbour, current);
                }
            }
        }

        if came_from.contains_key(&end) {
            let mut path = reconstruct_path(came_from, end);
            path.pop();
            Some(path)
        } else {
            None
        }
    }

    fn find_path(&self, start: Vector2, end: Vector2) -> Option<Vec<Vector2>> {

        fn heuristic(a: &Vector2, b: &Vector2) -> usize {
            let dx = (b.x - a.x).abs() as usize;
            let dy = (b.y - a.y).abs() as usize;
            dx + dy
        }

        let mut g_score: HashMap<Vector2, usize> = HashMap::new();
        g_score.insert(start, 0);

        let mut f_score: HashMap<Vector2, usize> = HashMap::new();
        f_score.insert(start, heuristic(&start, &end));

        let mut closed_set = HashSet::new();
        let mut open_set = vec![start];

        let mut came_from: HashMap<Vector2, Vector2> = HashMap::new();

        while !open_set.is_empty() {
            open_set.sort_by(|a, b| {
                let ascore = f_score.get(a).unwrap_or(&usize::max_value());
                let bscore = f_score.get(b).unwrap_or(&usize::max_value());

                match ascore.cmp(&bscore).reverse() {
                    Ordering::Equal => {
                        a.cmp(b).reverse()
                    },
                    other => other,
                }
            });

            let current = open_set.pop().unwrap();

            if current == end {
                let path = reconstruct_path(came_from, current);
                // eprintln!("Found: {:?}", path);
                return Some(path);
            }

            closed_set.insert(current);

            for neighbour in current.around().iter().filter(|p| self.is_free(**p) || **p == end) {
                if closed_set.contains(&neighbour) {
                    continue;
                }

                let possible_g_score = g_score[&current] + heuristic(&current, &neighbour);

                if open_set.iter().position(|x| *x == *neighbour).is_none() {
                    open_set.push(*neighbour);
                } else if let Some(g) = g_score.get(neighbour) {
                    if possible_g_score >= *g {
                        continue;
                    }
                }

                // Insert node path
                came_from.insert(*neighbour, current);

                g_score.insert(*neighbour, possible_g_score);
                f_score.insert(*neighbour, possible_g_score + heuristic(neighbour, &end));
            }
        }
        None
    }

    fn entity_move_on_path(&mut self, entity: Entity, path: &[Vector2]) {
        let new_pos = *path.first().unwrap();

        let free = {
            if let Some(_) = self.position_components.get(entity) {
                self.is_free(new_pos)
            } else {
                false
            }
        };

        if free {
            if let Some(pos) = self.position_components.get_mut(entity) {
                *pos = new_pos;
            }
        }
    }

    fn entity_move_towards(&mut self, attacker: Entity, target: Entity) {

        match (self.position_components.get(attacker), self.position_components.get(target)) {
            (Some(my_pos), Some(target_pos)) => {

                // eprintln!("path from {} to {}", my_pos, target_pos);
                // self.find_path(*my_pos, *target_pos);
                /*
                let target_around = target_pos.around();

                let mut available: Vec<_> = target_around.into_iter()
                    .filter(|p| self.is_free(**p))
                    .map(|p| {
                        let dist = manhattan_distance(my_pos, p);
                        (p, dist)
                    })
                    .collect();

                available.sort_by(|&(a, adist), &(b, bdist)| {
                    match adist.cmp(&bdist) {
                        Ordering::Equal => {
                            a.cmp(&b)
                        },
                        other => other,
                    }
                });


                if let Some((pos, _)) = available.first() {

                    let me_around = my_pos.around();

                    let mut moves: Vec<_> = me_around.into_iter()
                        .filter(|p| self.is_free(**p))
                        .map(|p| {
                            let dist = manhattan_distance(pos, p);
                            (p, dist)
                        })
                        .collect();

                    moves.sort_by(|&(a, adist), &(b, bdist)| {
                        match adist.cmp(&bdist) {
                            Ordering::Equal => {
                                a.cmp(&b)
                            },
                            other => other,
                        }
                    });

                    if let Some((mv, _)) = moves.first() {
                        // self.debug_entity("attacker", attacker);
                        // self.debug_entity("target", target);
                        // eprintln!("avilable {} {}", pos, dist);
                        // eprintln!("Moving {} -> {}", my_pos, mv);
                        *self.position_components.get_mut(attacker).unwrap() = **mv;
                    }
                }
                */


            },

            _ => {
            }
        }

    }

    fn entity_attack(&mut self, attacker: Entity, target: Entity) {
        // self.debug_entity("attacker", attacker);
        // self.debug_entity("target", target);

        match (self.attack_components.get(attacker), self.health_components.get_mut(target)) {
            (Some(attack), Some(hp)) => {
                *hp -= attack;

                if *hp < 0 {
                    self.debug_entity("died", target);
                    self.remove_entity(target);
                }
            },
            _ => {
            }
        }
    }

    fn entity_to_string(&self, entity: Entity) -> String {
        let mut buf = String::new();

        match (self.position_components.get(entity),
               self.type_components.get(entity),
               self.health_components.get(entity)) {

            (Some(pos), Some(r#type), Some(hp)) => {
                buf.push_str(&format!("{}: {}({})@{}", entity.index, r#type.as_char(), hp, pos));
            }

            _ => {}
        }

        buf
    }

    fn find_target_square(&self, entity: Entity) -> Option<Vector2> {
        match (self.type_components.get(entity), self.position_components.get(entity)) {
            (Some(my_type), Some(my_pos)) => {
                let enemy_type = my_type.enemy_type();

                let mut possible_targets: Vec<_> = self.entities.iter()
                    .filter(|&e| self.type_components.get(*e) == Some(&enemy_type))
                    .filter(|&e| self.position_components.get(*e).is_some())
                    .map(|e| *e)
                    .collect();

                None
            },
            _ => None,
        }
    }

    fn find_target_paths(&self, entity: Entity) -> Vec<(Entity, Vec<Vector2>)> {

        match (self.type_components.get(entity), self.position_components.get(entity)) {
            (Some(my_type), Some(my_pos)) => {
                let enemy_type = my_type.enemy_type();

                let mut possible_targets: Vec<_> = self.entities.iter()
                    .filter(|&e| self.type_components.get(*e) == Some(&enemy_type))
                    .filter(|&e| self.position_components.get(*e).is_some())
                    .map(|e| *e)
                    .filter_map(|e| {
                        let pos = self.position_components.get(e).unwrap();
                        let path = self.bfs(*my_pos, *pos);
                        if let Some(path) = path {
                            Some((e, path))
                        } else {
                            None
                        }
                    })
                    .collect()
                    ;

                possible_targets.sort_by(|(a, adist), (b, bdist)| {
                    match adist.len().cmp(&bdist.len()) {
                        Ordering::Equal => {
                            match adist.last().cmp(&bdist.last()) {
                                Ordering::Equal => {
                                    match adist.first().cmp(&bdist.first()) {
                                        Ordering::Equal => {
                                            let a_pos = self.position_components.get(*a).unwrap();
                                            let b_pos = self.position_components.get(*b).unwrap();
                                            a_pos.cmp(&b_pos)
                                        },
                                        other => other,
                                    }
                                },
                                other => other,
                            }
                        },
                        other => other,
                    }
                });

                possible_targets

            },
            _ => vec![]
        }
    }

    fn find_targets(&self, entity: Entity) -> Vec<(Entity, i32)> {
        let my_type = self.type_components.get(entity).unwrap();
        let my_pos = self.position_components.get(entity).unwrap();
        let enemy_type = my_type.enemy_type();

        let mut possible_targets: Vec<_> = self.entities.iter()
            .filter(|&e| self.type_components.get(*e) == Some(&enemy_type))
            .filter(|&e| self.position_components.get(*e).is_some())
            .filter(|&e| self.health_components.get(*e).is_some())
            .map(|e| *e)
            .filter_map(|e| {
                let pos = self.position_components.get(e).unwrap();
                let dist = manhattan_distance(my_pos, pos);
                if dist == 1 {
                    let hp = self.health_components.get(e).unwrap();
                    if *hp > 0 {
                        Some((e, *hp))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
            ;

        possible_targets.sort_by(|&(a, adist), &(b, bdist)| {
            match adist.cmp(&bdist) {
                Ordering::Equal => {
                    let a_pos = self.position_components.get(a).unwrap();
                    let b_pos = self.position_components.get(b).unwrap();
                    a_pos.cmp(&b_pos)
                },
                other => other,
            }
        });

        possible_targets
    }

    fn find_target_in_range(&self, entity: Entity) -> Option<Entity> {
        let targets = self.find_targets(entity);

        if targets.is_empty() {
            None
        } else {
            let (target, _) = targets.first().unwrap();

            Some(*target)
        }
    }

    fn sort_entities(&mut self) {
        let mut entities = std::mem::replace(&mut self.entities, Vec::new());

        entities.sort_by(|&a, &b| {
            let a_pos = self.position_components.get(a);
            let b_pos = self.position_components.get(b);

            a_pos.cmp(&b_pos)
        });

        std::mem::replace(&mut self.entities, entities);
    }

    // make_getter!(Vector2, get_position, position_components);
    // make_getter!(i32, get_health, health_components);
    // make_getter!(EntityType, get_type, type_components);

    fn debug_entities(&self) -> String {
        let mut output = String::new();

        for e in self.entities.iter() {
            output.push_str(&self.entity_to_string(*e));
            output.push('\n');
        }

        output
    }

    fn render_to_string(&self) -> String {
        let mut buf = String::new();

        let mut chars: Vec<char> = self.tiles.iter().map(Tile::as_char).collect();

        for &entity in &self.entities {
            match (self.position_components.get(entity), self.type_components.get(entity)) {
                (Some(pos), Some(tp)) => {
                    chars[index(self.width, *pos)] = tp.as_char();
                },
                _ => { }
            }
        }

        buf.push(' ');
        buf.push(' ');

        for ind in 0..self.width {
            buf.push_str(&format!("{}", ind % 10));
        }
        for (ind, ch) in chars.into_iter().enumerate() {
            if ind % self.width == 0 {
                buf.push('\n');
                buf.push_str(&format!("{} ", (ind / self.width) % 10));
            }

            buf.push(ch);
        }
        buf.push('\n');

        buf
    }
}

macro_rules! impl_index  {
    ($type: ty; $v: pat => $x: expr, $y: expr) => {
        impl std::ops::Index<$type> for World {
            type Output = Tile;

            fn index(&self, $v: $type) -> &Self::Output {
                &self.tiles[$y as usize * self.width + $x as usize]
            }
        }
    };
}

impl_index!((usize, usize); xy => xy.0, xy.1);
impl_index!(Vector2; xy => xy.x, xy.y);


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1_example_inputs() {
        let example1 = r"
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
        ";

        let movement = r"
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
        ";

        let example0 = r"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
        ";

        let example2 = r"
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
        ";

        let example3 = r"
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
        ";
        let example4 = r"
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
        ";

        assert_eq!(54 * 536, part1(example4.trim()).unwrap());
        assert_eq!(20 * 937, part1(example3.trim()).unwrap());
        assert_eq!(47 * 590, part1(example0.trim()).unwrap());
        assert_eq!(37 * 982, part1(example1.trim()).unwrap());
        assert_eq!(18 * 1543, part1(movement.trim()).unwrap());
        assert_eq!(46 * 859, part1(example2.trim()).unwrap());
    }

    #[test]
    fn more_tests() {
        let sample1 = r"
#######
#######
#.E..G#
#.#####
#G#####
#######
#######
        ";

        let sample2 = r"
####
#GG#
#.E#
####
        ";

        let sample3 = r"
########
#..E..G#
#G######
########
        ";

        let targets = r"
#######
#E..G.#
#...#.#
#.G.#G#
#######
        ";
        let sample4 = r"
#######
#..E#G#
#.....#
#G#...#
#######
        ";
        assert_eq!(33 * 501, part1(sample4.trim()).unwrap());
        assert_eq!(33 * 501, part1(targets.trim()).unwrap());
        assert_eq!(34 * 301, part1(sample1.trim()).unwrap());
        assert_eq!(33 * 301, part1(sample2.trim()).unwrap());
        assert_eq!(34 * 301, part1(sample3.trim()).unwrap());

    }

}