use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
enum CheckResult {
    Valid,
    AlmostValid,
    Bad,
}

fn check_mirror_at_col(pattern: &[Vec<char>], col: usize) -> CheckResult {
    let mut smudges = 0;
    for row in pattern {
        let cols = row.len();
        for i in 0..std::cmp::min(cols - col - 1, col + 1) {
            if row[col + i + 1] != row[col - i] {
                if smudges == 1 {
                    return CheckResult::Bad;
                }
                smudges += 1;
            }
        }
    }
    if smudges == 1 {
        CheckResult::AlmostValid
    } else if smudges == 0 {
        CheckResult::Valid
    } else {
        CheckResult::Bad
    }
}

fn check_mirror_at_row(pattern: &Vec<Vec<char>>, row: usize) -> CheckResult {
    let mut smudges = 0;
    for j in 0..pattern[0].len() {
        let rows = pattern.len();
        for i in 0..std::cmp::min(rows - row - 1, row + 1) {
            if pattern[row + i + 1][j] != pattern[row - i][j] {
                if smudges == 1 {
                    return CheckResult::Bad;
                }
                smudges += 1;
            }
        }
    }
    if smudges == 1 {
        CheckResult::AlmostValid
    } else if smudges == 0 {
        CheckResult::Valid
    } else {
        CheckResult::Bad
    }
}

fn solution() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut part_1_result = 0;
    let mut part_2_result = 0;

    let mut all_patterns: Vec<Vec<Vec<char>>> = Vec::new();
    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in BufReader::new(handle).lines() {
        let line = line.expect("line must exist");
        if line.is_empty() {
            all_patterns.push(pattern.clone());
            pattern.clear();
        } else {
            pattern.push(line.chars().collect::<Vec<_>>());
        }
    }

    all_patterns.push(pattern);

    for p in all_patterns {
        for i in 0..p[0].len() - 1 {
            match check_mirror_at_col(&p, i) {
                CheckResult::Valid => part_1_result += i + 1,
                CheckResult::AlmostValid => part_2_result += i + 1,
                CheckResult::Bad => {}
            }
        }
        for i in 0..p.len() - 1 {
            match check_mirror_at_row(&p, i) {
                CheckResult::Valid => part_1_result += (i + 1) * 100,
                CheckResult::AlmostValid => part_2_result += (i + 1) * 100,
                CheckResult::Bad => {}
            }
        }
    }

    println!("part 1 answer: {part_1_result}");
    println!("part 2 answer: {part_2_result}");
}

fn main() {
    solution();
}
