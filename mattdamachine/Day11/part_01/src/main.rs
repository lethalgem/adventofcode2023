use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;
use thiserror::Error;

#[derive(Error, Debug)]
enum Day11Error {
    #[error("Unable to read contents from file")]
    FailedToReadFile(#[from] std::io::Error),
}

fn run() -> Result<(), Day11Error> {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day11/part_01/input.txt";

    let lines = read_file_into_buffer(file_path)?;

    let expanded_universe = expand_universe(lines);

    let galaxy_indices: Vec<(usize, usize)> = locate_galaxy_indices(expanded_universe);

    let sum = calculate_distances(galaxy_indices);

    println!("Sum: {}", sum);

    Ok(())
}

fn calculate_distances(indices: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    for (i, location) in indices.iter().enumerate() {
        let remaining_indices = indices.iter().skip(i + 1);

        for next_location in remaining_indices {
            let y_diff = next_location.1.abs_diff(location.1);
            let x_diff = next_location.0.abs_diff(location.0);
            sum += y_diff + x_diff;
        }
    }

    sum
}

fn locate_galaxy_indices(universe: Vec<String>) -> Vec<(usize, usize)> {
    // Keep track of where the galaxies are (i, j)
    let mut galaxy_indices: Vec<(usize, usize)> = vec![];

    for (i, line) in universe.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                galaxy_indices.push((i, j))
            }
        }
    }

    galaxy_indices
}

fn expand_universe(mut lines: Vec<String>) -> Vec<String> {
    // Deal with horizintal rows
    let mut empty_rows = vec![];
    let line_length = lines[0].len();
    for (i, line) in lines.clone().iter().enumerate() {
        if !line.contains('#') {
            // Empty row
            empty_rows.push(i);
        }
    }

    // Add in extra rows
    // Because elements are shifted over whenever a new row is added, I need to keep track of the offset that I'm creating
    // and add it to the index of each empty row
    for (i, row) in empty_rows.iter().enumerate() {
        lines.insert(*row + i, ".".repeat(line_length));
    }

    // Deal with vertical columns
    let mut is_empty_column = true;
    let mut empty_columns = vec![];

    for (i, char) in lines[0].chars().enumerate() {
        if char == '.' {
            // Check the remaining lines at this index
            for line in lines.iter() {
                if line.chars().nth(i).unwrap() == '#' {
                    is_empty_column = false;
                    break;
                }
            }
            if is_empty_column {
                empty_columns.push(i);
            }
        }
        is_empty_column = true;
    }

    // Add in extra columns with offset
    for (i, column) in empty_columns.iter().enumerate() {
        for line in lines.iter_mut() {
            line.insert(*column + i, '.');
        }
    }

    lines
}

fn read_file_into_buffer(file_path: &str) -> Result<Vec<String>, Day11Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error running program: {}", e);
    }
}
