extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate multiarray;
extern crate chrono;
extern crate array_init;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    println!("Hello, world!");

    day01::solve_part1();
    day01::solve_part2();
    day02::solve_part1();
    day02::solve_part2();
    day03::solve_part1();
    day03::solve_part2();
    day04::solve();
}
