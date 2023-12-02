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
    let mut numbers_found: Vec<String> = Vec::new();
    let line_vec: Vec<char> = line.chars().collect();
    for (starting_location, char) in line_vec.iter().enumerate() {
        if char.is_ascii_digit() {
            numbers_found.push(char.to_string())
        } else {
            let mut look_at_location = starting_location;

            match char {
                'o' => {
                    look_at_location += 1;

                    if look_at_location < line_vec.len() && line_vec[look_at_location] == 'n' {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'e' {
                            println!("found: one");
                            numbers_found.push("one".to_owned());
                        }
                    }
                }
                't' => {
                    look_at_location += 1;

                    if look_at_location < line_vec.len() && line_vec[look_at_location] == 'h' {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'r' {
                            look_at_location += 1;

                            if look_at_location < line_vec.len()
                                && line_vec[look_at_location] == 'e'
                            {
                                look_at_location += 1;

                                if look_at_location < line_vec.len()
                                    && line_vec[look_at_location] == 'e'
                                {
                                    println!("found: three");
                                    numbers_found.push("three".to_owned());
                                }
                            }
                        }
                    } else if look_at_location < line_vec.len() && line_vec[look_at_location] == 'w'
                    {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'o' {
                            println!("found: two");
                            numbers_found.push("two".to_owned());
                        }
                    }
                }
                'f' => {
                    look_at_location += 1;

                    if look_at_location < line_vec.len() && line_vec[look_at_location] == 'o' {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'u' {
                            look_at_location += 1;

                            if look_at_location < line_vec.len()
                                && line_vec[look_at_location] == 'r'
                            {
                                println!("found: four");
                                numbers_found.push("four".to_owned());
                            }
                        }
                    } else if look_at_location < line_vec.len() && line_vec[look_at_location] == 'i'
                    {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'v' {
                            look_at_location += 1;

                            if look_at_location < line_vec.len()
                                && line_vec[look_at_location] == 'e'
                            {
                                println!("found: five");
                                numbers_found.push("five".to_owned());
                            }
                        }
                    }
                }
                's' => {
                    look_at_location += 1;

                    if look_at_location < line_vec.len() && line_vec[look_at_location] == 'i' {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'x' {
                            println!("found: six");
                            numbers_found.push("six".to_owned());
                        }
                    } else if look_at_location < line_vec.len() && line_vec[look_at_location] == 'e'
                    {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'v' {
                            look_at_location += 1;

                            if look_at_location < line_vec.len()
                                && line_vec[look_at_location] == 'e'
                            {
                                look_at_location += 1;

                                if look_at_location < line_vec.len()
                                    && line_vec[look_at_location] == 'n'
                                {
                                    println!("found: seven");
                                    numbers_found.push("seven".to_owned());
                                }
                            }
                        }
                    }
                }
                'e' => {
                    look_at_location += 1;

                    if look_at_location < line_vec.len() && line_vec[look_at_location] == 'i' {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'g' {
                            look_at_location += 1;

                            if look_at_location < line_vec.len()
                                && line_vec[look_at_location] == 'h'
                            {
                                look_at_location += 1;

                                if look_at_location < line_vec.len()
                                    && line_vec[look_at_location] == 't'
                                {
                                    println!("found: eight");
                                    numbers_found.push("eight".to_owned());
                                }
                            }
                        }
                    }
                }
                'n' => {
                    look_at_location += 1;

                    if look_at_location < line_vec.len() && line_vec[look_at_location] == 'i' {
                        look_at_location += 1;

                        if look_at_location < line_vec.len() && line_vec[look_at_location] == 'n' {
                            look_at_location += 1;

                            if look_at_location < line_vec.len()
                                && line_vec[look_at_location] == 'e'
                            {
                                println!("found: nine");
                                numbers_found.push("nine".to_owned());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let first_num = numbers_found
        .first()
        .ok_or_else(|| Part1Error::NoFirstNum)?;
    let last_num = numbers_found.last().ok_or_else(|| Part1Error::NoLastNum)?;

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
