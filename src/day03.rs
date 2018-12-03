use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;
use std::collections::BTreeMap;


#[derive(Debug)]
struct FabricClaim {
    id : i32,
    top_x : usize,
    top_y : usize,
    width : usize,
    height : usize,
    overlaps : bool
}

impl FabricClaim {
    fn new(s : &String) -> FabricClaim {
        //Avoid compiling the same regex
        lazy_static! {
            static ref RE: Regex = Regex::new("#(\\d+)\\s@\\s(\\d+),(\\d+):\\s(\\d+)x(\\d+)").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        FabricClaim { id : caps[1].parse().unwrap(),
                    top_x : caps[2].parse().unwrap(),
                    top_y : caps[3].parse().unwrap(),
                    width : caps[4].parse().unwrap(),
                    height : caps[5].parse().unwrap(),
                    overlaps : false}
    }

    fn overlap(&self, s : &FabricClaim) -> bool {
        if self.top_x > s.top_x + s.width || // s is on the left
           self.top_x + self.width < s.top_x || // s is on the right
           self.top_y > s.top_y + s.height || // s is above
           self.top_y + self.height  < s.top_y {
               return false
           }
        true
        }
}

pub fn solve_part1() {
    let puzzle_input = BufReader::new(File::open("./input/day03.txt").unwrap());
    let fabric_claims : Vec<FabricClaim> = puzzle_input.lines().map(|l| FabricClaim::new(&l.unwrap())).collect();

    let mut fabric : BTreeMap<(usize, usize), u16> = BTreeMap::new();
    for claim in fabric_claims {
        for y in claim.top_y..(claim.top_y + claim.height) {
            for x in claim.top_x..(claim.top_x + claim.width) {
                fabric.entry((y,x)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }
    println!("Day 03 pt 1: {}", fabric.values().filter(|&x| *x >= 2u16).count());
}

pub fn solve_part2() {
    let puzzle_input = BufReader::new(File::open("./input/day03.txt").unwrap());
    let mut fabric_claims : Vec<FabricClaim> = puzzle_input.lines().map(|l| FabricClaim::new(&l.unwrap())).collect();

    for i in 0..fabric_claims.len() {
        for j in i+1..fabric_claims.len() {
            if fabric_claims[i].overlap(&fabric_claims[j]) {
                fabric_claims[i].overlaps = true;
                fabric_claims[j].overlaps = true;
            }
        }
    }

    let result : &FabricClaim = fabric_claims.iter().find(|&x| !x.overlaps).unwrap();
    println!("Day 03 pt 2: {:?}", result);
}