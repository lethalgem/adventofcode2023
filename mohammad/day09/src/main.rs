use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn solution() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut part_1_result = 0;
    let mut part_2_result = 0;
    for line in BufReader::new(handle).lines() {
        let mut input: Vec<i64> = line
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().expect("only expecting i64 integers"))
            .collect();

        let mut lasts = Vec::new();
        let mut firsts = Vec::new();

        while input.iter().any(|e| *e != 0) {
            firsts.push(input[0]);
            for i in 0..input.len() - 1 {
                input[i] = input[i + 1] - input[i];
            }
            lasts.push(input.pop().unwrap());
        }
        part_1_result += lasts.iter().sum::<i64>();

        let n = firsts.len();
        for i in 1..n {
            firsts[n - i - 1] -= firsts[n - i];
        }
        part_2_result += firsts[0];
    }

    println!("part 1 answer: {part_1_result}");
    println!("part 2 answer: {part_2_result}");
}

fn main() {
    solution();
}
