use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn solution() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut lines = BufReader::new(handle).lines();

    // Get Directions
    let directions = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect::<Vec<_>>();

    _ = lines.next(); // Advance to skip the empty line

    // Get the map
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let split: Vec<_> = line
            .split(['=', ','])
            .map(|s| {
                let s = s.trim();
                let s = s.trim_start_matches('(');
                s.trim_end_matches(')')
            })
            .collect();
        map.insert(
            split[0].to_string(),
            (split[1].to_string(), split[2].to_string()),
        );
    }

    let mut steps = 0;
    let mut current = String::from("AAA");
    while current != "ZZZ" {
        for d in &directions {
            match d {
                Direction::Left => {
                    current = map[&current].0.clone();
                    steps += 1;
                }
                Direction::Right => {
                    current = map[&current].1.clone();
                    steps += 1;
                }
            }
        }
    }

    println!("Part 1 answer: {steps}");
}

fn main() {
    solution();
}
