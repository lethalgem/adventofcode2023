use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

const STEPS: usize = 64;

fn neighbors(p: (usize, usize), grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let dim = grid.len();
    let mut result = HashSet::new();
    if p.0 > 0 && grid[p.0 - 1][p.1] != '#' {
        result.insert((p.0 - 1, p.1));
    }

    if p.0 < dim - 1 && grid[p.0 + 1][p.1] != '#' {
        result.insert((p.0 + 1, p.1));
    }

    if p.1 > 0 && grid[p.0][p.1 - 1] != '#' {
        result.insert((p.0, p.1 - 1));
    }

    if p.1 < dim - 1 && grid[p.0][p.1 + 1] != '#' {
        result.insert((p.0, p.1 + 1));
    }

    result
}

fn part1() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut grid = Vec::new();
    let mut start = (0, 0);
    for (row, line) in BufReader::new(handle).lines().enumerate() {
        let line = line.unwrap().chars().collect::<Vec<_>>();
        if let Some(col) = line.iter().position(|c| *c == 'S') {
            start = (row, col);
        }
        grid.push(line);
    }

    let _dim = grid.len();

    let mut front = neighbors(start, &grid);
    for _s in 1..STEPS {
        let mut next_front = HashSet::new();
        for f in front {
            next_front.extend(neighbors(f, &grid));
        }
        front = next_front;
    }

    println!("Part 1 answer: {}", front.len());
}

fn main() {
    part1();
}
