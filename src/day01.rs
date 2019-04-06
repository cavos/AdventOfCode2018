// mod day01 {

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_part1() {
    let buffered = BufReader::new(File::open("./input/day01.txt").unwrap());
    let result = buffered
        .lines()
        .map(|l| l.unwrap())
        .fold(0i32, |freq, x| freq + x.parse::<i32>().unwrap());
    println!("Day 01 pt1 : {:}", result);
}

pub fn solve_part2() {
    let buffered = BufReader::new(File::open("./input/day01.txt").unwrap());
    let freqs: Vec<i32> = buffered
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut found = false;
    let mut freq = 0;
    let mut it = freqs.iter();
    let mut sum_freqs: BTreeSet<i32> = BTreeSet::new();
    while !found {
        let f = it.next();
        if f.is_some() {
            freq += f.unwrap();
            if !sum_freqs.insert(freq) {
                found = true;
            }
        } else {
            it = freqs.iter();
        }
    }
    println!("Day 01 pt2 : {:}", freq);
}
// }
