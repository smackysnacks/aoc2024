use aoc2024_core::lines_chars;

fn part1(input: &[Vec<char>]) {
    let height = input.len();
    let width = input[0].len();

    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            count += (j >= 3
                && input[i][j] == 'X'
                && input[i][j - 1] == 'M'
                && input[i][j - 2] == 'A'
                && input[i][j - 3] == 'S') as usize;

            count += (i >= 3
                && j >= 3
                && input[i][j] == 'X'
                && input[i - 1][j - 1] == 'M'
                && input[i - 2][j - 2] == 'A'
                && input[i - 3][j - 3] == 'S') as usize;

            count += (i >= 3
                && input[i][j] == 'X'
                && input[i - 1][j] == 'M'
                && input[i - 2][j] == 'A'
                && input[i - 3][j] == 'S') as usize;

            count += (i >= 3
                && width - j > 3
                && input[i][j] == 'X'
                && input[i - 1][j + 1] == 'M'
                && input[i - 2][j + 2] == 'A'
                && input[i - 3][j + 3] == 'S') as usize;

            count += (width - j > 3
                && input[i][j] == 'X'
                && input[i][j + 1] == 'M'
                && input[i][j + 2] == 'A'
                && input[i][j + 3] == 'S') as usize;

            count += (height - i > 3
                && width - j > 3
                && input[i][j] == 'X'
                && input[i + 1][j + 1] == 'M'
                && input[i + 2][j + 2] == 'A'
                && input[i + 3][j + 3] == 'S') as usize;

            count += (height - i > 3
                && input[i][j] == 'X'
                && input[i + 1][j] == 'M'
                && input[i + 2][j] == 'A'
                && input[i + 3][j] == 'S') as usize;

            count += (height - i > 3
                && j >= 3
                && input[i][j] == 'X'
                && input[i + 1][j - 1] == 'M'
                && input[i + 2][j - 2] == 'A'
                && input[i + 3][j - 3] == 'S') as usize;
        }
    }
    println!("p04 part 1: {}", count);
}

fn part2(input: &[Vec<char>]) {
    let height = input.len();
    let width = input[0].len();

    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            if !(input[i][j] == 'A' && i > 0 && j > 0 && height - i > 1 && width - j > 1) {
                continue;
            }

            count += (((input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S')
                || (input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M'))
                && ((input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S')
                    || (input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M')))
                as usize;
        }
    }
    println!("p04 part 2: {}", count);
}

pub fn solve() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/p04.in"));
    let lines = lines_chars(input);

    part1(&lines);
    part2(&lines);
}
