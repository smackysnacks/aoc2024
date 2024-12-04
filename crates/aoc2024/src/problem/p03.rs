use aoc2024_core::parse;
use regex::Regex;

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += parse::<usize>(x) * parse::<usize>(y);
    }

    println!("p03 part 1: {}", sum);
}

fn part2(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for (s, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        match s {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => sum += parse::<usize>(x) * parse::<usize>(y) * enabled as usize,
        }
    }

    println!("p03 part 2: {}", sum);
}

pub fn solve() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/p03.in"));

    part1(input);
    part2(input);
}
