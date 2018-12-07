use std::fs::File;
use std::io::{BufRead, BufReader};

fn reduce(in_polymer_chain : Vec<char>) -> Vec<char> {
    let mut polymer_chain = in_polymer_chain;
    let mut reduced = true;
    while reduced {
        reduced = false;
        let mut new_chain: Vec<char> = Vec::new();
        let mut i = 0;
        while i < polymer_chain.len() - 1 {
            if (polymer_chain[i] as i16 - polymer_chain[i+1] as i16).abs() == 32 {
                i += 1;
                reduced = true;
            }
            else {
                new_chain.push(polymer_chain[i]);
            }
            i += 1;
        }
        new_chain.push(*polymer_chain.last().unwrap());
        polymer_chain = new_chain;
    }
    polymer_chain
}

pub fn solve_part1() {
    let mut puzzle_input = BufReader::new(File::open("./input/day05.txt").unwrap());
    let mut line = String::new();
    let _result = puzzle_input.read_line(&mut line);
    let mut polymer_chain : Vec<char> = line.chars().collect();

    polymer_chain = reduce(polymer_chain);
    println!("Day 05 pt 1: polymer contains {} units", polymer_chain.len());
}

pub fn solve_part2() {
    let mut puzzle_input = BufReader::new(File::open("./input/day05.txt").unwrap());
    let mut line = String::new();
    let _result = puzzle_input.read_line(&mut line);
    let polymer_chain : Vec<char> = line.chars().collect();

    //<a..z>
    let mut best_size = polymer_chain.len();
    for u in 97..123 {
        let mut modified_chain : Vec<char>;
        modified_chain = polymer_chain.iter().filter(|&&x| x as i32 != u && x as i32 != u - 32).map(|&x| x).collect();
        modified_chain = reduce(modified_chain);
        if best_size > modified_chain.len() {
            best_size = modified_chain.len();
        }
    }
    println!("Day 05 pt 2: shortest polymer consists of {} units", best_size);
}