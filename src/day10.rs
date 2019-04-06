use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn new(s: &String) -> Point {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "position=<\\s*(-?\\d+),\\s*(-?\\d+)> velocity=<\\s*(-?\\d+),\\s*(-?\\d+)>"
            )
            .unwrap();
        }

        let point_def = RE.captures(s).unwrap();
        Point {
            position: (point_def[1].parse().unwrap(), point_def[2].parse().unwrap()),
            velocity: (point_def[3].parse().unwrap(), point_def[4].parse().unwrap()),
        }
    }

    fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

pub fn solve() {
    let puzzle_input = BufReader::new(File::open("./input/day10.txt").unwrap());
    let mut points: Vec<Point> = puzzle_input
        .lines()
        .map(|l| Point::new(&l.unwrap()))
        .collect();

    let seconds_to_simulate = 50000;
    let mut second = 1;
    while second <= seconds_to_simulate {
        for p in &mut points {
            p.update();
        }
        second += 1;
        let mut min_x = 100000;
        let mut min_y = 100000;
        let mut max_x = -100000;
        let mut max_y = -100000;
        for p in &points {
            if p.position.0 < min_x {
                min_x = p.position.0;
            }
            if p.position.0 > max_x {
                max_x = p.position.0;
            }
            if p.position.1 < min_y {
                min_y = p.position.1;
            }
            if p.position.1 > max_y {
                max_y = p.position.1;
            }
        }

        if second > 10243 && second < 10247 {
            // println!("second: {}", second);
            let mut _star_map = [[' '; 200]; 200];
            for p in &points {
                _star_map[p.position.1 as usize][p.position.0 as usize] = 'X';
            }

            // for r in star_map.iter() {
            // for rr in r.iter() {
            // print!("{} ", rr);
            // }
            // println!("");
            // }
        }

        // if min_x > 0 && min_y > 0 && max_x < 200 && max_y < 200 {
        //     println!("min: {},{}, max: {},{} at {}", min_x, min_y, max_x, max_y, second);

        // }
    }
}
