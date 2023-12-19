use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;
use thiserror::Error;

#[derive(Error, Debug)]
enum Day9Error {
    #[error("Unable to read contents from file")]
    FailedToReadFile(#[from] std::io::Error),
}

#[derive(Clone, Debug)]
struct HistoryValues {
    values: Vec<Vec<i32>>,
}

fn run() -> Result<(), Day9Error> {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day09/part_01/input.txt";

    let reader = read_file_into_buffer(file_path)?;

    let mut sum = 0;

    let mut all_histories: Vec<HistoryValues> = populate_history_vectors(reader)?;

    for history in all_histories.iter_mut() {
        // Loop until a vector of all 0s is created
        while !differences_are_zero(history.values.last().unwrap()) {
            let history_values_length = history.values.len();
            let difference_vector =
                create_differences_vector(history.values[history_values_length - 1].clone());
            history.values.push(difference_vector);
        }

        // Moving from last to first,
        // add the last number of the last vector with the last number of the vector before it until the first vector is reached
        for i in 0..history.values.len() {
            let history_length = history.values.len();
            let current_values = history.values[history_length - (i + 1)].clone();

            if i < (history_length - 1) {
                let previous_values = &mut history.values[history_length - (i + 2)];

                let new_num = current_values.last().unwrap() + previous_values.last().unwrap();

                previous_values.push(new_num);
            } else {
                // Arrived at the finalized first set of numbers
                let final_num = current_values[current_values.len() - 1];

                sum += final_num;
            }
        }
    }

    println!("Sum = {}", sum);

    Ok(())
}

fn differences_are_zero(values: &[i32]) -> bool {
    values.iter().all(|num| *num == 0)
}

fn create_differences_vector(values: Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = vec![];

    for (i, _) in values.iter().enumerate() {
        if i != (values.len() - 1) {
            let difference = values[i + 1] - values[i];
            differences.push(difference);
        }
    }

    differences
}

fn populate_history_vectors(reader: BufReader<File>) -> Result<Vec<HistoryValues>, Day9Error> {
    let mut all_histories: Vec<HistoryValues> = vec![];

    for line in reader.lines() {
        let line = line?;

        let values: Vec<i32> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();

        let history = HistoryValues {
            values: vec![values],
        };

        all_histories.push(history);
    }

    Ok(all_histories)
}

fn read_file_into_buffer(file_path: &str) -> Result<BufReader<File>, Day9Error> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error running program: {}", e);
    }
}
