use aoc2024_core::iter_lines;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
    Invalid,
}

fn check<'a>(mut report: impl Iterator<Item = &'a i32>) -> bool {
    let mut prev = report.next().unwrap();
    let mut it = report.map(|next| {
        let direction = match next - prev {
            1..=3 => Direction::Increasing,
            -3..=-1 => Direction::Decreasing,
            _ => Direction::Invalid,
        };
        prev = next;
        direction
    });
    let direction = it.next().unwrap();
    let result = it.fold(direction, |expected, direction| {
        match expected != direction || direction == Direction::Invalid {
            true => Direction::Invalid,
            false => direction,
        }
    });
    result != Direction::Invalid
}

pub fn solve() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/p02.in"));

    let reports = iter_lines(input)
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let valid = reports
        .iter()
        .map(|report| {
            let diff: Vec<_> = report.windows(2).map(|pair| pair[0] - pair[1]).collect();
            diff.iter().all(|x| (1..=3).contains(x)) || diff.iter().all(|x| (-3..0).contains(x))
        })
        .filter(|&x| x)
        .count();
    println!("p02 part 1: {}", valid);

    let mut sum = 0;
    for report in reports {
        if check(report.iter()) {
            sum += 1;
            continue;
        }

        let mut valid = false;
        for i in 0..report.len() {
            if check(report[0..i].iter().chain(&report[i + 1..])) {
                valid = true;
                break;
            }
        }

        if valid {
            sum += 1;
        }
    }
    println!("p02 part 2: {}", sum);
}
