use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

struct Race {
    time: i32,
    distance: i32,
}

impl Race {
    fn new(time: i32, distance: i32) -> Self {
        Race { time, distance }
    }
}

#[derive(Error, Debug)]
enum Day6Error {
    #[error("Cannot read contents from file")]
    CannotReadFile(#[from] std::io::Error),
    #[error("Time or distance values are 0")]
    AbsenceOfTimes,
    #[error("Number of time values does not match number of distance values")]
    MismatchedLengths,
}

fn read_file_into_buffer(file_path: &str) -> Result<BufReader<File>, Day6Error> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

fn extract_time_and_distance(reader: BufReader<File>) -> Result<(Vec<i32>, Vec<i32>), Day6Error> {
    let mut time_numbers: Vec<i32> = vec![];
    let mut distance_numbers: Vec<i32> = vec![];

    for line in reader.lines() {
        let line = line?;

        if line.contains("Time") {
            let time_str: Vec<&str> = line.split(':').collect();

            time_numbers = time_str[1]
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse num"))
                .collect();
        } else if line.contains("Distance") {
            let distance_str: Vec<&str> = line.split(':').collect();

            distance_numbers = distance_str[1]
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse num"))
                .collect();
        }
    }

    // Lengths of the vectors must be the same and must not be zero
    if time_numbers.is_empty() || distance_numbers.is_empty() {
        Err(Day6Error::AbsenceOfTimes)
    } else if time_numbers.len() != distance_numbers.len() {
        Err(Day6Error::MismatchedLengths)
    } else {
        Ok((time_numbers, distance_numbers))
    }
}

fn calculate_ways_to_win(race: &Race) -> i32 {
    let mut ways = 0;

    for i in 1..race.time {
        let distance_traveled = i * (race.time - i);

        if distance_traveled > race.distance {
            ways += 1;
        }
    }

    ways
}

fn run() -> Result<(), Day6Error> {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day06/part_01/input.txt";

    let reader = read_file_into_buffer(file_path)?;

    let (time_numbers, distance_numbers) = extract_time_and_distance(reader)?;

    let races: Vec<Race> = time_numbers
        .iter()
        .zip(distance_numbers.iter())
        .map(|(&time, &distance)| Race::new(time, distance))
        .collect();

    let mut results: Vec<i32> = vec![];

    for race in races {
        results.push(calculate_ways_to_win(&race))
    }

    let product = results.iter().product::<i32>();

    println!("Product of all the possible ways to win is: {:?}", product);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error finding sum of possible ways to win. Err: {}", err)
    }
}
