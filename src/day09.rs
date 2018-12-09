use aoc::{Result, CustomError};
use std::fmt;
use lazy_static::lazy_static;
use std::ops::{Index, IndexMut};
use regex::Regex;

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s, 100)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Node {
    left: Option<NodeId>,
    right: Option<NodeId>,
    value: usize,
    index: usize,
}

impl Node {
    pub fn null() -> Self {
        Node {
            left: None,
            right: None,
            value: usize::max_value(),
            index: usize::max_value(),
        }
    }

    // Returns `true` if this pointer is null.
    #[inline]
    fn is_null(&self) -> bool {
        *self == Node::null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct NodeId {
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct List {
    nodes: Vec<Node>,
    count: usize,
}

impl List {
    pub fn new_cap(cap: usize) -> Self {
        List {
            nodes: Vec::with_capacity(cap),
            count: 0,
        }
    }

    pub fn new_node(&mut self, value: usize) -> NodeId {
        let index = self.nodes.len();

        let node = NodeId { index };

        self.nodes.push(Node {
            left: Some(node),
            right: Some(node),
            value,
            index,
        });

        self.count += 1;

        node
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn next_right(&self, id: NodeId) -> NodeId {
        let mut result = id;

        if let Some(next) = self.get(result) {
            if let Some(right) = next.right {
                result = right;
            }
        }

        result
    }

    pub fn next_left(&self, id: NodeId) -> NodeId {
        let mut result = id;

        if let Some(next) = self.get(result) {
            if let Some(left) = next.left {
                result = left;
            }
        }

        result
    }

    pub fn remove(&mut self, node: NodeId) -> Node {

        let left_index = self[node].left;
        let right_index = self[node].right;

        if let Some(left) = left_index.and_then(|n| self.get_mut(n)) {
            left.right = right_index;
        }

        if let Some(right) = right_index.and_then(|n| self.get_mut(n)) {
            right.left = left_index;
        }

        self.count -= 1;

        std::mem::replace(&mut self[node], Node::null())
    }

    pub fn root(&self) -> Option<(usize, &Node)> {
        let mut iter = self.nodes.iter()
            .enumerate()
            .filter(|(_, v)| !v.is_null());
        iter.next()
    }

    pub fn insert_after(&mut self, left: NodeId, value: usize) -> NodeId {
        let new = self.new_node(value);

        let (lft, right) = match self.get(left) {
            Some(node) => (Some(left), node.right),
            None => (None, None),
        };

        if let Some(node) = lft.and_then(|v| self.get_mut(v)) {
            node.right = Some(new);
        }

        if let Some(node) = right.and_then(|v| self.get_mut(v)) {
            node.left = Some(new);
        }

        if let Some(node) = self.get_mut(new) {
            node.left = lft;
            node.right = right;
        }

        new
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id.index)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id.index)
    }

    pub fn iter(&self) -> NodeIter {
        let (index, root) = match self.root() {
            Some((i, r)) => (i, Some(r)),
            None => (!0, None),
        };

        NodeIter {
            list: self,
            next: root,
            root_id: NodeId { index },
        }
    }
}

struct NodeIter<'a> {
    list: &'a List,
    next: Option<&'a Node>,
    root_id: NodeId,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.next.take();

        if let Some(node) = out {
            if let Some(right) = node.right {
                if right != self.root_id {
                    self.next = self.list.get(right);
                }
            }
        }

        out
    }
}


// Just for convenience, so that we can type `self[i]` instead of `self.nodes[i]`.
impl Index<NodeId> for List {
    type Output = Node;

