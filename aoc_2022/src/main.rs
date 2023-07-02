#![allow(unused)]

mod day1;

use day1::*;

fn main() {
    let file_content = include_str!("../input.txt");
    println!("{}", solve_part1(file_content));
    println!("{:?}", solve_part2(file_content).iter().fold(0, |acc, x| acc + x));
}
