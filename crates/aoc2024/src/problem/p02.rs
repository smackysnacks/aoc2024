use aoc2024_core::iter_lines;

fn check(report: &[i32]) -> bool {
    let diff: Vec<_> = report.windows(2).map(|pair| pair[0] - pair[1]).collect();
    diff.iter().all(|x| (1..=3).contains(x)) || diff.iter().all(|x| (-3..0).contains(x))
}

fn part1(reports: &[Vec<i32>]) {
    let valid = reports
        .iter()
        .map(|report| check(report))
        .filter(|&x| x)
        .count();
    println!("p02 part 1: {}", valid);
}

fn part2(reports: Vec<Vec<i32>>) {
    let mut sum = 0;
    for mut report in reports {
        if check(&report) {
            sum += 1;
            continue;
        }

        let mut valid = false;
        for i in 0..report.len() {
            let removed = report.remove(i);
            if check(&report) {
                valid = true;
                break;
            }
            report.insert(i, removed);
        }

        if valid {
            sum += 1;
        }
    }
    println!("p02 part 2: {}", sum);
}

pub fn solve() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/p02.in"));

    let reports: Vec<Vec<i32>> = iter_lines(input)
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    part1(&reports);
    part2(reports);
}
