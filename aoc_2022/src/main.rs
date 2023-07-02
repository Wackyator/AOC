#![allow(unused)]
mod day1;
mod day2;
mod day3;

fn main() {
    let input = include_str!("../input.txt");
    // println!("{input}");
    // println!("{}", day3::solve_part1(input.into()));
    println!("{}", day3::solve_part2(input.into()));
}
