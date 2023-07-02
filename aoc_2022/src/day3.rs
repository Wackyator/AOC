use std::{collections::HashSet, hash::Hash};

pub(crate) fn solve_part1(input: String) -> i32 {
    let mut v = vec![];
    input
        .split("\n")
        .map(|s| {
            let (comp1, comp2) = s.split_at(s.len() / 2);
            for ch in comp1.chars() {
                if comp2.contains(ch) {
                    v.push(find_priority(ch));
                    return;
                }
            }
        })
        .collect::<Vec<()>>();

    v.iter().fold(0, |acc, x| acc + x)
}

pub(crate) fn solve_part2(input: String) -> i32 {
    let mut sum: i32 = 0;
    let v: Vec<_> = input.split("\n").collect();
    for i in (0..v.len() - 1).step_by(3) {
        let sack1: HashSet<char> = HashSet::from_iter(v[i].chars());
        let sack2: HashSet<char> = HashSet::from_iter(v[i + 1].chars());
        let sack3: HashSet<char> = HashSet::from_iter(v[i + 2].chars());

        // this is the worst piece of garbage that could exist but it works
        sack1
            .iter()
            .filter(|&k| sack2.contains(k))
            .filter(|&k| sack3.contains(&k))
            .map(|&ch| sum += find_priority(ch))
            .collect::<Vec<_>>();
    }
    sum
}

fn find_priority(ch: char) -> i32 {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for (i, v) in chars.chars().enumerate() {
        if ch == v {
            return (i + 1) as i32;
        }
    }
    0
}
