use aoc::Result;
use std::collections::VecDeque;
use std::fmt;

type Meta = Vec<u32>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
struct Node {
    children: Vec<NodeId>,
    meta: Meta,
}

impl Node {
    pub fn sum(&self) -> u32 {
        self.meta.iter().sum::<u32>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct NodeId {
    index: usize,
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    pub fn new_node(&mut self, data: Meta) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(Node {
            children: Vec::new(),
            meta: data,
        });

        NodeId { index }
    }

    pub fn root(&self) -> Option<&Node> {
        self.nodes.get(0)
    }

    /// Get a reference to the node with the given id if in the arena, None
    /// otherwise.
    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id.index)
    }

    /// Get a mutable reference to the node with the given id if in the arena,
    /// None otherwise.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id.index)
    }

    /// Iterate over all nodes in the arena in storage-order.
    ///
    /// Note that this iterator also contains removed elements, which can be
    /// tested with the `is_removed()` method on the node.
    pub fn iter(&self) -> std::slice::Iter<Node> {
        self.nodes.iter()
    }
}

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    part1(&s)?;
    part2(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<usize> {
    let values: Result<VecDeque<u32>> = s
        .split(' ')
        .map(|v| v.parse::<u32>().map_err(|err| err.into()))
        .collect();

    let mut values = values?;

    let mut tree = Tree::new();

    read_tree(&mut tree, &mut values);

    let result: u32 = tree.iter().map(|v| v.sum()).sum();

    eprintln!("part1: {:?}", result);

    Ok(result as usize)
}

fn part2(s: &str) -> Result<usize> {
    let values: Result<VecDeque<u32>> = s
        .split(' ')
        .map(|v| v.parse::<u32>().map_err(|err| err.into()))
        .collect();

    let mut values = values?;

    let mut tree = Tree::new();

    read_tree(&mut tree, &mut values);

    let result = count_tree(&tree);

    eprintln!("part2: {:?}", result);

    Ok(result as usize)
}

fn count_tree(tree: &Tree) -> u32 {
    fn count_node(tree: &Tree, node: &Node) -> u32 {
        if node.children.is_empty() {
            node.sum()
        } else {
            let mut sum = 0;
            for index in node.meta.iter() {
                if *index == 0 {
                    continue;
                }

                let index = *index - 1;
                let child = node.children.get(index as usize);
                if let Some(id) = child {
                    let child = tree.get(*id).unwrap();
                    sum += count_node(tree, child)
                }
            }
            sum
        }
    }

    let root = tree.root().unwrap();

    count_node(tree, root)
}

fn read_tree(tree: &mut Tree, values: &mut VecDeque<u32>) -> NodeId {
    let children = values.pop_front().unwrap();
    let metacount = values.pop_front().unwrap();

    // eprintln!("children {} metacount {}", children, metacount);

    let mut nodes = Vec::new();

    let node = tree.new_node(Vec::new());

    for _ in 0..children {
        nodes.push(read_tree(tree, values));
    }

    let mut metas = Vec::new();

    for _ in 0..metacount {
        metas.push(values.pop_front().unwrap());
    }

    tree.get_mut(node).unwrap().children = nodes;
    tree.get_mut(node).unwrap().meta = metas;

    node
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part1_example_input() {
        assert_eq!(138, part1(INPUT.trim()).unwrap());
    }

    #[test]
    fn part2_example_input() {
        assert_eq!(66, part2(INPUT.trim()).unwrap());
    }
}
