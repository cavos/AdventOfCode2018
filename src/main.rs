extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate array_init;
extern crate chrono;
extern crate multiarray;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    println!("Hello, world!");

    day01::solve_part1();
    day01::solve_part2();
    day02::solve_part1();
    day02::solve_part2();
    day03::solve_part1();
    day03::solve_part2();
    day04::solve();
    day05::solve_part1();
    // day05::solve_part2();
    day06::solve_part1();
    day06::solve_part2();
    day07::solve();
    day08::solve();
    day09::solve();
    day10::solve();
    day11::solve();
}
