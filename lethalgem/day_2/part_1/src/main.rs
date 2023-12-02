use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day2Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Could not find number in line")]
    NoNumberFound,
    #[error("Failed to parse int from string")]
    ParseIntFailed(#[from] ParseIntError),
    #[error("Could not find colon in line")]
    NoColonFound,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    sets: Vec<Set>,
    is_possible: Option<bool>,
}

impl Game {
    const POSSIBLE_SET: Set = Set {
        red_count: 12,
        green_count: 13,
        blue_count: 14,
    };

    fn new_with_values(id: i32, sets: Vec<Set>) -> Game {
        let mut is_possible = true;
        for set in &sets {
            if set.red_count > Self::POSSIBLE_SET.red_count
                || set.green_count > Self::POSSIBLE_SET.green_count
                || set.blue_count > Self::POSSIBLE_SET.blue_count
            {
                is_possible = false;
            }
        }
        Game {
            id,
            sets,
            is_possible: Some(is_possible),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Set {
    red_count: i32,
    green_count: i32,
    blue_count: i32,
}

impl Set {
    fn new() -> Set {
        Set {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        }
    }

    #[warn(dead_code)]
    fn new_with_values(red_count: i32, green_count: i32, blue_count: i32) -> Set {
        Set {
            red_count,
            green_count,
            blue_count,
        }
    }

    fn set_red_count(&mut self, count: i32) {
        self.red_count = count
    }

    fn set_green_count(&mut self, count: i32) {
        self.green_count = count
    }

    fn set_blue_count(&mut self, count: i32) {
        self.blue_count = count
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day2Error> {
    let input_data = load_input("src/input.txt".to_string())?;
    let games = extract_games(input_data)?;
    let sum = sum_possible_games(games);
    println!("{}", sum);
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day2Error> {
    let data = fs::read_to_string(file_path).map_err(Day2Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn sum_possible_games(games: Vec<Game>) -> i32 {
    games
        .iter()
        .map(|game| {
            if game.is_possible == Some(true) {
                game.id
            } else {
                0
            }
        })
        .sum()
}

fn extract_games(input: String) -> Result<Vec<Game>, Day2Error> {
    let lines = input.lines();

    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        games.push(extract_game(line.to_owned())?)
    }

    Ok(games)
}

fn extract_game(input: String) -> Result<Game, Day2Error> {
    let colon_index = input.find(':').ok_or(Day2Error::NoColonFound)?;
    let (game, sets) = input.split_at(colon_index);

    Ok(Game::new_with_values(
        extract_game_id(game.to_owned())?,
        extract_sets(sets.to_owned())?,
    ))
}

fn extract_game_id(input: String) -> Result<i32, Day2Error> {
    extract_number_from_line(input)
}

fn extract_sets(input: String) -> Result<Vec<Set>, Day2Error> {
    let delimited_sets = input.split(';');
    let mut sets: Vec<Set> = Vec::new();
    for set in delimited_sets {
        sets.push(extract_set(set.to_owned())?)
    }
    Ok(sets)
}

fn extract_set(input: String) -> Result<Set, Day2Error> {
    let color_counts = input.split(',');
    let mut set = Set::new();

    for color_count in color_counts {
        let count = extract_number_from_line(color_count.to_owned())?;
        if color_count.contains("red") {
            set.set_red_count(count)
        } else if color_count.contains("green") {
            set.set_green_count(count)
        } else if color_count.contains("blue") {
            set.set_blue_count(count)
        }
    }

    Ok(set)
}

fn extract_number_from_line(line: String) -> Result<i32, Day2Error> {
    let mut digits = String::new();

    for c in line.chars() {
        if c.is_ascii_digit() {
            digits.push(c)
        }
    }

    match digits.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(e) => Err(Day2Error::ParseIntFailed(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        extract_game, extract_game_id, extract_games, extract_set, extract_sets, load_input,
        sum_possible_games, Game, Set,
    };

    #[test]
    fn set() {
        let input = "1 green, 3 red, 6 blue;";
        let result = extract_set(input.to_owned()).unwrap();
        assert_eq!(result, Set::new_with_values(3, 1, 6))
    }

    #[test]
    fn sets() {
        let input = "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let result = extract_sets(input.to_owned()).unwrap();
        assert_eq!(
            result,
            vec![
                Set::new_with_values(0, 2, 1),
                Set::new_with_values(1, 3, 4),
                Set::new_with_values(0, 1, 1)
            ]
        )
    }

    #[test]
    fn game_id() {
        let input = "Game 2:";
        let result = extract_game_id(input.to_owned()).unwrap();
        assert_eq!(result, 2)
    }

    #[test]
    fn game() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let result = extract_game(input.to_owned()).unwrap();
        assert_eq!(
            result,
            Game {
                id: 3,
                sets: vec![
                    Set::new_with_values(20, 8, 6),
                    Set::new_with_values(4, 13, 5),
                    Set::new_with_values(1, 5, 0)
                ],
                is_possible: Some(false)
            }
        )
    }

    #[test]
    fn extract_all_games() {
        let input = load_input("src/example_1.txt".to_string()).unwrap();
        let result = extract_games(input.to_owned()).unwrap();
        assert_eq!(
            result,
            vec![
                Game {
                    id: 1,
                    sets: vec![
                        Set::new_with_values(4, 0, 3),
                        Set::new_with_values(1, 2, 6),
                        Set::new_with_values(0, 2, 0)
                    ],
                    is_possible: Some(true)
                },
                Game {
                    id: 2,
                    sets: vec![
                        Set::new_with_values(0, 2, 1),
                        Set::new_with_values(1, 3, 4),
                        Set::new_with_values(0, 1, 1)
                    ],
                    is_possible: Some(true)
                },
                Game {
                    id: 3,
                    sets: vec![
                        Set::new_with_values(20, 8, 6),
                        Set::new_with_values(4, 13, 5),
                        Set::new_with_values(1, 5, 0)
                    ],
                    is_possible: Some(false)
                },
                Game {
                    id: 4,
                    sets: vec![
                        Set::new_with_values(3, 1, 6),
                        Set::new_with_values(6, 3, 0),
                        Set::new_with_values(14, 3, 15)
                    ],
                    is_possible: Some(false)
                },
                Game {
                    id: 5,
                    sets: vec![Set::new_with_values(6, 3, 1), Set::new_with_values(1, 2, 2),],
                    is_possible: Some(true)
                },
            ]
        )
    }

    #[test]
    fn sum_games() {
        let input = load_input("src/example_1.txt".to_string()).unwrap();
        let games = extract_games(input.to_owned()).unwrap();
        let result = sum_possible_games(games);
        assert_eq!(result, 8)
    }
}
