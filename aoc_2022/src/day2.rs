use std::{collections::HashMap, io::BufRead};

pub(crate) fn solve_part1(input: impl Into<String>) -> i32 {
    parse_input(input.into())
        .into_iter()
        .fold(0, |acc, (elf, you)| acc + calc_point(elf, you))
}

pub(crate) fn solve_part2(input: impl Into<String>) -> i32 {
    parse_input(input.into())
        .into_iter()
        .fold(0, |acc, (elf, outcome)| acc + calc_point2(elf, outcome))
}

fn parse_input(input: String) -> Vec<(String, String)> {
    input[..]
        .split("\n")
        .map(|s| {
            let mut v = s.split(" ");
            (v.next().unwrap().to_owned(), v.next().unwrap().to_owned())
        })
        .collect()
}

fn calc_point2(elf: String, outcome: String) -> i32 {
    let score_lookup = get_config();
    let elf = decrypt(&elf);
    let outcome = match outcome.as_ref() {
        "X" => "loss",
        "Y" => "draw",
        "Z" => "win",
        _ => "",
    };

    let score = match (elf.as_ref(), outcome) {
        ("rock", "win") => score_lookup[outcome] + score_lookup["paper"],
        ("rock", "loss") => score_lookup[outcome] + score_lookup["scissors"],
        ("rock", "draw") => score_lookup[outcome] + score_lookup["rock"],

        ("paper", "win") => score_lookup[outcome] + score_lookup["scissors"],
        ("paper", "loss") => score_lookup[outcome] + score_lookup["rock"],
        ("paper", "draw") => score_lookup[outcome] + score_lookup["paper"],

        ("scissors", "win") => score_lookup[outcome] + score_lookup["rock"],
        ("scissors", "loss") => score_lookup[outcome] + score_lookup["paper"],
        ("scissors", "draw") => score_lookup[outcome] + score_lookup["scissors"],

        (_, _) => 0,
    };

    score
}

fn calc_point(elf: String, you: String) -> i32 {
    let score_lookup = get_config();
    let elf = decrypt(&elf);
    let you = decrypt(&you);

    let score = match (elf.as_ref(), you.as_ref()) {
        ("rock", "rock") => score_lookup["draw"],
        ("paper", "rock") => score_lookup["loss"],
        ("scissors", "rock") => score_lookup["win"],

        ("paper", "paper") => score_lookup["draw"],
        ("rock", "paper") => score_lookup["win"],
        ("scissors", "paper") => score_lookup["loss"],

        ("scissors", "scissors") => score_lookup["draw"],
        ("paper", "scissors") => score_lookup["win"],
        ("rock", "scissors") => score_lookup["loss"],

        (_, _) => 0,
    };

    score + score_lookup[&you]
}

fn decrypt(v: &str) -> String {
    match v {
        "X" | "A" => "rock".into(),
        "Y" | "B" => "paper".into(),
        "Z" | "C" => "scissors".into(),
        _ => "".into(),
    }
}

fn get_config() -> HashMap<String, i32> {
    HashMap::from([
        ("rock".into(), 1),
        ("paper".into(), 2),
        ("scissors".into(), 3),
        ("win".into(), 6),
        ("draw".into(), 3),
        ("loss".into(), 0),
    ])
}
