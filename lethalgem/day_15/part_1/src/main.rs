use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day15Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int")]
    ParseIntFailed(#[from] ParseIntError),
}

#[derive(Debug)]
struct Step {
    hash_result: u32,
}

impl Step {
    fn new(string: &str) -> Step {
        Step {
            hash_result: Self::calculate_hash(string),
        }
    }

    fn calculate_hash(string: &str) -> u32 {
        let mut current_hash_value = 0;
        for c in string.chars() {
            current_hash_value += c as u32;
            current_hash_value *= 17;
            current_hash_value %= 256;
        }
        current_hash_value
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day15Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt")?;

    println!(
        "finding hashes for steps, time elapsed:{:?}",
        start.elapsed()
    );
    let steps = extract_steps(&input_data);

    println!("finding sum, time elapsed:{:?}", start.elapsed());
    let sum = sum_steps(steps);

    println!("sum: {}, time elapsed: {:?}", sum, start.elapsed());

    Ok(())
}

fn load_input(file_path: &str) -> Result<String, Day15Error> {
    let data = fs::read_to_string(file_path).map_err(Day15Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn sum_steps(steps: Vec<Step>) -> u32 {
    steps.into_iter().map(|step| step.hash_result).sum()
}

fn extract_steps(input: &str) -> Vec<Step> {
    input.split(',').map(Step::new).collect()
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::{extract_steps, load_input, sum_steps, Step};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn test_hashing() {
        let input = "H";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["200"]);

        let input = "HA";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["153"]);

        let input = "HAS";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["172"]);

        let input = "HASH";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["52"]);

        let input = "rn=1";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["30"]);
    }

    #[test]
    fn test_extraction() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = extract_steps(input);
        check(
            &format!("{:?}", result),
            expect!["[Step { hash_result: 30 }, Step { hash_result: 253 }, Step { hash_result: 97 }, Step { hash_result: 47 }, Step { hash_result: 14 }, Step { hash_result: 180 }, Step { hash_result: 9 }, Step { hash_result: 197 }, Step { hash_result: 48 }, Step { hash_result: 214 }, Step { hash_result: 231 }]"],
        );
    }

    #[test]
    fn test_sum() {
        let input = load_input("src/example.txt").unwrap();
        let steps = extract_steps(&input);
        let result = sum_steps(steps);
        check(&format!("{:?}", result), expect!["1320"]);
    }
}
