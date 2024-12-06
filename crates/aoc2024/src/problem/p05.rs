use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use sscanf::sscanf;

use aoc2024_core::{lines_iter, parse};

fn part1(rules: &[(i32, i32)], updates: &[Vec<i32>]) {
    let mut table = HashMap::new();
    for (x, y) in rules {
        table.entry(*x).or_insert(HashSet::new()).insert(*y);
    }

    let comp = |a: &i32, b: &i32| table.get(a).unwrap_or(&HashSet::new()).contains(b);

    let sum = updates
        .iter()
        .filter(|update| update.is_sorted_by(comp))
        .map(|update| update[update.len() / 2])
        .sum::<i32>();

    println!("p05 part 1: {}", sum);
}

fn part2(rules: &[(i32, i32)], updates: &mut [Vec<i32>]) {
    let mut table = HashMap::new();
    for (x, y) in rules {
        table.entry(*x).or_insert(HashSet::new()).insert(*y);
    }

    let comp = |a: &i32, b: &i32| table.get(a).unwrap_or(&HashSet::new()).contains(b);

    let sort_fn = |a: &i32, b: &i32| {
        if comp(a, b) {
            return Ordering::Less;
        }
        Ordering::Greater
    };

    let sum = updates
        .iter_mut()
        .filter(|update| !update.is_sorted_by(comp))
        .map(|update| {
            update.sort_by(sort_fn);
            update[update.len() / 2]
        })
        .sum::<i32>();

    println!("p05 part 2: {}", sum);
}

pub fn solve() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/p05.in"));
    let rules: Vec<(i32, i32)> = lines_iter(input)
        .take_while(|line| !line.is_empty())
        .map(|line| sscanf!(line, "{}|{}", i32, i32).unwrap())
        .collect();
    let mut updates: Vec<Vec<i32>> = lines_iter(input)
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split_terminator(',').map(parse::<i32>).collect())
        .collect();

    part1(&rules, &updates);
    part2(&rules, &mut updates);
}
