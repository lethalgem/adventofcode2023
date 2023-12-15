use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn part1() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();
    let part_1_result = BufReader::new(handle)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .fold(0, |acc, s| {
            acc + s.chars().fold(0, |acc, c| (17 * (acc + c as usize)) % 256)
        });

    println!("part 1 answer: {part_1_result}");
}

fn hash(chars: &[char]) -> usize {
    chars
        .iter()
        .fold(0, |acc, c| (17 * (acc + *c as usize)) % 256)
}

fn part2() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();
    let mut table: Vec<Vec<(String, usize)>> = Vec::new();
    table.resize(256, Vec::new());
    BufReader::new(handle)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .for_each(|s| {
            let split = s.split(['-', '=']).collect::<Vec<_>>();
            let label = split[0].to_string();
            let hash = hash(&label.chars().collect::<Vec<_>>());
            if split[1].is_empty() {
                // We have an `-`
                table[hash].retain(|(l, _)| l != &label);
            } else {
                // We have an `=` followed by a focal length
                let focal_length = split[1].parse::<usize>().unwrap();
                if let Some(pos) = table[hash].iter().position(|(l, _)| l == &label) {
                    table[hash][pos] = (label, focal_length);
                } else {
                    table[hash].push((label, focal_length));
                }
            }
        });

    let part_2_result = table.iter().enumerate().fold(0, |acc, (r#box, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .fold(0, |acc, (slot, (_, focal_length))| {
                acc + (r#box + 1) * (slot + 1) * focal_length
            })
    });
    println!("part 2 answer: {part_2_result}");
}

fn main() {
    part1();
    part2();
}
