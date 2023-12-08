use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day6Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse time: line")]
    NoTimeLine,
    #[error("Failed to parse distance: line")]
    NoDistanceLine,
    #[error("Did not find equal amounts of times and distancse")]
    UnequalTimesAndDistances,
}

#[derive(Debug, Clone)]
struct Race {
    total_time: i32,
    distance_to_beat: i32,
    ways_to_win: i32,
}

impl Race {
    fn new(total_time: i32, distance_to_beat: i32) -> Race {
        Race {
            total_time,
            distance_to_beat,
            ways_to_win: Self::calculate_ways_to_win(total_time, distance_to_beat),
        }
    }

    fn calculate_ways_to_win(total_time: i32, distance_to_beat: i32) -> i32 {
        let mut ways_to_win = 0;

        for button_held_time in 0..=total_time {
            let speed_per_time_step = button_held_time;
            let time_steps_left_to_move = total_time - button_held_time;
            let distance_traveled = speed_per_time_step * time_steps_left_to_move;
            if distance_traveled > distance_to_beat {
                ways_to_win += 1;
            }
        }

        ways_to_win
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day6Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt".to_string())?;

    println!("parsing races, time elapsed: {:?}", start.elapsed());
    let races = extract_races(input_data.to_owned())?;

    println!("calculating margin of error: {:?}", start.elapsed());
    let margin_of_error = calculate_margin_of_error(races);

    println!(
        "margin of error: {}, time elapsed: {:?}",
        margin_of_error,
        start.elapsed()
    );

    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day6Error> {
    let data = fs::read_to_string(file_path).map_err(Day6Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn calculate_margin_of_error(races: Vec<Race>) -> i32 {
    races.iter().map(|race| race.ways_to_win).product()
}

fn extract_races(input: String) -> Result<Vec<Race>, Day6Error> {
    let mut races: Vec<Race> = Vec::new();

    let mut lines = input.lines();
    let race_line = lines.next().ok_or(Day6Error::NoTimeLine)?;
    let race_times: Vec<i32> = race_line
        .split_whitespace()
        .filter_map(|str| str.parse::<i32>().ok())
        .collect();

    let distance_line = lines.next().ok_or(Day6Error::NoDistanceLine)?;
    let distances: Vec<i32> = distance_line
        .split_whitespace()
        .filter_map(|str| str.parse::<i32>().ok())
        .collect();

    println!("times: {:?}", race_times);
    println!("distances: {:?}", distances);

    if race_times.len() == distances.len() {
        for (i, time) in race_times.iter().enumerate() {
            races.push(Race::new(*time, distances[i]))
        }
    } else {
        return Err(Day6Error::UnequalTimesAndDistances);
    }

    Ok(races)
}

#[cfg(test)]
mod tests {
    use crate::{extract_races, load_input};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn parse_correct_races() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let result = extract_races(input.to_owned()).unwrap();
        check(
            &format!("{:?}", result),
            expect_test::expect!["[Race { total_time: 7, distance_to_beat: 9, ways_to_win: 4 }, Race { total_time: 15, distance_to_beat: 40, ways_to_win: 8 }, Race { total_time: 30, distance_to_beat: 200, ways_to_win: 9 }]"],
        );
    }
}
