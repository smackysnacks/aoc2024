use aoc2024_core::{lines_iter, Counter};

pub fn solve() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/p01.in"));

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines_iter(input) {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<usize>().unwrap());
        right.push(split.next().unwrap().parse::<usize>().unwrap());
    }
    left.sort();
    right.sort();

    let sum: usize = left.iter().zip(&right).map(|(&a, &b)| a.abs_diff(b)).sum();
    println!("p01 part 1: {}", sum);

    let table = right.counts();
    let sum: usize = left
        .into_iter()
        .map(|ele| ele * table.get(&ele).unwrap_or(&0))
        .sum();
    println!("p01 part 2: {}", sum);
}
