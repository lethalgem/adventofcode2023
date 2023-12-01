// Steps:
// Read the lines of the text input
// Grab the first number and the last number that appear in each line.
// Concat these two values together (be careful not to sum them!!)
// Return the sum of the values obtained from each line in the text input

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_into_buffer(file_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

fn sum_lines(reader: BufReader<File>) -> io::Result<u32> {
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;

        let only_numerical_chars: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

        let length = only_numerical_chars.len();

        if length == 1 {
            let num = only_numerical_chars[0]
                .to_string()
                .repeat(2)
                .parse::<u32>()
                .unwrap();
            sum += num
        } else {
            let first_num = only_numerical_chars[0].to_string();
            let last_num = only_numerical_chars[length - 1].to_string();
            let total = (first_num + &last_num).parse::<u32>().unwrap();
            sum += total
        }
    }

    Ok(sum)
}

fn main() {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day01/Part1/day01-input.txt";

    match read_file_into_buffer(file_path) {
        Ok(reader) => match sum_lines(reader) {
            Ok(sum) => println!("The sum of the input file is {}", sum),
            Err(err) => println!("Error summing up the lines {}", err),
        },
        Err(err) => println!("Error reading the contents of {} {}", file_path, err),
    }
}
