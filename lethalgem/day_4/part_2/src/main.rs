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
    next_cards_won: i32,
    copies_won: i32,
}

impl Card {
    fn new() -> Card {
        Card {
            winning_numbers: Vec::new(),
            scratched_numbers: Vec::new(),
            next_cards_won: 0,
            copies_won: 1,
        }
    }

    fn set_winning_numbers(&mut self, numbers: Vec<i32>) {
        self.winning_numbers = numbers;
    }

    fn set_scratched_numbers(&mut self, numbers: Vec<i32>) {
        self.scratched_numbers = numbers;
        self.next_cards_won = self.calculate_next_cards_won();
    }

    fn add_copy_won(&mut self) {
        self.copies_won += 1
    }

    fn calculate_next_cards_won(&mut self) -> i32 {
        if self.winning_numbers.is_empty() || self.scratched_numbers.is_empty() {
            0
        } else {
            let matching_numbers: Vec<i32> = self
                .scratched_numbers
                .clone()
                .into_iter()
                .filter(|num| self.winning_numbers.contains(num))
                .collect();

            matching_numbers.len() as i32
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
    let total_copies = calculate_copies_won(extracted_cards);
    println!("{}", total_copies);
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day4Error> {
    let data = fs::read_to_string(file_path).map_err(Day4Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn calculate_copies_won(cards: Vec<Card>) -> i32 {
    let mut updated_cards: Vec<Card> = cards.clone();
    for (i, _) in cards.iter().enumerate() {
        println!("At card: {}", i);
        for _ in 1..=updated_cards.clone()[i].copies_won {
            updated_cards = update_copies_won_for_next_cards(
                updated_cards.clone(),
                i,
                updated_cards[i].clone(),
            );
        }
    }

    updated_cards.iter().map(|card| card.copies_won).sum()
}

fn update_copies_won_for_next_cards(
    cards: Vec<Card>,
    index_of_current_card: usize,
    current_card: Card,
) -> Vec<Card> {
    let mut updated_cards: Vec<Card> = Vec::new();
    for (i, card) in cards.iter().enumerate() {
        let mut updated_card = card.clone();
        if i > index_of_current_card
            && i <= index_of_current_card + (current_card.next_cards_won as usize)
        {
            updated_card.add_copy_won();
        }

        updated_cards.push(updated_card);
    }
    updated_cards
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
    use crate::{
        calculate_copies_won, extract_card, extract_cards, load_input,
        update_copies_won_for_next_cards, Card,
    };

    #[test]
    fn calculate_proper_next_cards_won() {
        let mut card = Card::new();
        card.set_winning_numbers(vec![41, 48, 83, 86, 17]);
        card.set_scratched_numbers(vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.next_cards_won, 4)
    }

    #[test]
    fn update_next_copies_won() {
        let mut card = Card::new();
        card.set_winning_numbers(vec![41, 48, 83, 86, 17]);
        card.set_scratched_numbers(vec![83, 86, 6, 31, 17, 9, 48, 53]);
        let mut card_2 = Card::new();
        card_2.set_winning_numbers(vec![13, 32, 20, 16, 61]);
        card_2.set_scratched_numbers(vec![61, 30, 68, 82, 17, 32, 24, 19]);
        let mut card_3 = Card::new();
        card_3.set_winning_numbers(vec![1, 21, 53, 59, 44]);
        card_3.set_scratched_numbers(vec![69, 82, 63, 72, 16, 21, 14, 1]);
        let cards = vec![card.clone(), card_2.clone(), card_3];
        let updated_cards = update_copies_won_for_next_cards(cards, 0, card);
        let result = update_copies_won_for_next_cards(updated_cards, 1, card_2);

        let test_card = Card {
            winning_numbers: vec![41, 48, 83, 86, 17],
            scratched_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            next_cards_won: 4,
            copies_won: 1,
        };
        let test_card_2 = Card {
            winning_numbers: vec![13, 32, 20, 16, 61],
            scratched_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            next_cards_won: 2,
            copies_won: 2,
        };
        let test_card_3 = Card {
            winning_numbers: vec![1, 21, 53, 59, 44],
            scratched_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            next_cards_won: 2,
            copies_won: 3,
        };
        let test_cards = vec![test_card, test_card_2, test_card_3];

        assert_eq!(result, test_cards)
    }

    #[test]
    fn calculate_copies_of_simple_example() {
        let mut card = Card::new();
        card.set_winning_numbers(vec![41, 48, 83, 86, 17]); // wins 2
        card.set_scratched_numbers(vec![83, 48, 6, 31, 5, 9, 1, 53]); // 1 copy
        let mut card_2 = Card::new();
        card_2.set_winning_numbers(vec![13, 32, 20, 16, 61]); // wins 1
        card_2.set_scratched_numbers(vec![61, 1, 2, 3, 4, 5, 6, 7]); // 2 copies
        let mut card_3 = Card::new();
        card_3.set_winning_numbers(vec![1, 21, 53, 59, 44]); // wins 0
        card_3.set_scratched_numbers(vec![69, 82, 63, 72, 16, 0, 14, 2]); // 4 copies
        let mut card_4 = Card::new();
        card_4.set_winning_numbers(vec![1, 2, 3, 4, 5]); // wins 0
        card_4.set_scratched_numbers(vec![6, 7, 8, 9, 11, 12, 13, 14]); // 1 copy
        let cards = vec![card.clone(), card_2, card_3, card_4];

        let result = calculate_copies_won(cards);

        assert_eq!(result, 8)
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
    fn calculate_copies_won_for_example() {
        let input = load_input("src/example_1.txt".to_string()).unwrap();
        let extracted_cards = extract_cards(input).unwrap();
        let result = calculate_copies_won(extracted_cards);
        assert_eq!(result, 30)
    }
}
