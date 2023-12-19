use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
enum Direction {
    R,
    D,
    U,
    L,
}

#[derive(Debug)]
enum Part {
    One,
    Two,
}

fn solution(part: &Part) {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();
    let mut plan = vec![];
    for line in BufReader::new(handle).lines() {
        let line = line.unwrap();
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();

        match part {
            Part::One => {
                let direction = match split[0] {
                    "U" => Direction::U,
                    "D" => Direction::D,
                    "L" => Direction::L,
                    "R" => Direction::R,
                    _ => unreachable!(),
                };
                let distance = split[1].parse::<i64>().unwrap();
                plan.push((direction, distance));
            }
            Part::Two => {
                let direction = match &split[2][7..8] {
                    "0" => Direction::R,
                    "1" => Direction::D,
                    "2" => Direction::L,
                    "3" => Direction::U,
                    _ => unreachable!(),
                };
                let distance = i64::from_str_radix(&split[2][2..=6], 16).unwrap();
                plan.push((direction, distance));
            }
        }
    }

    let mut corners: Vec<(i64, i64)> = Vec::new();
    let mut outer_corners: Vec<(i64, i64)> = Vec::new();
    corners.push((0, 0));
    outer_corners.push((0, 0));
    let mut area = 0;
    for i in 0..plan.len() - 1 {
        let prev = corners.last().unwrap();
        let distance = plan[i].1;
        match &plan[i].0 {
            Direction::R => {
                let current = (prev.0, prev.1 + distance);
                corners.push(current);
                match plan[i + 1].0 {
                    Direction::D => outer_corners.push((current.0, current.1 + 1)),
                    Direction::U => outer_corners.push((current.0, current.1)),
                    _ => unreachable!(),
                }
            }
            Direction::L => {
                let current = (prev.0, prev.1 - distance);
                corners.push(current);
                match plan[i + 1].0 {
                    Direction::U => outer_corners.push((current.0 + 1, current.1)),
                    Direction::D => outer_corners.push((current.0 + 1, current.1 + 1)),
                    _ => unreachable!(),
                }
            }
            Direction::U => {
                let current = (prev.0 - distance, prev.1);
                corners.push(current);
                match plan[i + 1].0 {
                    Direction::R => outer_corners.push((current.0, current.1)),
                    Direction::L => outer_corners.push((current.0 + 1, current.1)),
                    _ => unreachable!(),
                }
            }
            Direction::D => {
                let current = (prev.0 + distance, prev.1);
                corners.push(current);
                match plan[i + 1].0 {
                    Direction::L => outer_corners.push((current.0 + 1, current.1 + 1)),
                    Direction::R => outer_corners.push((current.0, current.1 + 1)),
                    _ => unreachable!(),
                }
            }
        };
    }

    for i in 0..outer_corners.len() - 1 {
        area += outer_corners[i].0 * outer_corners[i + 1].1
            - outer_corners[i + 1].0 * outer_corners[i].1;
    }

    match part {
        Part::One => println!("Part 1 answer: {}", area.abs() / 2),
        Part::Two => println!("Part 2 answer: {}", area.abs() / 2),
    };
}

fn main() {
    solution(&Part::One);
    solution(&Part::Two);
}
