// Determine which games would have been possible if the bag had been loaded
// with only 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those games?
// Example input: Game 1: 9 red, 5 blue, 6 green; 6 red, 13 blue; 2 blue, 7 green, 5 red

// Would be nice to add error handling later.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_into_buffer(file_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn find_possible_id_sums(
    reader: BufReader<File>,
    color_limits: HashMap<&str, u32>,
) -> io::Result<u32> {
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;

        let (game_id, all_rounds) = extract_game_id(line);

        let is_game_possible = all_rounds.split(';').all(|round| {
            round.split(',').all(|handful| {
                let (balls, color_grabbed) = parse_handful(handful);
                !is_color_limit_exceeded(&color_limits, color_grabbed, balls)
            })
        });

        if is_game_possible {
            sum += game_id;
        }
    }

    Ok(sum)
}

fn is_color_limit_exceeded(color_limits: &HashMap<&str, u32>, color: &str, balls: u32) -> bool {
    if let Some(&limit) = color_limits.get(color) {
        balls > limit
    } else {
        // Color does not exist in bag
        false
    }
}

fn parse_handful(handful: &str) -> (u32, &str) {
    let parts: Vec<&str> = handful.trim().split(' ').collect();
    let number_of_balls = parts[0].parse::<u32>().unwrap();
    let color_grabbed = parts[1];

    (number_of_balls, color_grabbed)
}

fn extract_game_id(line: String) -> (u32, String) {
    // Extract the game id and return the remaining string with the rounds
    let sep_line: Vec<&str> = line.split(':').collect();

    let remaining_line = sep_line[1].to_string();

    (
        sep_line[0].replace("Game ", "").parse::<u32>().unwrap(),
        remaining_line,
    )
}

fn main() {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day02/Part1/day02-input.txt";

    let color_limits: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    match read_file_into_buffer(file_path) {
        Ok(reader) => match find_possible_id_sums(reader, color_limits) {
            Ok(sum) => println!("Sum: {}", sum),
            Err(err) => println!("Error calculating sum from file {}", err),
        },
        Err(err) => println!("Error reading file into memory: {}", err),
    }
}
