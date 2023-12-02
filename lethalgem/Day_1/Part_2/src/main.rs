use std::{fs, io, num};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Part1Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Could not parse to int")]
    ParseIntFailed(#[from] num::ParseIntError),
    #[error("Could not parse to int from word: {0}")]
    ParseIntFromWordFailed(String),
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
    let number_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut numbers_found = Vec::new();
    let line_vec: Vec<char> = line.chars().collect();

    // Iterate over each character in the string
    for (index, char) in line_vec.iter().enumerate() {
        // check for digit
        if char.is_ascii_digit() {
            numbers_found.push(char.to_string());
        }

        // Check if any number word starts at this index
        for &num_word in &number_words {
            if line[index..].starts_with(num_word) {
                numbers_found.push(num_word.to_string());
                break; // Break to avoid matching smaller numbers within larger ones
            }
        }
    }

    let first_num = convert_string_to_number(numbers_found.first().ok_or(Part1Error::NoFirstNum)?);
    let last_num = convert_string_to_number(numbers_found.last().ok_or(Part1Error::NoLastNum)?);

    let composed_num = format!("{}{}", first_num, last_num)
        .parse::<i32>()
        .map_err(Part1Error::ParseIntFailed)?;

    Ok(composed_num)
}

fn convert_string_to_number(s: &String) -> String {
    match s.as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{extract_all_numbers, extract_number_from_line, load_input};

    #[test]
    fn manual_parsing_one() {
        let hay = "1one";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 11)
    }

    #[test]
    fn manual_parsing_two() {
        let hay = "2two";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 22)
    }

    #[test]
    fn manual_parsing_three() {
        let hay = "3three";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 33)
    }

    #[test]
    fn manual_parsing_four() {
        let hay = "4four";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 44)
    }

    #[test]
    fn manual_parsing_five() {
        let hay = "5five";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 55)
    }

    #[test]
    fn manual_parsing_six() {
        let hay = "6six";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 66)
    }

    #[test]
    fn manual_parsing_seven() {
        let hay = "7seven";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 77)
    }

    #[test]
    fn manual_parsing_eight() {
        let hay = "8eight";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 88)
    }

    #[test]
    fn manual_parsing_nine() {
        let hay = "9nine";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 99)
    }

    #[test]
    fn manual_parsing_all_numbers() {
        let hay = "123456789onetwothreefourfivesixseveneightnine";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 19)
    }

    #[test]
    fn manual_parsing_overlap() {
        let hay = "eighthree";
        let result = extract_number_from_line(hay.to_owned()).unwrap();
        assert_eq!(result, 83)
    }

    #[test]
    fn extract_correct_number_from_line() {
        let input = "one1two";
        let result = extract_number_from_line(input.to_owned()).unwrap();
        assert_eq!(result, 12)
    }

    #[test]
    fn load_file() {
        let result = load_input("src/example_input_part_2.txt".to_string()).unwrap();
        assert_eq!(result, "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
    }

    #[test]
    fn find_numbers() {
        let input_data = load_input("src/example_input_part_2.txt".to_string()).unwrap();
        let mut result: Vec<i32> = Vec::new();
        for line in input_data.lines() {
            let extracted_number = extract_number_from_line(line.to_owned()).unwrap();
            result.push(extracted_number);
        }
        assert_eq!(result, vec![29, 83, 13, 24, 42, 14, 76]);
    }

    #[test]
    fn find_sum() {
        let input_data = load_input("src/example_input_part_2.txt".to_string()).unwrap();
        let extracted_numbers = extract_all_numbers(input_data).unwrap();
        let result: i32 = extracted_numbers.iter().sum();
        assert_eq!(result, 281);
    }
}
