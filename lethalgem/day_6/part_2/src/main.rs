use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day6Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse time: line")]
    NoTimeLine,
    #[error("Failed to parse distance: line")]
    NoDistanceLine,
    #[error("Failed to parse int")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Clone)]
struct Race {
    ways_to_win: u64,
}

impl Race {
    fn new(total_time: u64, distance_to_beat: u64) -> Race {
        Race {
            ways_to_win: Self::calculate_ways_to_win(total_time, distance_to_beat),
        }
    }

    fn calculate_ways_to_win(total_time: u64, distance_to_beat: u64) -> u64 {
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

    println!("parsing race, time elapsed: {:?}", start.elapsed());
    let race = extract_race(input_data.to_owned())?;

    println!(
        "ways_to_win: {}, time elapsed: {:?}",
        race.ways_to_win,
        start.elapsed()
    );

    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day6Error> {
    let data = fs::read_to_string(file_path).map_err(Day6Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn extract_race(input: String) -> Result<Race, Day6Error> {
    let mut lines = input.lines();
    let race_line = lines.next().ok_or(Day6Error::NoTimeLine)?;
    let mut race_time = String::new();
    for str in race_line.split_whitespace() {
        if str.parse::<u64>().is_ok() {
            race_time.push_str(str);
        }
    }

    let distance_line = lines.next().ok_or(Day6Error::NoDistanceLine)?;
    let mut race_distance = String::new();
    for str in distance_line.split_whitespace() {
        if str.parse::<u64>().is_ok() {
            race_distance.push_str(str);
        }
    }

    println!("race_time: {}", race_time);
    println!("race_distance: {}", race_distance);

    let race = Race::new(race_time.parse::<u64>()?, race_distance.parse::<u64>()?);
    Ok(race)
}

#[cfg(test)]
mod tests {
    use crate::{extract_race, load_input};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn parse_correct_race() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let result = extract_race(input.to_owned()).unwrap();
        check(
            &format!("{:?}", result),
            expect_test::expect!["Race { ways_to_win: 71503 }"],
        );
    }
}
