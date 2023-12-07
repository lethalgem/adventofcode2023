use core::cmp::max;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

enum Part {
    One,
    Two,
}

#[derive(Debug)]
enum Entry {
    Char(char),
    Number(usize),
}

fn solution(part: Part) {
    let path: PathBuf = "src/input_small".into();
    let handle = File::open(path.clone()).unwrap();

    let mut result = 0;

    let mut v: Vec<Vec<Entry>> = Vec::new();
    for line in BufReader::new(handle).lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let mut line = vec![];
        let mut value = String::new();
        let mut c_iter = chars.next();
        while let Some(c) = c_iter {
            if c.is_ascii_digit() {
                value = format!("{value}{c}");
                c_iter = chars.next();
            } else {
                if value != String::new() {
                    for i in 0..value.len() {
                        line.push(Entry::Number(value.parse::<usize>().unwrap()));
                    }
                }
                value = String::new();
                line.push(Entry::Char(c));
                c_iter = chars.next();
            }
        }
        v.push(line);
    }

    let rows = v.len();
    let cols = v[0].len();

    dbg!(&v);

    for i in 0..rows {
        for j in 0..cols {
            if let Entry::Char(c) = v[i][j] {
                if !c.is_ascii_digit() && c != '.' {
                    if let Entry::Number(s) = v[i - 1][j - 1] {
                        result += s;
                    }
                    if let Entry::Number(s) = v[i - 1][j] {
                        result += s;
                    }
                    if let Entry::Number(s) = v[i - 1][j + 1] {
                        result += s;
                    }
                    if let Entry::Number(s) = v[i][j - 1] {
                        result += s;
                    }
                    if let Entry::Number(s) = v[i][j + 1] {
                        result += s;
                    }
                    if let Entry::Number(s) = v[i + 1][j - 1] {
                        result += s;
                    }
                    if let Entry::Number(s) = v[i + 1][j] {
                        result += s;
                    }
                    if let Entry::Number(s) = v[i + 1][j + 1] {
                        result += s;
                    }
                }
            }
        }
    }

    println!(
        "Part {} answer: {result}",
        match part {
            Part::One => 1,
            Part::Two => 2,
        }
    );
}

fn main() {
    solution(Part::One);
    solution(Part::Two);
}
