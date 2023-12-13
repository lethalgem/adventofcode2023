use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day8Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int")]
    ParseIntFailed(#[from] ParseIntError),
    #[error("Failed to find last value in sequence {0:?}")]
    NoLastValueInSequence(Vec<i32>),
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day8Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt".to_string())?;

    println!("calculating sum, time elapsed:{:?}", start.elapsed());
    let sum = extrapolate_all_values(input_data.to_owned())?;

    println!("sum: {}, time elapsed: {:?}", sum, start.elapsed());

    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day8Error> {
    let data = fs::read_to_string(file_path).map_err(Day8Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn extrapolate_all_values(input: String) -> Result<i32, Day8Error> {
    let mut sum = 0;

    for line in input.lines() {
        let sequence: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        sum += predict_first_reading(sequence)?;
    }

    Ok(sum)
}

fn predict_first_reading(sequence: Vec<i32>) -> Result<i32, Day8Error> {
    let mut found_zero_sequence = false;
    let mut first_steps: Vec<i32> = vec![*sequence
        .first()
        .ok_or_else(|| Day8Error::NoLastValueInSequence(sequence.clone()))?];
    let mut next_sequence: Vec<i32> = sequence;

    while !found_zero_sequence {
        let sequence_evaluation = find_next_sequence(&next_sequence);

        first_steps.push(
            *sequence_evaluation
                .0
                .first()
                .ok_or_else(|| Day8Error::NoLastValueInSequence(sequence_evaluation.0.clone()))?,
        );
        found_zero_sequence = sequence_evaluation.1;

        next_sequence = sequence_evaluation.0;
    }

    first_steps.reverse();
    let mut predicted_first_step = *first_steps
        .first()
        .ok_or_else(|| Day8Error::NoLastValueInSequence(first_steps.clone()))?;
    for (i, _) in first_steps.iter().enumerate() {
        if i + 1 == first_steps.len() {
            break;
        };
        let new_predicted_step = first_steps[i + 1] - predicted_first_step;
        predicted_first_step = new_predicted_step
    }

    Ok(predicted_first_step)
}

// return sequence of steps, and if sequence was all 0's
fn find_next_sequence(sequence: &[i32]) -> (Vec<i32>, bool) {
    let mut is_zero_sequence = true;
    let mut next_sequence: Vec<i32> = Vec::new();
    for (i, _) in sequence.iter().enumerate() {
        if i > 0 {
            let step = sequence[i] - sequence[i - 1];
            next_sequence.push(step);

            if step != 0 {
                is_zero_sequence = false;
            }
        }
    }

    (next_sequence, is_zero_sequence)
}

#[cfg(test)]
mod tests {
    use crate::{extrapolate_all_values, find_next_sequence, load_input, predict_first_reading};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn find_correct_steps() {
        let input = [0, 3, 6, 9, 12, 15];
        let result = find_next_sequence(input.as_ref());
        check(
            &format!("{:?}", result),
            expect_test::expect!["([3, 3, 3, 3, 3], false)"],
        );

        let input = [3, 3, 3, 3, 3];
        let result = find_next_sequence(input.as_ref());
        check(
            &format!("{:?}", result),
            expect_test::expect!["([0, 0, 0, 0], true)"],
        );
    }

    #[test]
    fn find_correct_next_reading() {
        let input = [0, 3, 6, 9, 12, 15];
        let result = predict_first_reading(input.to_vec()).unwrap();
        check(&format!("{:?}", result), expect_test::expect!["-3"]);

        let input = [1, 3, 6, 10, 15, 21];
        let result = predict_first_reading(input.to_vec()).unwrap();
        check(&format!("{:?}", result), expect_test::expect!["0"]);

        let input = [10, 13, 16, 21, 30, 45];
        let result = predict_first_reading(input.to_vec()).unwrap();
        check(&format!("{:?}", result), expect_test::expect!["5"]);
    }

    #[test]
    fn find_correct_sum() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let result = extrapolate_all_values(input).unwrap();
        check(&format!("{:?}", result), expect_test::expect!["2"]);
    }
}
