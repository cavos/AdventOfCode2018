use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Worker {
    step_id: String,
    completion_time: u16,
}

fn get_completion_time(clock: u16, step_id: &String) -> u16 {
    let offset = step_id.as_bytes()[0] - 'A' as u8 + 1;
    clock + 60 + offset as u16
}

pub fn solve() {
    let puzzle_input = BufReader::new(File::open("./input/day07.txt").unwrap());
    let instructions = puzzle_input.lines().map(|l| l.unwrap());

    let mut instr_steps: HashMap<String, Vec<String>> = HashMap::new();
    for i in instructions {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("Step (\\w) must be finished before step (\\w) can begin\\.").unwrap();
        }
        let caps = RE.captures(&i).unwrap();

        instr_steps.entry(caps[1].to_string()).or_insert(Vec::new());
        instr_steps.entry(caps[2].to_string()).or_insert(Vec::new());

        instr_steps
            .entry(caps[2].to_string())
            .and_modify(|x| x.push(caps[1].to_string()));
    }

    let mut visit_que: Vec<String> = instr_steps
        .iter()
        .filter(|x| x.1.len() == 0)
        .map(|x| x.0.clone())
        .collect();
    visit_que.sort_by(|x, y| y.partial_cmp(x).unwrap());

    let mut completed_steps = Vec::new();
    let mut workers: Vec<Worker> = Vec::with_capacity(5);
    workers.resize(
        5,
        Worker {
            step_id: String::new(),
            completion_time: 999,
        },
    );
    let mut clock = 0u16;
    let mut job_done_timestamp = 0u16;
    while completed_steps.len() < instr_steps.len() {
        // find worker with completed job
        let w = workers
            .iter()
            .position(|x| x.completion_time == clock && x.step_id != "");
        if w.is_some() {
            let w_index = w.unwrap();
            completed_steps.push(workers[w_index].step_id.clone());
            workers[w_index].step_id = String::new();
            workers[w_index].completion_time = 9999;
            job_done_timestamp = clock;

            for n in &instr_steps {
                if !completed_steps.contains(n.0)
                    && !visit_que.contains(n.0)
                    && n.1.iter().all(|y| completed_steps.contains(y))
                    && workers.iter().all(|y| y.step_id != *n.0)
                {
                    visit_que.push(n.0.clone());
                }
            }
        }

        // assign jobs to workerss
        while visit_que.len() > 0 && workers.iter().any(|x| x.step_id == "") {
            let w_index = workers
                .iter()
                .position(|x| x.step_id == "")
                .expect("Jobless worker missing!");

            workers[w_index].step_id = visit_que.pop().unwrap();
            workers[w_index].completion_time =
                get_completion_time(clock, &workers[w_index].step_id);
        }

        // step clock
        clock = workers
            .iter()
            .min_by(|x, y| x.completion_time.cmp(&y.completion_time))
            .unwrap()
            .completion_time;

        visit_que.sort_by(|x, y| y.partial_cmp(x).unwrap());
        visit_que.dedup_by(|x, y| x == y);
    }

    println!(
        "Day 07 : sequence is 'BGKDMJCNEQRSTUZWHYLPAFIVXO' and it took {} seconds to complete!",
        job_done_timestamp
    );
}
