// The pipes are arranged in a two-dimensional grid of tiles:

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

// Check all four sides (N, S, E, W) for a connecting pipe ->
//      | = (i + 1, j) AND (i - 1, j)
//      - = (i, j + 1) AND (i, j - 1)
//      L = (i, j - 1) AND (i - 1, j)
//      J = (i, j + 1) AND (i - 1, j)
//      7 = (i + 1, j) AND (i, j + 1)
//      F = (i + 1, j) AND (i, j - 1)

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum Day10Error {
    #[error("Unable to read contents from file")]
    FailedToReadFile(#[from] std::io::Error),
    #[error("Failed to locate a starting index")]
    NoStartingIndexFound,
}

fn run() -> Result<(), Day10Error> {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day10/part_01/input.txt";

    let lines = read_file_into_buffer(file_path)?;

    // (17, 83)
    let starting_index = find_starting_index(&lines)?;

    let total_steps = traverse_the_pipes(starting_index, lines);

    println!("total steps = {}", total_steps);

    Ok(())
}

fn find_starting_index(lines: &[String]) -> Result<(usize, usize), Day10Error> {
    for (i, line) in lines.iter().enumerate() {
        if let Some(j) = line.find('S') {
            return Ok((i, j));
        }
    }

    Err(Day10Error::NoStartingIndexFound)
}

fn traverse_the_pipes(starting_index: (usize, usize), pipes: Vec<String>) -> usize {
    let mut index = starting_index;
    let mut next_pipe = "";
    let mut step_counter = 0;
    let mut visited = HashSet::new();
    visited.insert(index);
    let east_boundary = pipes[0].len() - 1;
    let south_boundary = pipes.len() - 1;

    // Travel until the next pipe is the starting pipe
    while next_pipe != "S" {
        let current_pipe = pipes[index.0].get(index.1..index.1 + 1).unwrap();

        // Make sure that I'm not reaching off the board boundaries
        let n_pipe = if index.0 > 0 {
            pipes[index.0 - 1].get(index.1..index.1 + 1).unwrap()
        } else {
            ""
        };
        let e_pipe = if index.1 < east_boundary {
            pipes[index.0].get(index.1 + 1..index.1 + 2).unwrap()
        } else {
            ""
        };
        let s_pipe = if index.0 < south_boundary {
            pipes[index.0 + 1].get(index.1..index.1 + 1).unwrap()
        } else {
            ""
        };
        let w_pipe = if index.1 > 0 {
            pipes[index.0].get(index.1 - 1..index.1).unwrap()
        } else {
            ""
        };

        // Check N
        if north_pipe_can_be_travelled(n_pipe)
            && current_pipe_can_go_north(current_pipe)
            && !visited.contains(&(index.0 - 1, index.1))
        {
            next_pipe = n_pipe;
            index = (index.0 - 1, index.1);
            step_counter += 1;
            visited.insert(index);
        }
        // Check E
        else if east_pipe_can_be_travelled(e_pipe)
            && current_pipe_can_go_east(current_pipe)
            && !visited.contains(&(index.0, index.1 + 1))
        {
            next_pipe = e_pipe;
            index = (index.0, index.1 + 1);
            step_counter += 1;
            visited.insert(index);
        }
        // Check S
        else if south_pipe_can_be_travelled(s_pipe)
            && current_pipe_can_go_south(current_pipe)
            && !visited.contains(&(index.0 + 1, index.1))
        {
            next_pipe = s_pipe;
            index = (index.0 + 1, index.1);
            step_counter += 1;
            visited.insert(index);
        }
        // Check W
        else if west_pipe_can_be_travelled(w_pipe)
            && current_pipe_can_go_west(current_pipe)
            && !visited.contains(&(index.0, index.1 - 1))
        {
            next_pipe = w_pipe;
            index = (index.0, index.1 - 1);
            step_counter += 1;
            visited.insert(index);
        } else {
            println!("Broke out of the loop! Did not find a valid direction to take!");
            break;
        }
    }

    step_counter
}

// Utility functions to make above function a little more concise
fn current_pipe_can_go_north(current_pipe: &str) -> bool {
    current_pipe == "S" || current_pipe == "|" || current_pipe == "L" || current_pipe == "J"
}

fn north_pipe_can_be_travelled(n_pipe: &str) -> bool {
    n_pipe == "|" || n_pipe == "7" || n_pipe == "F"
}
fn current_pipe_can_go_east(current_pipe: &str) -> bool {
    current_pipe == "S" || current_pipe == "-" || current_pipe == "L" || current_pipe == "F"
}

fn east_pipe_can_be_travelled(e_pipe: &str) -> bool {
    e_pipe == "-" || e_pipe == "J" || e_pipe == "7"
}

fn current_pipe_can_go_south(current_pipe: &str) -> bool {
    current_pipe == "S" || current_pipe == "|" || current_pipe == "7" || current_pipe == "F"
}
fn south_pipe_can_be_travelled(s_pipe: &str) -> bool {
    s_pipe == "|" || s_pipe == "L" || s_pipe == "J"
}

fn current_pipe_can_go_west(current_pipe: &str) -> bool {
    current_pipe == "S" || current_pipe == "-" || current_pipe == "J" || current_pipe == "7"
}
fn west_pipe_can_be_travelled(w_pipe: &str) -> bool {
    w_pipe == "-" || w_pipe == "L" || w_pipe == "F"
}

fn read_file_into_buffer(file_path: &str) -> Result<Vec<String>, Day10Error> {
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

// Recursive attempt that I may come back to. Currently experiencing a stack overflow.
// fn traverse_the_pipes(
//     index: (usize, usize),
//     pipes: Vec<String>,
//     step_counter: usize,
//     visited: &mut HashSet<(usize, usize)>,
// ) -> usize {
//     if visited.contains(&index) {
//         return step_counter;
//     }

//     visited.insert(index);

//     let current_pipe = pipes[index.0].get(index.1..index.1 + 1).unwrap();
//     println!("Current index: {:?}", index);

//     // Make sure that I'm not reaching off the board boundaries
//     let n_pipe = if index.0 > 0 {
//         pipes[index.0 - 1].get(index.1..index.1 + 1).unwrap()
//     } else {
//         ""
//     };
//     let e_pipe = if index.1 < 139 {
//         pipes[index.0].get(index.1 + 1..index.1 + 2).unwrap()
//     } else {
//         ""
//     };
//     let s_pipe = pipes[index.0 + 1].get(index.1..index.1 + 1).unwrap();
//     let w_pipe = pipes[index.0].get(index.1 - 1..index.1).unwrap();

//     // Check N
//     if (n_pipe == "|" || n_pipe == "7" || n_pipe == "F")
//         && current_pipe_can_go_north(current_pipe)
//         && !visited.contains(&(index.0 - 1, index.1))
//     {
//         println!("N current step: {}", step_counter);
//         traverse_the_pipes((index.0 - 1, index.1), pipes, step_counter + 1, visited)
//     }
//     // Check E
//     else if east_pipe_can_be_travelled(e_pipe)
//         && current_pipe_can_go_east(current_pipe)
//         && !visited.contains(&(index.0, index.1 + 1))
//     {
//         println!("E current step: {}", step_counter);
//         traverse_the_pipes((index.0, index.1 + 1), pipes, step_counter + 1, visited)
//     }
//     // Check S
//     else if (s_pipe == "|" || s_pipe == "L" || s_pipe == "J")
//         && current_pipe_can_go_south(current_pipe)
//         && !visited.contains(&(index.0 + 1, index.1))
//     {
//         println!("S current step: {}", step_counter);
//         traverse_the_pipes((index.0 + 1, index.1), pipes, step_counter + 1, visited)
//     }
//     // Check W
//     else if (w_pipe == "-" || w_pipe == "L" || w_pipe == "F")
//         && current_pipe_can_go_west(current_pipe)
//         && !visited.contains(&(index.0, index.1 - 1))
//     {
//         println!("W current step: {}", step_counter);
//         traverse_the_pipes((index.0, index.1 - 1), pipes, step_counter + 1, visited)
//     } else {
//         return step_counter;
//     }
// }
