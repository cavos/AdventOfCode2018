use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_part1() {
    let puzzle_input = BufReader::new(File::open("./input/day02.txt").unwrap());
    let box_ids = puzzle_input.lines().map(|l| l.unwrap());

    let mut twos_count = 0u32;
    let mut threes_count = 0u32;
    for box_id in box_ids {
        let mut chars: BTreeMap<char, u8> = BTreeMap::new();
        for c in box_id.chars() {
            chars.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        if chars.values().any(|&x| x == 2u8) {
            twos_count += 1;
        }
        if chars.values().any(|&x| x == 3u8) {
            threes_count += 1;
        }
    }

    println!(
        "Day 02 pt1:{} * {} = {}",
        twos_count,
        threes_count,
        threes_count * twos_count
    );
}

pub fn solve_part2() {
    let puzzle_input = BufReader::new(File::open("./input/day02.txt").unwrap());
    let box_ids: Vec<String> = puzzle_input.lines().map(|l| l.unwrap()).collect();

    for i in 0..box_ids.len() {
        for j in i + 1..box_ids.len() {
            let test: Vec<(char, char)> = box_ids[i].chars().zip(box_ids[j].chars()).collect();
            let mut dif_count = 0;
            for x in &test {
                if x.0 != x.1 {
                    dif_count += 1;
                }
                if dif_count > 1 {
                    break;
                }
            }
            if dif_count == 1 {
                let result = test.iter().fold(String::new(), |mut s, c| {
                    if c.0 == c.1 {
                        s.push(c.0);
                        s
                    } else {
                        s
                    }
                });
                println!("Day 02 pt2: {}", result);
                break;
            }
        }
    }
}
