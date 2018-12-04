use chrono::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum GuardRecordType {
    ShiftStart(u32),
    NapStart,
    NapEnd,
}

#[derive(Debug)]
struct GuardRecord {
    datetime: NaiveDateTime,
    record_type: GuardRecordType,
}

impl GuardRecord {
    fn new(log_entry: &str) -> GuardRecord {
        //Avoid compiling the same regex
        lazy_static! {
            static ref RE_DATE: Regex =
                Regex::new("\\[(\\d{4}-\\d{2}-\\d{2}\\s\\d{2}:\\d{2})\\]\\s(.+)").unwrap();
            static ref RE_GUARDID: Regex = Regex::new("Guard #(\\d+)\\sbegins\\sshift").unwrap();
        }

        let caps = RE_DATE.captures(log_entry).unwrap();
        let dt = NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M").unwrap();
        let rt;
        if "wakes up" == &caps[2] {
            rt = GuardRecordType::NapEnd;
        } else if "falls asleep" == &caps[2] {
            rt = GuardRecordType::NapStart;
        } else {
            let guarcap = RE_GUARDID.captures(&caps[2]).unwrap();
            rt = GuardRecordType::ShiftStart(guarcap[1].parse().unwrap());
        }

        GuardRecord {
            datetime: dt,
            record_type: rt,
        }
    }
}

impl PartialOrd for GuardRecord {
    fn partial_cmp(&self, other: &GuardRecord) -> Option<Ordering> {
        Some(self.datetime.cmp(&other.datetime))
    }
}

impl Ord for GuardRecord {
    fn cmp(&self, other: &GuardRecord) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl PartialEq for GuardRecord {
    fn eq(&self, other: &GuardRecord) -> bool {
        self.datetime == other.datetime
    }
}

impl Eq for GuardRecord {}

pub fn solve() {
    let puzzle_input = BufReader::new(File::open("./input/day04.txt").unwrap());
    let mut guard_log: Vec<GuardRecord> = puzzle_input
        .lines()
        .map(|l| GuardRecord::new(&l.unwrap()))
        .collect();

    guard_log.sort();
    let mut guard_naps: BTreeMap<u32, [u16; 60]> = BTreeMap::new();
    let mut guard_id = None;
    let mut nap_start = None;
    for gl in guard_log {
        match gl.record_type {
            GuardRecordType::ShiftStart(id) => guard_id = Some(id),
            GuardRecordType::NapStart => nap_start = Some(gl.datetime),
            GuardRecordType::NapEnd => {
                let start = nap_start.unwrap().time().minute() as usize;
                let end = gl.datetime.time().minute() as usize;
                guard_naps
                    .entry(guard_id.unwrap())
                    .and_modify(|nap_times| {
                        for m in start..end {
                            nap_times[m] += 1;
                        }
                    }).or_insert(array_init::array_init(|x| {
                        if start <= x && x < end {
                            1
                        } else {
                            0
                        }
                    }));
            }
        }
    }

    // part 1
    let mut longest_nap = 0;
    let mut minute = 0;
    for x in &guard_naps {
        let nap_duration = x.1.iter().fold(0, |acc, x| acc + x);
        if nap_duration > longest_nap {
            longest_nap = nap_duration;
            guard_id = Some(*x.0);
            minute =
                x.1.iter()
                    .enumerate()
                    .max_by_key(|&(_, item)| item)
                    .unwrap()
                    .0;
        }
    }

    println!(
        "Day 04 pt 1: guard #{} took {} minutes nap, {} * {} = {}",
        guard_id.unwrap(),
        longest_nap,
        guard_id.unwrap(),
        minute,
        guard_id.unwrap() * minute as u32
    );

    // part 2
    let mut max_nap_cnt = 0u16;
    for x in guard_naps {
        let nap_cnt =
            x.1.iter()
                .enumerate()
                .max_by_key(|&(_, item)| item)
                .unwrap();

        if nap_cnt.1 > &max_nap_cnt {
            max_nap_cnt = *nap_cnt.1;
            minute = nap_cnt.0;
            guard_id = Some(x.0);
        }
    }

    println!(
        "Day 04 pt 1: guard #{} took {} naps at {} minute = {}",
        guard_id.unwrap(),
        max_nap_cnt,
        minute,
        guard_id.unwrap() * minute as u32
    );
}
