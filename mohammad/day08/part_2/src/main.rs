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

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

    let starting: Vec<String> = map
        .keys()
        .filter(|k| k.get(2..=2).unwrap() == "A")
        .cloned()
        .collect();

    let mut all_steps = Vec::new();
    for mut current in starting {
        let mut steps = 0;
        while current.get(2..=2).unwrap() != "Z" {
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
        all_steps.push(steps);
    }

    println!("Part 2 answer: {}", lcm(&all_steps));
}

fn main() {
    solution();
}
