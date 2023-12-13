use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn is_valid(candidate: &[char], amounts: &Vec<usize>) -> bool {
    &candidate
        .iter()
        .collect::<String>()
        .split('.')
        .filter_map(|s| if s.is_empty() { None } else { Some(s.len()) })
        .collect::<Vec<_>>()
        == amounts
}

fn compute_arrangements(record: &[char], amounts: &Vec<usize>) -> i64 {
    let mut unknowns = Vec::new();
    let mut damaged_count = 0;
    for (idx, c) in record.iter().enumerate() {
        if *c == '?' {
            unknowns.push(idx);
        } else if *c == '#' {
            damaged_count += 1;
        }
    }

    let amounts_sum = amounts.iter().sum::<usize>();
    if amounts_sum < damaged_count {
        return 0;
    }

    let combinations = unknowns
        .iter()
        .combinations(amounts_sum - damaged_count)
        .collect::<Vec<_>>();

    let mut arrangements: i64 = 0;
    for c in combinations {
        let mut candidate = record.to_owned();
        for i in 0..c.len() {
            candidate[*c[i]] = '#';
        }
        for r in &mut candidate {
            if *r == '?' {
                *r = '.';
            }
        }
        if is_valid(&candidate, amounts) {
            arrangements += 1;
        }
    }
    arrangements
}

fn solution() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut part_1_result = 0;

    for line in BufReader::new(handle).lines() {
        let line = line.unwrap();
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let record = split[0].chars().collect::<Vec<_>>();
        let amounts = split[1]
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let a = compute_arrangements(&record, &amounts);
        part_1_result += a;
    }

    println!("part 1 answer: {part_1_result}");
}

fn main() {
    solution();
}
