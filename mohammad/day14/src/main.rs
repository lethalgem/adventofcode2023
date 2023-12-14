use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn swap(v: &mut [Vec<char>], a: (usize, usize), b: (usize, usize)) {
    let tmp = v[a.0][a.1];
    v[a.0][a.1] = v[b.0][b.1];
    v[b.0][b.1] = tmp;
}

fn tilt_north(grid: &mut Vec<Vec<char>>, col: usize) {
    let mut square_loc = 0;
    let mut idx = 0;
    let dim = grid.len();
    while idx < dim {
        match grid[idx][col] {
            'O' => {
                if idx > 0 {
                    for i in (square_loc..idx).rev() {
                        match grid[i][col] {
                            'O' | '#' => {
                                swap(grid, (idx, col), (i + 1, col));
                                idx = i + 1;
                                break;
                            }
                            '.' if i == 0 => {
                                swap(grid, (idx, col), (0, col));
                                idx = i + 1;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            '#' => square_loc = idx,
            _ => {}
        }
        idx += 1;
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>, col: usize) {
    let mut ridx = 0;
    let dim = grid.len();
    let mut square_loc = dim - 1;
    while ridx < dim {
        let idx = dim - 1 - ridx;
        match grid[idx][col] {
            'O' => {
                if idx < dim {
                    for i in idx + 1..=square_loc {
                        match grid[i][col] {
                            'O' | '#' => {
                                swap(grid, (idx, col), (i - 1, col));
                                break;
                            }
                            '.' if i == dim - 1 => {
                                swap(grid, (idx, col), (dim - 1, col));
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            '#' => square_loc = idx,
            _ => {}
        }
        ridx += 1;
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>, row: usize) {
    let mut square_loc = 0;
    let mut idx = 0;
    let dim = grid.len();
    while idx < dim {
        match grid[row][idx] {
            'O' => {
                if idx > 0 {
                    for j in (square_loc..idx).rev() {
                        match grid[row][j] {
                            'O' | '#' => {
                                swap(grid, (row, idx), (row, j + 1));
                                break;
                            }
                            '.' if j == 0 => {
                                swap(grid, (row, idx), (row, 0));
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            '#' => square_loc = idx,
            _ => {}
        }
        idx += 1;
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>, row: usize) {
    let mut ridx = 0;
    let dim = grid.len();
    let mut square_loc = dim - 1;
    while ridx < dim {
        let idx = dim - 1 - ridx;
        match grid[row][idx] {
            'O' => {
                if idx < dim {
                    for j in idx + 1..=square_loc {
                        match grid[row][j] {
                            'O' | '#' => {
                                swap(grid, (row, idx), (row, j - 1));
                                break;
                            }
                            '.' if j == dim - 1 => {
                                swap(grid, (row, idx), (row, dim - 1));
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            '#' => square_loc = idx,
            _ => {}
        }
        ridx += 1;
    }
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    let dim = grid.len();
    for i in 0..dim {
        tilt_north(grid, i);
    }
    for i in 0..dim {
        tilt_west(grid, i);
    }
    for i in 0..dim {
        tilt_south(grid, i);
    }
    for i in 0..dim {
        tilt_east(grid, i);
    }
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let dim = grid.len();
    for i in 0..dim {
        for idx in 0..dim {
            if grid[idx][i] == 'O' {
                result += dim - idx;
            }
        }
    }
    result
}

fn solution() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut original_grid = Vec::new();
    for line in BufReader::new(handle).lines() {
        original_grid.push(line.expect("line must exist").chars().collect::<Vec<_>>());
    }

    // Part 1:
    let mut grid = original_grid.clone();
    for i in 0..grid.len() {
        tilt_north(&mut grid, i);
    }
    println!("part 1 answer: {}", calculate_load(&grid));

    // Part 2:
    let mut grid = original_grid.clone();
    let mut hashes = Vec::new(); // to help look for patterns
    let mut repeat: (usize, usize) = (0, 0);
    for j in 0..1_000_000_000 {
        let mut s = DefaultHasher::new();
        grid.hash(&mut s);
        let grid_hash = s.finish();
        if let Some(start) = hashes.iter().position(|h| *h == grid_hash) {
            repeat.0 = start; // Start of pattern
            repeat.1 = j - start; // Length of pattern
            break;
        }
        hashes.push(grid_hash);
        cycle(&mut grid);
    }

    grid = original_grid;
    for _j in 0..repeat.0 {
        cycle(&mut grid);
    }

    for _j in 0..(1_000_000_000 - repeat.0) % repeat.1 {
        cycle(&mut grid);
    }

    println!("part 2 answer: {}", calculate_load(&grid));
}

fn main() {
    solution();
}
