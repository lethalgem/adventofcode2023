use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_into_buffer(file_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn find_total_points(reader: BufReader<File>) -> io::Result<i32> {
    let mut sum = 0;
    let base: i32 = 2;

    for line in reader.lines() {
        let mut winning_num_count = 0;

        let line = line?;

        let (winning_numbers, my_numbers) = extract_numbers(line);

        for num in my_numbers {
            if winning_numbers.contains(&num) {
                winning_num_count += 1;
            }
        }

        if winning_num_count > 0 {
            sum += base.pow(winning_num_count - 1)
        }
    }

    Ok(sum)
}

fn extract_numbers(line: String) -> (Vec<i32>, Vec<i32>) {
    let parts: Vec<&str> = line.split(':').collect();

    let numbers: Vec<&str> = parts[1].split('|').collect();

    let winning_numbers: Vec<i32> = numbers[0]
        .split_whitespace()
        .filter_map(|num_str| num_str.parse::<i32>().ok())
        .collect();

    let my_numbers: Vec<i32> = numbers[1]
        .split_whitespace()
        .filter_map(|num_str| num_str.parse::<i32>().ok())
        .collect();

    (winning_numbers, my_numbers)
}

fn main() {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day04/part_1/day04-input.txt";

    match read_file_into_buffer(file_path) {
        Ok(reader) => match find_total_points(reader) {
            Ok(sum) => println!("Sum is: {}", sum),
            Err(err) => println!("Error finding sum from file: {}", err),
        },
        Err(err) => println!("Error reading file: {}", err),
    }
}
