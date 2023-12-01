use regex::Regex;
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
    let regex =
        Regex::new(r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9").unwrap();

    let numbers: Vec<&str> = regex.find_iter(&line).map(|m| m.as_str()).collect();
    let first_num = numbers.first().ok_or_else(|| Part1Error::NoFirstNum)?;
    let last_num = numbers.last().ok_or_else(|| Part1Error::NoLastNum)?;

    let converted_first_digit = convert_string_to_int(first_num.to_string())?;
    let converted_second_digit = convert_string_to_int(last_num.to_string())?;

    let composed_num = format!("{}{}", converted_first_digit, converted_second_digit)
        .parse::<i32>()
        .map_err(Part1Error::ParseIntFailed)?;
    Ok(composed_num)
}

fn convert_string_to_int(string: String) -> Result<i32, Part1Error> {
    match string.parse::<i32>() {
        Ok(int) => Ok(int),
        Err(_) => match string.as_str() {
            "one" => Ok(1),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            _ => Err(Part1Error::ParseIntFromWordFailed(string)),
        },
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{extract_all_numbers, extract_number_from_line, load_input};

    #[test]
    fn regex_line() {
        // TODO: Create manual parser. Regex in rust doesn't support look ahead. So when parsing eighthree it only finds eight, and not three
        let re =
            Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9))")
                .unwrap();
        let hay = "eighthree";
        let numbers: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();
        assert_eq!(numbers, vec!["two", "1", "nine"]);
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
