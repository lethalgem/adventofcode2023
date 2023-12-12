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

    let mut galaxy: Vec<Vec<char>> = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for (idx, line) in BufReader::new(handle).lines().enumerate() {
        let line = line.expect("line must exist").chars().collect::<Vec<_>>();
        if line.iter().all(|c| *c == '.') {
            empty_rows.push(idx as i64);
        }
        galaxy.push(line);
    }

    for j in 0..galaxy[0].len() {
        if (0..galaxy.len()).all(|i| galaxy[i][j] == '.') {
            empty_cols.push(j as i64);
        }
    }

    let mut stars: Vec<(i64, i64)> = Vec::new();
    for i in 0..galaxy.len() {
        for j in 0..galaxy[0].len() {
            if galaxy[i][j] == '#' {
                stars.push((i as i64, j as i64));
            }
        }
    }

    // Calculate distances
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            let empty_rows_between = empty_rows.iter().fold(0, |acc, r| {
                acc + i64::from(
                    stars[i].0 < *r && stars[j].0 > *r || stars[i].0 > *r && stars[j].0 < *r,
                )
            });
            let empty_cols_between = empty_cols.iter().fold(0, |acc, c| {
                acc + i64::from(
                    stars[i].1 < *c && stars[j].1 > *c || stars[i].1 > *c && stars[j].1 < *c,
                )
            });
            part_1_result += (stars[i].0 - stars[j].0).abs() - empty_rows_between
                + empty_rows_between * 2
                + (stars[i].1 - stars[j].1).abs()
                - empty_cols_between
                + empty_cols_between * 2;

            // This is the same but with a 1000,000 multiplier
            part_2_result += (stars[i].0 - stars[j].0).abs() - empty_rows_between
                + empty_rows_between * 1_000_000
                + (stars[i].1 - stars[j].1).abs()
                - empty_cols_between
                + empty_cols_between * 1_000_000;
        }
    }

    println!("part 1 answer: {part_1_result}");
    println!("part 2 answer: {part_2_result}");
}

fn main() {
    solution();
}
