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
    minimum_set: Set,
}

impl Game {
    fn new_with_values(id: i32, sets: Vec<Set>) -> Game {
        let mut minimum_red_count = 0;
        let mut minimum_green_count = 0;
        let mut minimum_blue_count = 0;

        for set in &sets {
            if set.red_count > minimum_red_count {
                minimum_red_count = set.red_count
            }
            if set.green_count > minimum_green_count {
                minimum_green_count = set.green_count
            }
            if set.blue_count > minimum_blue_count {
                minimum_blue_count = set.blue_count
            }
        }

        Game {
            id,
            sets,
            minimum_set: Set::new_with_values(
                minimum_red_count,
                minimum_green_count,
                minimum_blue_count,
            ),
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
    let power = calculate_overall_power(games);
    println!("{}", power);
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day2Error> {
    let data = fs::read_to_string(file_path).map_err(Day2Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn calculate_overall_power(games: Vec<Game>) -> i32 {
    games
        .iter()
        .map(|game| calculate_game_power(game.to_owned()))
        .sum()
}

fn calculate_game_power(game: &Game) -> i32 {
    game.minimum_set.red_count * game.minimum_set.green_count * game.minimum_set.blue_count
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
        calculate_game_power, calculate_overall_power, extract_game, extract_game_id,
        extract_games, extract_set, extract_sets, load_input, Game, Set,
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
                minimum_set: Set::new_with_values(20, 13, 6),
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
                    minimum_set: Set::new_with_values(4, 2, 6),
                },
                Game {
                    id: 2,
                    sets: vec![
                        Set::new_with_values(0, 2, 1),
                        Set::new_with_values(1, 3, 4),
                        Set::new_with_values(0, 1, 1)
                    ],
                    minimum_set: Set::new_with_values(1, 3, 4),
                },
                Game {
                    id: 3,
                    sets: vec![
                        Set::new_with_values(20, 8, 6),
                        Set::new_with_values(4, 13, 5),
                        Set::new_with_values(1, 5, 0)
                    ],
                    minimum_set: Set::new_with_values(20, 13, 6),
                },
                Game {
                    id: 4,
                    sets: vec![
                        Set::new_with_values(3, 1, 6),
                        Set::new_with_values(6, 3, 0),
                        Set::new_with_values(14, 3, 15)
                    ],
                    minimum_set: Set::new_with_values(14, 3, 15),
                },
                Game {
                    id: 5,
                    sets: vec![Set::new_with_values(6, 3, 1), Set::new_with_values(1, 2, 2),],
                    minimum_set: Set::new_with_values(6, 3, 2),
                },
            ]
        )
    }

    #[test]
    fn one_game_power() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = extract_game(input.to_owned()).unwrap();
        let result = calculate_game_power(&game);
        assert_eq!(result, 48)
    }

    #[test]
    fn all_game_power() {
        let input = load_input("src/example_1.txt".to_string()).unwrap();
        let games = extract_games(input).unwrap();
        let result = calculate_overall_power(games);
        assert_eq!(result, 2286)
    }
}
