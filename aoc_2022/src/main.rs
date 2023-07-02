#![allow(unused)]
mod day1;
mod day2;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day2::solve_part1(input));
    println!("{}", day2::solve_part2(input));
}
