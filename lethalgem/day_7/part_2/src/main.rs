use std::{cmp::Reverse, collections::HashMap, fs, io, num::ParseIntError};
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day7Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int")]
    ParseIntFailed(#[from] ParseIntError),
    #[error("Failed to parse card from character {0}")]
    CouldNotConstructCard(char),
    #[error("Failed to sort in time")]
    FailedToSort,
    #[error("Failed to find hand with index")]
    FailedToFindHand,
    #[error("Failed to find card with highest count in hand")]
    FailedToFindHighestCardCount,
}

#[derive(Debug, Clone, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    type_: HandType,
    bid: i32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: i32) -> Result<Hand, Day7Error> {
        Ok(Hand {
            cards: cards.clone(),
            type_: Self::find_type(cards)?,
            bid,
        })
    }

    fn find_type(cards: Vec<Card>) -> Result<HandType, Day7Error> {
        let mut card_count: HashMap<Card, i32> = HashMap::new();
        for card in cards {
            match card_count.clone().get(&card) {
                Some(count) => {
                    card_count.insert(card.clone(), count + 1);
                }
                None => {
                    card_count.insert(card.clone(), 1);
                }
            };
        }

        let card_count_with_jokers = Self::convert_jokers_for_count(&card_count)?;

        if Self::find_count_of_counts(5, &card_count_with_jokers) == 1 {
            Ok(HandType::FiveOfAKind)
        } else if Self::find_count_of_counts(4, &card_count_with_jokers) == 1 {
            Ok(HandType::FourOfAKind)
        } else if Self::find_count_of_counts(2, &card_count_with_jokers) == 1
            && Self::find_count_of_counts(3, &card_count_with_jokers) == 1
        {
            Ok(HandType::FullHouse)
        } else if Self::find_count_of_counts(3, &card_count_with_jokers) == 1 {
            Ok(HandType::ThreeOfAKind)
        } else if Self::find_count_of_counts(2, &card_count_with_jokers) == 2 {
            Ok(HandType::TwoPair)
        } else if Self::find_count_of_counts(2, &card_count_with_jokers) == 1 {
            Ok(HandType::OnePair)
        } else {
            Ok(HandType::HighCard)
        }
    }

    fn find_count_of_counts(desired_count: i32, card_count: &HashMap<Card, i32>) -> i32 {
        card_count
            .iter()
            .filter(|(_, count)| count == &&desired_count)
            .count() as i32
    }

    fn convert_jokers_for_count(
        card_count: &HashMap<Card, i32>,
    ) -> Result<HashMap<Card, i32>, Day7Error> {
        let joker_count = card_count.get(&Card::Jack).unwrap_or(&0);

        let mut new_card_count = card_count.clone();
        new_card_count.insert(Card::Jack, 0);

        let highest_card_count = new_card_count
            .iter()
            .max_by_key(|entry| entry.1)
            .ok_or_else(|| Day7Error::FailedToFindHighestCardCount)?;

        new_card_count.insert(
            highest_card_count.0.clone(),
            highest_card_count.1 + joker_count,
        );

        Ok(new_card_count)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(char: char) -> Result<Card, Day7Error> {
        match char {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(Day7Error::CouldNotConstructCard(char)),
        }
    }

    fn strength(&self) -> i32 {
        match *self {
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 0,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn strength(&self) -> i32 {
        match *self {
            HandType::FiveOfAKind => 7,  // AAAAA
            HandType::FourOfAKind => 6,  // AA8AA
            HandType::FullHouse => 5,    // 23332
            HandType::ThreeOfAKind => 4, // TTT98
            HandType::TwoPair => 3,      // 23432
            HandType::OnePair => 2,      // A23A4
            HandType::HighCard => 1,     // 23456
        }
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day7Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt".to_string())?;

    println!("parsing hands, time elapsed: {:?}", start.elapsed());
    let mut hands = extract_hands(input_data.to_owned())?;

    println!("ranking hands: {:?}", start.elapsed());
    rank_hands(&mut hands);

    println!("calcing total winnings : {:?}", start.elapsed());
    let total_winnings = calc_total_winnings(hands);

    println!(
        "total winnings: {}, time elapsed: {:?}",
        total_winnings,
        start.elapsed()
    );

    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day7Error> {
    let data = fs::read_to_string(file_path).map_err(Day7Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn extract_hands(input: String) -> Result<Vec<Hand>, Day7Error> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        if let Some((cards_c, bid_c)) = line.split_once(' ') {
            let mut cards: Vec<Card> = Vec::new();
            for card in cards_c.chars() {
                cards.push(Card::new(card)?)
            }
            hands.push(Hand::new(cards, bid_c.parse::<i32>()?)?)
        }
    }
    Ok(hands)
}

fn rank_hands(hands: &mut [Hand]) {
    hands.sort_by_key(|hand| {
        Reverse((
            hand.type_.strength(),
            hand.cards
                .iter()
                .map(|card| card.strength())
                .collect::<Vec<i32>>(),
        ))
    });
}

fn calc_total_winnings(hands: Vec<Hand>) -> i32 {
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += (hands.len() - i) as i32 * hand.bid;
    }
    total
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{calc_total_winnings, extract_hands, load_input, rank_hands, Card, Hand, HandType};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn find_pairs() {
        let mut input: HashMap<Card, i32> = HashMap::new();
        input.insert(Card::new('J').unwrap(), 2);
        input.insert(Card::new('2').unwrap(), 1);
        let result = Hand::find_count_of_counts(2, &input);
        check(&format!("{:?}", result), expect_test::expect!["1"]);

        input.insert(Card::new('Q').unwrap(), 2);
        let result = Hand::find_count_of_counts(2, &input);
        check(&format!("{:?}", result), expect_test::expect!["2"]);
    }

    #[test]
    fn find_correct_hand_types() {
        fn construct_hand_type(input: &str) -> HandType {
            let mut cards: Vec<Card> = Vec::new();
            for c in input.chars() {
                cards.push(Card::new(c).unwrap())
            }
            Hand::find_type(cards).unwrap()
        }

        let result = construct_hand_type("AAAAA");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FiveOfAKind"],
        );

        let result = construct_hand_type("AA8AA");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FourOfAKind"],
        );

        let result = construct_hand_type("23332");
        check(&format!("{:?}", result), expect_test::expect!["FullHouse"]);

        let result = construct_hand_type("TTT98");
        check(
            &format!("{:?}", result),
            expect_test::expect!["ThreeOfAKind"],
        );

        let result = construct_hand_type("23432");
        check(&format!("{:?}", result), expect_test::expect!["TwoPair"]);

        let result = construct_hand_type("A23A4");
        check(&format!("{:?}", result), expect_test::expect!["OnePair"]);

        let result = construct_hand_type("23456");
        check(&format!("{:?}", result), expect_test::expect!["HighCard"]);

        let result = construct_hand_type("2222J");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FiveOfAKind"],
        );

        let result = construct_hand_type("222JJ");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FiveOfAKind"],
        );

        let result = construct_hand_type("22JJJ");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FiveOfAKind"],
        );

        let result = construct_hand_type("2JJJJ");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FiveOfAKind"],
        );

        let result = construct_hand_type("222J3");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FourOfAKind"],
        );

        let result = construct_hand_type("22JJ3");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FourOfAKind"],
        );

        let result = construct_hand_type("2JJJ3");
        check(
            &format!("{:?}", result),
            expect_test::expect!["FourOfAKind"],
        );

        let result = construct_hand_type("2233J");
        check(&format!("{:?}", result), expect_test::expect!["FullHouse"]);

        let result = construct_hand_type("22J56");
        check(
            &format!("{:?}", result),
            expect_test::expect!["ThreeOfAKind"],
        );

        let result = construct_hand_type("2JJ56");
        check(
            &format!("{:?}", result),
            expect_test::expect!["ThreeOfAKind"],
        );

        let result = construct_hand_type("2J345");
        check(&format!("{:?}", result), expect_test::expect!["OnePair"]);
    }

    #[test]
    fn extract_proper_hands() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let result = extract_hands(input).unwrap();
        check(
            &format!("{:?}", result),
            expect_test::expect!["[Hand { cards: [Three, Two, Ten, Three, King], type_: OnePair, bid: 765 }, Hand { cards: [Ten, Five, Five, Jack, Five], type_: FourOfAKind, bid: 684 }, Hand { cards: [King, King, Six, Seven, Seven], type_: TwoPair, bid: 28 }, Hand { cards: [King, Ten, Jack, Jack, Ten], type_: FourOfAKind, bid: 220 }, Hand { cards: [Queen, Queen, Queen, Jack, Ace], type_: FourOfAKind, bid: 483 }]"],
        );
    }

    #[test]
    fn sort_test() {
        let input = "2AAAA 435\nA2AAA 555";
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        check(&format!("{:?}", hands), expect_test::expect!["[Hand { cards: [Ace, Two, Ace, Ace, Ace], type_: FourOfAKind, bid: 555 }, Hand { cards: [Two, Ace, Ace, Ace, Ace], type_: FourOfAKind, bid: 435 }]"]);

        let input = "A2AAA 435\nAA2AA 555";
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        check(&format!("{:?}", hands), expect_test::expect!["[Hand { cards: [Ace, Ace, Two, Ace, Ace], type_: FourOfAKind, bid: 555 }, Hand { cards: [Ace, Two, Ace, Ace, Ace], type_: FourOfAKind, bid: 435 }]"]);

        let input = "AA2AA 435\nAAA2A 555";
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        check(&format!("{:?}", hands), expect_test::expect!["[Hand { cards: [Ace, Ace, Ace, Two, Ace], type_: FourOfAKind, bid: 555 }, Hand { cards: [Ace, Ace, Two, Ace, Ace], type_: FourOfAKind, bid: 435 }]"]);

        let input = "AAA2A 435\nAAAA2 555";
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        check(&format!("{:?}", hands), expect_test::expect!["[Hand { cards: [Ace, Ace, Ace, Ace, Two], type_: FourOfAKind, bid: 555 }, Hand { cards: [Ace, Ace, Ace, Two, Ace], type_: FourOfAKind, bid: 435 }]"]);

        let input = "KK677 28\nKTJJT 220";
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        check(&format!("{:?}", hands), expect_test::expect!["[Hand { cards: [King, Ten, Jack, Jack, Ten], type_: FourOfAKind, bid: 220 }, Hand { cards: [King, King, Six, Seven, Seven], type_: TwoPair, bid: 28 }]"]);

        let input = "T55J5 684\nQQQJA 483";
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        check(&format!("{:?}", hands), expect_test::expect!["[Hand { cards: [Queen, Queen, Queen, Jack, Ace], type_: FourOfAKind, bid: 483 }, Hand { cards: [Ten, Five, Five, Jack, Five], type_: FourOfAKind, bid: 684 }]"]);
    }

    #[test]
    fn sort_all_hands_test() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        check(&format!("{:?}", hands), expect_test::expect!["[Hand { cards: [King, Ten, Jack, Jack, Ten], type_: FourOfAKind, bid: 220 }, Hand { cards: [Queen, Queen, Queen, Jack, Ace], type_: FourOfAKind, bid: 483 }, Hand { cards: [Ten, Five, Five, Jack, Five], type_: FourOfAKind, bid: 684 }, Hand { cards: [King, King, Six, Seven, Seven], type_: TwoPair, bid: 28 }, Hand { cards: [Three, Two, Ten, Three, King], type_: OnePair, bid: 765 }]"]);
    }

    #[test]
    fn calc_total_winnings_test() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let mut hands = extract_hands(input.to_owned()).unwrap();
        rank_hands(&mut hands);
        let result = calc_total_winnings(hands);
        check(&format!("{:?}", result), expect_test::expect!["5905"]);
    }
}
