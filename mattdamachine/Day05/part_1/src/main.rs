use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug)]
struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn new() -> Self {
        Almanac { maps: vec![] }
    }
}

#[derive(Debug, Clone)]
struct Map {
    conversion_lists: Vec<ConversionList>,
}

impl Map {
    fn new() -> Self {
        Map {
            conversion_lists: vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct ConversionList {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl ConversionList {
    fn new(destination_start: u64, source_start: u64, range: u64) -> Self {
        ConversionList {
            destination_start,
            source_start,
            range,
        }
    }
}

#[derive(Error, Debug)]
enum Day5Error {
    #[error("Cannot read contents from file")]
    CannotReadFile(#[from] std::io::Error),
    #[error("Cannot parse data into an int")]
    ParsingError(#[from] ParseIntError),
}

fn read_file_into_buffer(file_path: &str) -> std::io::Result<(String, Vec<String>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();
    let first_line = lines_iter.next().unwrap()?;

    let remaining_lines: Vec<String> = lines_iter.map(|line| line.unwrap()).collect();

    println!("First line: {}", first_line);
    println!("remaining lines: {:?}", remaining_lines);

    Ok((first_line, remaining_lines))
}

fn populate_almanac(lines: Vec<String>) -> Result<Almanac, Day5Error> {
    let mut almanac = Almanac::new();
    let mut current_map = Map::new();

    for line in lines {
        if line.contains("map") {
            if !current_map.conversion_lists.is_empty() {
                almanac.maps.push(current_map);
            }
            // Start of a new map
            current_map = Map::new();
        } else if !line.is_empty() {
            // populate the map with its corresponding numbers
            let numbers: Result<Vec<u64>, _> = line
                .split_whitespace()
                .map(|item| item.parse().map_err(Day5Error::ParsingError))
                .collect();
            let numbers = numbers?;
            let conversion_list = ConversionList::new(numbers[0], numbers[1], numbers[2]);
            current_map.conversion_lists.push(conversion_list);
        }
    }

    if !current_map.conversion_lists.is_empty() {
        almanac.maps.push(current_map);
    }

    Ok(almanac)
}

fn find_location_value(seed: u64, maps: Vec<Map>) -> u64 {
    let mut current_value = seed;

    for map in maps {
        for list in map.conversion_lists {
            if current_value >= list.source_start && current_value <= list.source_start + list.range
            {
                current_value = current_value - list.source_start + list.destination_start;
                break;
            }
        }
    }

    current_value
}

fn run() -> Result<(), Day5Error> {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day05/part_1/day05-input.txt";

    let (first_line, remaining_lines) = read_file_into_buffer(file_path)?;

    let seeds: Result<Vec<u64>, _> = first_line
        .replace("seeds: ", "")
        .split(' ')
        .map(|s| s.parse::<u64>().map_err(Day5Error::ParsingError))
        .collect();

    let seeds = seeds?;

    let almanac = populate_almanac(remaining_lines)?;

    let mut location_values: Vec<u64> = seeds
        .into_iter()
        .map(|seed| find_location_value(seed, almanac.maps.clone()))
        .collect();

    location_values.sort();

    println!("Smallest value is: {}", location_values[0]);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}
