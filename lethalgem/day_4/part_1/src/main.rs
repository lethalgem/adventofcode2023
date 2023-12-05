use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day4Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int from string")]
    ParseIntFailed(#[from] ParseIntError),
    #[error("Failed to find colon in string")]
    NoColonFound,
    #[error("Failed to find pipe in string")]
    NoPipeFound,
}

#[derive(Clone, PartialEq, Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    scratched_numbers: Vec<i32>,
    points: i32,
}

impl Card {
    fn new() -> Card {
        Card {
            winning_numbers: Vec::new(),
            scratched_numbers: Vec::new(),
            points: 0,
        }
    }

    fn set_winning_numbers(&mut self, numbers: Vec<i32>) {
        self.winning_numbers = numbers;
        self.points = self.calculate_points()
    }

    fn set_scratched_numbers(&mut self, numbers: Vec<i32>) {
        self.scratched_numbers = numbers;
        self.points = self.calculate_points()
    }

    fn calculate_points(&mut self) -> i32 {
        if self.winning_numbers.is_empty() || self.scratched_numbers.is_empty() {
            0
        } else {
            let matching_numbers: Vec<i32> = self
                .scratched_numbers
                .clone()
                .into_iter()
                .filter(|num| self.winning_numbers.contains(num))
                .collect();

            match matching_numbers.len() {
                0 => 0,
                1 => 1,
                len => 2_i32.pow((len - 1) as u32),
            }
        }
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day4Error> {
    let input_data = load_input("src/input.txt".to_string())?;
    let extracted_cards = extract_cards(input_data)?;
    let sum = sum_cards(extracted_cards);
    println!("{}", sum);
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day4Error> {
    let data = fs::read_to_string(file_path).map_err(Day4Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn sum_cards(cards: Vec<Card>) -> i32 {
    cards.iter().map(|card| card.points).sum()
}

fn extract_cards(input: String) -> Result<Vec<Card>, Day4Error> {
    let mut extracted_cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        extracted_cards.push(extract_card(line.to_owned())?)
    }

    Ok(extracted_cards)
}

fn extract_card(line: String) -> Result<Card, Day4Error> {
    let colon_index = line.find(':').ok_or(Day4Error::NoColonFound)?;
    let (_, number_sets) = line.split_at(colon_index);
    let cleaned_number_sets = number_sets
        .strip_prefix(':')
        .ok_or_else(|| Day4Error::NoColonFound)?;

    let pipe_index = cleaned_number_sets
        .find('|')
        .ok_or(Day4Error::NoPipeFound)?;
    let (winning_numbers, scratched_numbers) = cleaned_number_sets.split_at(pipe_index);
    let cleaned_scratched_numbers = scratched_numbers
        .strip_prefix('|')
        .ok_or_else(|| Day4Error::NoPipeFound)?;

    let mut card = Card::new();
    card.set_winning_numbers(extract_numbers(winning_numbers.to_owned())?);
    card.set_scratched_numbers(extract_numbers(cleaned_scratched_numbers.to_owned())?);
    Ok(card)
}

fn extract_numbers(set: String) -> Result<Vec<i32>, Day4Error> {
    let nums: Vec<&str> = set.split_ascii_whitespace().collect();

    let mut converted_nums: Vec<i32> = Vec::new();
    for num in nums {
        converted_nums.push(num.parse::<i32>()?)
    }
    Ok(converted_nums)
}

#[cfg(test)]
mod tests {
    use crate::{extract_card, extract_cards, load_input, sum_cards, Card};

    #[test]
    fn calculate_proper_points() {
        let mut card = Card::new();
        card.set_winning_numbers(vec![41, 48, 83, 86, 17]);
        card.set_scratched_numbers(vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.points, 8)
    }

    #[test]
    fn calculate_proper_solo_points() {
        let mut card = Card::new();
        card.set_winning_numbers(vec![41, 92, 73, 84, 69]);
        card.set_scratched_numbers(vec![59, 84, 76, 51, 58, 5, 54, 83]);
        assert_eq!(card.points, 1)
    }

    #[test]
    fn create_card() {
        let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let result = extract_card(input.to_owned()).unwrap();
        let mut test_card = Card::new();
        test_card.set_winning_numbers(vec![41, 92, 73, 84, 69]);
        test_card.set_scratched_numbers(vec![59, 84, 76, 51, 58, 5, 54, 83]);
        assert_eq!(result, test_card)
    }

    #[test]
    fn create_cards() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let result = extract_cards(input).unwrap();
        let mut test_card = Card::new();
        test_card.set_winning_numbers(vec![41, 48, 83, 86, 17]);
        test_card.set_scratched_numbers(vec![83, 86, 6, 31, 17, 9, 48, 53]);
        let mut test_card_2 = Card::new();
        test_card_2.set_winning_numbers(vec![13, 32, 20, 16, 61]);
        test_card_2.set_scratched_numbers(vec![61, 30, 68, 82, 17, 32, 24, 19]);
        let mut test_card_3 = Card::new();
        test_card_3.set_winning_numbers(vec![1, 21, 53, 59, 44]);
        test_card_3.set_scratched_numbers(vec![69, 82, 63, 72, 16, 21, 14, 1]);
        let mut test_card_4 = Card::new();
        test_card_4.set_winning_numbers(vec![41, 92, 73, 84, 69]);
        test_card_4.set_scratched_numbers(vec![59, 84, 76, 51, 58, 5, 54, 83]);
        let mut test_card_5 = Card::new();
        test_card_5.set_winning_numbers(vec![87, 83, 26, 28, 32]);
        test_card_5.set_scratched_numbers(vec![88, 30, 70, 12, 93, 22, 82, 36]);
        let mut test_card_6 = Card::new();
        test_card_6.set_winning_numbers(vec![31, 18, 13, 56, 72]);
        test_card_6.set_scratched_numbers(vec![74, 77, 10, 23, 35, 67, 36, 11]);
        assert_eq!(
            result,
            vec![
                test_card,
                test_card_2,
                test_card_3,
                test_card_4,
                test_card_5,
                test_card_6
            ]
        )
    }

    #[test]
    fn sum_card_set() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let cards = extract_cards(input).unwrap();
        let result = sum_cards(cards);
        assert_eq!(result, 13)
    }
}
