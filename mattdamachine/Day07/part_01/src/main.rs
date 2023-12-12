// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand.
// A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
// The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

// The seven types of hands from strongest to weakest:
// 7. Five of a kind, where all five cards have the same label: AAAAA
// 6. Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// 5. Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// 4. Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// 3. Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// 2. One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// 1. High card, where all cards' labels are distinct: 23456

// If two hands have the same type, then you compare one card from each hand
// (using the strength at the top) until one is stronger than the other.

// In this input, there are 1000 hands, put them in order of their strength and then multiply
// that rank by their bid. Total up this amount for the answer.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    hand_type: u32, // Key for types will be above
    bid: u32,
}

impl Hand {
    fn new(cards: String, bid: u32) -> Self {
        Hand {
            cards,
            hand_type: 0,
            bid,
        }
    }
}

enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
}

impl CardValue {
    fn from(ch: char) -> Self {
        match ch {
            '2' => CardValue::Two,
            '3' => CardValue::Three,
            '4' => CardValue::Four,
            '5' => CardValue::Five,
            '6' => CardValue::Six,
            '7' => CardValue::Seven,
            '8' => CardValue::Eight,
            '9' => CardValue::Nine,
            'T' => CardValue::Ten,
            'J' => CardValue::Eleven,
            'Q' => CardValue::Twelve,
            'K' => CardValue::Thirteen,
            'A' => CardValue::Fourteen,
            _ => unreachable!("Other kinds of cards do not exist"),
        }
    }
    fn from_enum(card_value: CardValue) -> u32 {
        match card_value {
            CardValue::Two => 2,
            CardValue::Three => 3,
            CardValue::Four => 4,
            CardValue::Five => 5,
            CardValue::Six => 6,
            CardValue::Seven => 7,
            CardValue::Eight => 8,
            CardValue::Nine => 9,
            CardValue::Ten => 10,
            CardValue::Eleven => 11,
            CardValue::Twelve => 12,
            CardValue::Thirteen => 13,
            CardValue::Fourteen => 14,
        }
    }
}

#[derive(Error, Debug)]
enum Day7Error {
    #[error("Cannot read contents from file")]
    CannotReadFile(#[from] std::io::Error),
}

fn read_file_into_buffer(file_path: &str) -> Result<BufReader<File>, Day7Error> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

fn extract_hands_from_file(reader: BufReader<File>) -> Result<Vec<Hand>, Day7Error> {
    let mut hands = vec![];

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let hand = Hand::new(parts[0].to_string(), parts[1].parse::<u32>().unwrap());
        hands.push(hand);
    }

    Ok(hands)
}

fn find_hand_type(hand: &mut Hand) -> &mut Hand {
    let mut hand_map: HashMap<String, u32> = HashMap::new();

    // Populate hand map with the count that each card appears
    for card in hand.cards.chars() {
        if let std::collections::hash_map::Entry::Vacant(e) = hand_map.entry(card.to_string()) {
            e.insert(1);
        } else if let Some(value) = hand_map.get_mut(&card.to_string()) {
            *value += 1;
        }
    }

    // Find the type of the hand and modify the hand accordingly
    // 23332 -> [2: 2, 3: 3].len() = 2
    for (_, count) in hand_map.iter() {
        let count = *count;
        if count == 5 {
            // Five of a kind
            hand.hand_type = 7;
            break;
        } else if count == 4 {
            // Four of a kind
            hand.hand_type = 6;
            break;
        } else if count == 3 {
            if hand_map.len() == 2 {
                // Full house
                hand.hand_type = 5;
                break;
            } else {
                // Three of a kind
                hand.hand_type = 4;
                break;
            }
        } else if count == 2 {
            if hand_map.len() == 3 {
                // Two pair
                hand.hand_type = 3;
                break;
            } else if hand_map.len() == 4 {
                // One pair
                hand.hand_type = 2;
                break;
            }
        }
    }

    // No previous hands were found - default to high card
    if hand.hand_type == 0 {
        hand.hand_type = 1;
    }

    hand
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error running program: {}", e);
    }
}

fn run() -> Result<(), Day7Error> {
    let file_path =
        "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day07/part_01/input.txt";

    let reader = read_file_into_buffer(file_path)?;

    let mut hands: Vec<Hand> = extract_hands_from_file(reader)?;

    for hand in &mut hands {
        find_hand_type(hand);
    }

    let mut sum = 0;

    hands.sort_by_key(|hand| {
        (
            hand.hand_type,
            hand.cards
                .chars()
                .map(|card_char| {
                    let card = CardValue::from(card_char);
                    CardValue::from_enum(card)
                })
                .collect::<Vec<u32>>(),
        )
    });

    for (i, hand) in hands.iter().enumerate() {
        println!("i: {}, bid: {}", i, hand.bid);
        sum += (i + 1) * hand.bid as usize
    }

    println!("Sum is {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{extract_hands_from_file, find_hand_type, read_file_into_buffer};

    #[test]
    fn get_hand_type() {
        let file_path =
            "/Users/Mattdamachine/Code/adventofcode2023/mattdamachine/Day07/part_01/examples.txt";
        let reader = read_file_into_buffer(file_path).unwrap();
        let mut hands = extract_hands_from_file(reader).unwrap();
        for hand in &mut hands {
            find_hand_type(hand);
        }

        let result_hand_types = hands
            .iter()
            .map(|hand| hand.hand_type)
            .collect::<Vec<u32>>();

        assert_eq!(result_hand_types, vec![7, 6, 5, 4, 3, 2, 1])
    }
}
