use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
struct LicenseNode {
    id: u32,
    parent: Option<u32>,
    value: u32,
    childs: VecDeque<u32>,
    metadata: VecDeque<u32>,
    metadata_value: u32,
}

impl LicenseNode {
    fn new(n_id: u32, p_id: Option<u32>) -> LicenseNode {
        LicenseNode {
            id: n_id,
            parent: p_id,
            value: 0,
            childs: VecDeque::new(),
            metadata: VecDeque::new(),
            metadata_value: 0,
        }
    }

    fn compute_value(&self, license_tree: &Vec<LicenseNode>) -> u32 {
        let mut value = 0;
        if self.childs.is_empty() {
            return self.value;
        }
        for m in &self.metadata {
            if let Some(child_id) = self.childs.get((*m - 1) as usize) {
                let child = license_tree
                    .iter()
                    .find(|x| x.id == *child_id)
                    .expect("Child not found!");
                value += child.compute_value(license_tree);
            }
        }

        value
    }
}

struct NodeIdGenerator {
    state: u32,
}

impl NodeIdGenerator {
    fn new() -> NodeIdGenerator {
        NodeIdGenerator { state: 0 }
    }

    fn next(&mut self) -> u32 {
        self.state += 1;
        self.state
    }
}

fn parse_license_tree(
    license_nodes: &Vec<u32>,
    index: &mut usize,
    parent_id: Option<u32>,
    node_gen: &mut NodeIdGenerator,
) -> Vec<LicenseNode> {
    let child_cnt = license_nodes[*index];
    *index += 1;
    let metadata_cnt = license_nodes[*index];
    *index += 1;

    let node_id = node_gen.next();
    let mut tree_nodes = Vec::new();
    let mut node = LicenseNode::new(node_id, parent_id);
    for _ in 0..child_cnt {
        let sub_tree = parse_license_tree(license_nodes, index, Some(node_id), node_gen);
        for tn in sub_tree {
            if tn.parent == Some(node_id) {
                node.childs.push_back(tn.id);
            }
            tree_nodes.push(tn);
        }
    }

    for _ in 0..metadata_cnt {
        node.metadata.push_back(license_nodes[*index]);
        node.metadata_value += node.metadata.back().unwrap();
        *index += 1;
    }

    if node.childs.is_empty() {
        node.value = node.metadata_value;
    }

    tree_nodes.push(node);
    tree_nodes
}

pub fn solve() {
    let mut puzzle_input = BufReader::new(File::open("./input/day08.txt").unwrap());
    let mut license_nodes_raw = String::new();
    let _result = puzzle_input.read_to_string(&mut license_nodes_raw);
    let license_nodes: Vec<u32> = license_nodes_raw
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut node_gen = NodeIdGenerator::new();
    let license_tree = parse_license_tree(&license_nodes, &mut 0, None, &mut node_gen);

    let metadata_sum = license_tree
        .iter()
        .fold(0u32, |acc, x| acc + x.metadata_value);

    println!(
        "Day 08 part 1: sum of all metadata entries is {}",
        metadata_sum
    );

    let root = license_tree
        .iter()
        .find(|x| x.parent == None)
        .expect("Tree root not found!");
    let root_value = root.compute_value(&license_tree);
    println!("Day 08 part 2: value of root node is {}", root_value);
}
