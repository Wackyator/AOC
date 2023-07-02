use std::{io::BufRead, ops::Index};

pub(crate) fn solve_part1(input: impl Into<String>) -> i32 {
    let input_vec = parse_input(input.into());
    let flat_input = flatten(input_vec);

    *flat_input.iter().max().unwrap()
}

pub(crate) fn solve_part2(input: impl Into<String>) -> Vec<i32> {
    let input_vec = parse_input(input.into());
    let mut flat_input = flatten(input_vec);
    
    flat_input.sort();
    
    flat_input[flat_input.len()-3..].into()
}


fn parse_input(input: String) -> Vec<Vec<i32>> {
    input[..]
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn flatten(ration_vec: Vec<Vec<i32>>) -> Vec<i32> {
    ration_vec
        .iter()
        .map(|v| v.iter().fold(0, |acc, x| acc + x))
        .collect()
}
