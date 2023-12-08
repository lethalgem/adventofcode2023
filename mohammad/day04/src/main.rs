use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

enum Part {
    One,
    Two,
}

fn solution(part: &Part) {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut part_1_result = 0;

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for (idx, line) in BufReader::new(handle).lines().enumerate() {
        let line = line.unwrap();
        let split: Vec<_> = line.split([':', '|']).collect();

        let winning = split[1]
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();

        let cards = split[2]
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();

        let matches: Vec<_> = winning.intersection(&cards).collect();

        map.entry(idx).and_modify(|value| *value += 1).or_insert(1);
        let multiplier = map[&idx];
        for i in (idx + 1)..=(idx + matches.len()) {
            map.entry(i)
                .and_modify(|value| *value += multiplier)
                .or_insert(multiplier);
        }

        if !matches.is_empty() {
            part_1_result += 2usize.pow((matches.len() - 1) as u32)
        };
    }

    match part {
        Part::One => println!("Part 1 answer: {part_1_result}"),
        Part::Two => println!("Part 2 answer: {}", map.values().sum::<usize>()),
    };
}

fn main() {
    solution(&Part::One);
    solution(&Part::Two);
}
