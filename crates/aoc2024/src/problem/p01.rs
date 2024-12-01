use std::collections::HashMap;

use aoc2024_core::iter_lines;

pub fn solve() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/p01.in"));

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in iter_lines(input) {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i32>().unwrap());
        right.push(split.next().unwrap().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += (right[i] - left[i]).abs();
    }
    println!("part 1: {}", sum);

    let mut table = HashMap::new();
    for ele in right {
        *table.entry(ele).or_insert(0) += 1;
    }
    let sum = left
        .into_iter()
        .map(|ele| ele * table.get(&ele).unwrap_or(&0))
        .sum::<i32>();
    println!("part 2: {}", sum);
}