    fn index(&self, index: NodeId) -> &Node {
        &self.nodes[index.index]
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.nodes[i]`.
impl IndexMut<NodeId> for List {
    fn index_mut(&mut self, index: NodeId) -> &mut Node {
        &mut self.nodes[index.index]
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[derive(Debug, Clone, Default)]
struct Player {
    id: u32,
    marbles: Vec<usize>,
}

impl Player {
    fn score(&self) -> usize {
        self.marbles.iter()
            .sum()
    }
}

fn part1(s: &str) -> Result<usize> {
    part2(s, 1)
}

fn part2(s: &str, multiplier: usize) -> Result<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(\d+) players; last marble is worth (\d+) points.*"
        ).unwrap();
    }

    let caps = RE
        .captures(s)
        .ok_or_else(|| CustomError("Invalid captures".to_owned()))?;

    let player_count: usize = aoc::get_value(&caps, 1)?;
    let mut last_points: usize = aoc::get_value(&caps, 2)?;
    last_points *= multiplier;

    eprintln!("players {} last_points {}", player_count, last_points);

    let mut marbles = List::new_cap(last_points as usize);
    let mut current_marble = 0;
    let mut current_index = marbles.new_node(current_marble);
    current_marble += 1;

    let mut players = (0..player_count)
        .map(|i| {
            Player {
                id: i as u32 + 1,
                marbles: Vec::new(),
            }
        })
        .collect::<Vec<_>>();

    'game: loop {
        for player in players.iter_mut() {
            place_marble(&mut current_marble, &mut marbles, &mut current_index, player);

            if current_marble > last_points {
                break 'game;
            }
        }

        if current_marble >= last_points {
            break;
        }
    }

    let max = players.iter()
        .max_by_key(|p| p.score());

    if let Some(player) = max {
        eprintln!("Winner {} score {}", player.id, player.score());

        return Ok(player.score());
    }

    Ok(0)
}

#[allow(dead_code)]
fn show_marbles(current_index: NodeId, marbles: &List, player: Option<u32>) {
    let mut output = String::new();

    if let Some(p) = player {
        output.push_str(&format!("[{:^3}]", p));
    } else {
        output.push_str("[ - ]");
    }

    let mut iter = marbles.iter();

    if let Some(marble) = iter.next() {
        if marble.index == current_index.index {
            output.push_str(&format!(" ({})", marble.value));
        } else {
            output.push_str(&format!("  {} ", marble.value));
        }
    }

    for marble in  iter {
        if marble.index == current_index.index {
            output.push_str(&format!(" ({})", marble.value));
        } else {
            output.push_str(&format!("  {} ", marble.value));
        }
    }

    eprintln!("{}", output);
}

fn place_marble(current: &mut usize, marbles: &mut List, current_index: &mut NodeId, player: &mut Player) {
    if marbles.len() == 1 {
        *current_index = marbles.insert_after(*current_index, *current);
    } else if marbles.len() == 2 {
        *current_index = marbles.insert_after(marbles.next_right(*current_index), *current);
    } else if *current % 23 == 0 {
        let mut wrapped_index = *current_index;

        for _ in 0..7 {
            wrapped_index = marbles.next_left(wrapped_index);
        }

        let new_current = marbles.next_right(wrapped_index);

        let item = marbles.remove(wrapped_index);

        player.marbles.push(*current);
        player.marbles.push(item.value);

        *current_index = new_current;
    } else {
        *current_index = marbles.insert_after(marbles.next_right(*current_index), *current);
    }
    *current += 1;
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"
9 players; last marble is worth 25 points: high score is 32
10 players; last marble is worth 1618 points: high score is 8317
13 players; last marble is worth 7999 points: high score is 146373
17 players; last marble is worth 1104 points: high score is 2764
21 players; last marble is worth 6111 points: high score is 54718
30 players; last marble is worth 5807 points: high score is 37305
    ";
    const SCORES: [usize; 6] = [
        32,
        8317,
        146373,
        2764,
        54718,
        37305
    ];
    #[test]
    fn part1_example_input() {
        for (i, line) in INPUT.trim().lines().enumerate() {
            let score = SCORES[i];

            assert_eq!(score, part1(line).unwrap());
        }
    }
}