use std::{fs, io, num};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Part1Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Could not parse to int")]
    ParseIntFailed(#[from] num::ParseIntError),
    #[error("No first number found")]
    NoFirstNum,
    #[error("No last number found")]
    NoLastNum,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Part1Error> {
    let input_data = load_input("src/input.txt".to_string())?;
    let extracted_numbers = extract_all_numbers(input_data)?;
    let sum: i32 = extracted_numbers.iter().sum();
    println!("{}", sum);
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Part1Error> {
    let data = fs::read_to_string(file_path).map_err(Part1Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn extract_all_numbers(input: String) -> Result<Vec<i32>, Part1Error> {
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        let extracted_number = extract_number_from_line(line.to_owned())?;
        numbers.push(extracted_number);
    }
    Ok(numbers)
}

fn extract_number_from_line(line: String) -> Result<i32, Part1Error> {
    let mut nums: Vec<char> = Vec::new();
    for c in line.chars() {
        c.is_ascii_digit().then(|| nums.push(c));
    }
    let first_num = nums.first().ok_or_else(|| Part1Error::NoFirstNum)?;
    let last_num = nums.last().ok_or_else(|| Part1Error::NoLastNum)?;
    let composed_num = format!("{}{}", first_num, last_num)
        .parse::<i32>()
        .map_err(Part1Error::ParseIntFailed)?;
    Ok(composed_num)
}

#[cfg(test)]
mod tests {
    use crate::{extract_all_numbers, extract_number_from_line, load_input};

    #[test]
    fn load_file() {
        let result = load_input("src/example_input.txt".to_string()).unwrap();
        assert_eq!(result, "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
    }

    #[test]
    fn find_numbers() {
        let input_data = load_input("src/example_input.txt".to_string()).unwrap();
        let mut result: Vec<i32> = Vec::new();
        for line in input_data.lines() {
            let extracted_number = extract_number_from_line(line.to_owned()).unwrap();
            result.push(extracted_number);
        }
        assert_eq!(result, vec![12, 38, 15, 77]);
    }

    #[test]
    fn find_sum() {
        let input_data = load_input("src/example_input.txt".to_string()).unwrap();
        let extracted_numbers = extract_all_numbers(input_data).unwrap();
        let result: i32 = extracted_numbers.iter().sum();
        assert_eq!(result, 142);
    }
}
