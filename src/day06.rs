use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(s: &String) -> Point {
        let coords: Vec<&str> = s.split(", ").collect();

        Point {
            x: coords[0].parse().unwrap(),
            y: coords[1].parse().unwrap(),
        }
    }
}

fn append_neighbours(id: &u8, pos: &Point, que: &mut VecDeque<(u8, Point)>) {
    //above
    if pos.x > 0 {
        que.push_back((
            *id,
            Point {
                x: pos.x - 1,
                y: pos.y,
            },
        ));
    }
    //below
    if pos.x < 499 {
        que.push_back((
            *id,
            Point {
                x: pos.x + 1,
                y: pos.y,
            },
        ));
    }
    //left
    if pos.y > 0 {
        que.push_back((
            *id,
            Point {
                x: pos.x,
                y: pos.y - 1,
            },
        ));
    }
    //right
    if pos.y < 499 {
        que.push_back((
            *id,
            Point {
                x: pos.x,
                y: pos.y + 1,
            },
        ));
    }
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as usize
}

pub fn solve_part1() {
    let puzzle_input = BufReader::new(File::open("./input/day06.txt").unwrap());
    let mut id = 0u8;
    let coordinates: BTreeMap<u8, Point> = puzzle_input
        .lines()
        .map(|l| {
            id += 1;
            (id, Point::new(&l.unwrap()))
        })
        .collect();

    let mut visit_que: VecDeque<(u8, Point)> = VecDeque::new();
    for c in &coordinates {
        visit_que.push_back((*c.0, c.1.clone()));
    }

    let mut map = [[0u8; 500]; 500];
    let no_mans_tile = 255;
    let free_tile = 0;
    let mut areas: BTreeMap<u8, usize> = BTreeMap::new();

    while visit_que.len() > 0 {
        let v = visit_que.pop_front().unwrap();

        if map[v.1.y][v.1.x] == free_tile {
            map[v.1.y][v.1.x] = v.0;
            areas.entry(v.0).and_modify(|x| *x += 1).or_insert(1);
            append_neighbours(&v.0, &v.1, &mut visit_que);
        } else if map[v.1.y][v.1.x] != no_mans_tile && map[v.1.y][v.1.x] != v.0 {
            let a = manhattan_distance(&v.1, coordinates.get(&map[v.1.y][v.1.x]).unwrap());
            let b = manhattan_distance(&v.1, coordinates.get(&v.0).unwrap());
            if b < a {
                areas
                    .entry(map[v.1.y][v.1.x])
                    .and_modify(|x| *x -= 1)
                    .or_insert(0);
                areas.entry(v.0).and_modify(|x| *x += 1).or_insert(1);
                map[v.1.y][v.1.x] = v.0;
                append_neighbours(&v.0, &v.1, &mut visit_que);
            } else if b == a {
                areas
                    .entry(map[v.1.y][v.1.x])
                    .and_modify(|x| *x -= 1)
                    .or_insert(0);
                map[v.1.y][v.1.x] = no_mans_tile;
            }
        }
    }

    let mut edge: BTreeSet<u8> = BTreeSet::new();
    for i in 0..map.len() {
        if i == 0 || i == map.len() - 1 {
            for x in 0..map[i].len() {
                edge.insert(map[i][x]);
            }
        } else {
            edge.insert(map[i][0]);
            edge.insert(map[i][map[i].len() - 1]);
        }
    }

    println!(
        "Day 06 pt 1: largest area is {:?}",
        areas
            .iter()
            .max_by_key(|x| if edge.contains(&x.0) {
                0 as usize
            } else {
                *x.1
            })
            .unwrap()
    );
}

fn append_neighbours_pt2(pos: &Point, que: &mut VecDeque<Point>) {
    //above
    if pos.x > 0 {
        que.push_back(Point {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    //below
    if pos.x < 499 {
        que.push_back(Point {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    //left
    if pos.y > 0 {
        que.push_back(Point {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    //right
    if pos.y < 499 {
        que.push_back(Point {
            x: pos.x,
            y: pos.y + 1,
        });
    }
}

fn compute_distance(p: &Point, coords: &Vec<Point>) -> u16 {
    let mut distance = 0u16;
    for c in coords {
        distance += manhattan_distance(p, c) as u16;
    }

    distance
}

pub fn solve_part2() {
    let puzzle_input = BufReader::new(File::open("./input/day06.txt").unwrap());
    let coordinates: Vec<Point> = puzzle_input
        .lines()
        .map(|l| Point::new(&l.unwrap()))
        .collect();

    let mut start_point = (0, 0);
    for c in &coordinates {
        start_point.0 += c.x;
        start_point.1 += c.y;
    }

    let mut visit_que: VecDeque<Point> = VecDeque::new();
    visit_que.push_back(Point {
        x: start_point.0 / coordinates.len(),
        y: start_point.1 / coordinates.len(),
    });

    let mut map = [[0u16; 500]; 500];
    let max_distance = 10000;
    let mut area = 0;
    while visit_que.len() > 0 {
        let v = visit_que.pop_front().unwrap();
        if map[v.y][v.x] == 0 {
            map[v.y][v.x] = compute_distance(&v, &coordinates);
            if map[v.y][v.x] < max_distance {
                append_neighbours_pt2(&v, &mut visit_que);
                area += 1;
            }
        }
    }

    println!("Day 06 pt 2: safe area is of size {:?}", area);
}
