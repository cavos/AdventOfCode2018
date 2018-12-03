extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate multiarray;

mod day01;
mod day02;
mod day03;

fn main() {
    println!("Hello, world!");

    day01::solve_part1();
    day01::solve_part2();
    day02::solve_part1();
    day02::solve_part2();
    day03::solve_part1();
    day03::solve_part2();
}
