use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum Day8Error {
    #[error("Cannot read contents from file")]
    CannotReadFile(#[from] std::io::Error),
    #[error("Key {0} does not exist in map")]
    FailureToLookUpMapValue(String),
    #[error("Unable to parse line from file")]
    FailureToParseLine,
}

fn read_file_into_buffer(file_path: &str) -> Result<BufReader<File>, Day8Error> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

fn extract_lines_from_file(reader: BufReader<File>) -> Result<(Vec<char>, Vec<String>), Day8Error> {
    let mut lines_iter = reader.lines();

    let first_line: Vec<char> = match lines_iter.next() {
        Some(Ok(line)) => line.chars().collect::<Vec<char>>(),
        _ => return Err(Day8Error::FailureToParseLine),
    };

    let remaining_lines: Result<Vec<String>, Day8Error> = lines_iter
        .map(|line| line.map_err(|_| Day8Error::FailureToParseLine))
        .collect();

    let remaining_lines = remaining_lines?;

    let filtered_lines: Vec<String> = remaining_lines
        .iter()
        .filter(|line| !line.is_empty())
        .cloned()
        .collect();

    Ok((first_line, filtered_lines))
}

fn populate_network_map_from_lines(
    network_lines: Vec<String>,
) -> HashMap<String, (String, String)> {
    let mut network_map: HashMap<String, (String, String)> = HashMap::new();

    for line in network_lines {
        let sep_line: Vec<&str> = line.split('=').collect();

        let node = sep_line[0].trim();

        let directions: Vec<&str> = sep_line[1].split(',').collect();

        let left = directions[0].trim().trim_matches('(');

        let right = directions[1].trim().trim_matches(')');

        network_map.insert(node.to_string(), (left.to_string(), right.to_string()));
    }

    network_map
}

fn calculate_steps_to_traverse_network(
    instructions: Vec<char>,
    network_map: HashMap<String, (String, String)>,
) -> Result<usize, Day8Error> {
    let mut num_of_steps = 0;
    let instructions_len = instructions.len();

    let mut current_node = String::from("AAA");
    let ending_node = String::from("ZZZ");

    while current_node != ending_node {
        match network_map.get(&current_node) {
            None => return Err(Day8Error::FailureToLookUpMapValue(current_node)),
            Some(node_values) => {
                if instructions[num_of_steps % instructions_len] == 'L' {
                    current_node = node_values.0.clone()
                } else {
                    // Instruction defaults to 'R'
                    current_node = node_values.1.clone()
                }

                num_of_steps += 1
            }
        }
    }

    Ok(num_of_steps)
}

fn run() -> Result<(), Day8Error> {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day08/part_01/input.txt";

    let reader = read_file_into_buffer(file_path)?;

    let (first_line, remaining_lines) = extract_lines_from_file(reader)?;

    let network_map = populate_network_map_from_lines(remaining_lines);

    let num_of_steps = calculate_steps_to_traverse_network(first_line, network_map)?;

    println!("Number of steps required = {}", num_of_steps);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error running program: {}", e);
    }
}
