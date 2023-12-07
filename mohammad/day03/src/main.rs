use std::collections::HashMap;
use std::{
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

    let mut v: Vec<Vec<char>> = Vec::new();
    for line in BufReader::new(handle).lines() {
        let line = line.unwrap();
        v.push(line.chars().collect());
    }

    let (rows, cols) = (v.len(), v[0].len());

    let is_special = |c: char| !c.is_ascii_digit() && c != '.';
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    let get_star = |i: usize, j: usize, gears: &mut HashMap<(usize, usize), Vec<u64>>| {
        if gears.get(&(i, j)).is_none() {
            gears.insert((i, j), Vec::new());
        }
        Some((i, j))
    };

    for i in 0..rows {
        let mut is_part_number = false;
        let mut value = String::new();
        let mut star = None;
        for j in 0..cols {
            if v[i][j].is_ascii_digit() {
                if i > 0 && j > 0 && v[i - 1][j - 1] == '*' {
                    star = get_star(i - 1, j - 1, &mut gears);
                }
                if i > 0 && v[i - 1][j] == '*' {
                    star = get_star(i - 1, j, &mut gears);
                }
                if i > 0 && j < cols - 1 && v[i - 1][j + 1] == '*' {
                    star = get_star(i - 1, j + 1, &mut gears);
                }
                if j > 0 && v[i][j - 1] == '*' {
                    star = get_star(i, j - 1, &mut gears);
                }
                if j < cols - 1 && v[i][j + 1] == '*' {
                    star = get_star(i, j + 1, &mut gears);
                }
                if i < rows - 1 && j > 0 && v[i + 1][j - 1] == '*' {
                    star = get_star(i + 1, j - 1, &mut gears);
                }
                if i < rows - 1 && v[i + 1][j] == '*' {
                    star = get_star(i + 1, j, &mut gears);
                }
                if i < rows - 1 && j < cols - 1 && v[i + 1][j + 1] == '*' {
                    star = get_star(i + 1, j + 1, &mut gears);
                }

                if i > 0 && j > 0 && is_special(v[i - 1][j - 1])
                    || i > 0 && is_special(v[i - 1][j])
                    || i > 0 && j < cols - 1 && is_special(v[i - 1][j + 1])
                    || j > 0 && is_special(v[i][j - 1])
                    || j < cols - 1 && is_special(v[i][j + 1])
                    || i < rows - 1 && j > 0 && is_special(v[i + 1][j - 1])
                    || i < rows - 1 && is_special(v[i + 1][j])
                    || i < rows - 1 && j < cols - 1 && is_special(v[i + 1][j + 1])
                {
                    is_part_number = true;
                }

                value = format!("{value}{}", v[i][j]);
            }

            if j == cols - 1 || !v[i][j].is_ascii_digit() {
                if is_part_number {
                    let value_to_int = value.parse::<u64>().expect("expect an integer here");
                    part_1_result += value_to_int;
                    if let Some(star) = star {
                        let v = gears.get_mut(&star);
                        v.expect("already inserted").push(value_to_int);
                    }
                }

                value = String::new();
                is_part_number = false;
                star = None;
            }
        }
    }

    let part_2_result = gears.iter().fold(0, |acc, (_, v)| {
        acc + if v.len() == 2 { v[0] * v[1] } else { 0 }
    });

    match part {
        Part::One => println!("Part 1 answer: {part_1_result}"),
        Part::Two => println!("Part 2 answer: {part_2_result}"),
    };
}

fn main() {
    solution(&Part::One);
    solution(&Part::Two);
}
