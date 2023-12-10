use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn get_type(hand: &Vec<usize>) -> Type {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for c in hand {
        map.entry(*c).and_modify(|value| *value += 1).or_insert(1);
    }

    let mut values = map.values().collect::<Vec<_>>();
    values.sort();

    match values[..] {
        [5] => Type::FiveOfAKind,
        [1, 4] => Type::FourOfAKind,
        [2, 3] => Type::FullHouse,
        [1, 1, 3] => Type::ThreeOfAKind,
        [1, 2, 2] => Type::TwoPairs,
        [1, 1, 1, 2] => Type::OnePair,
        [1, 1, 1, 1, 1] => Type::HighCard,
        _ => unreachable!("Unexpected pattern"),
    }
}

fn solution() {
    let path: PathBuf = "src/input_small".into();
    let handle = File::open(path.clone()).unwrap();

    let mut hands = Vec::new();
    for line in BufReader::new(handle).lines() {
        let line = line.unwrap();
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let hand = split[0]
            .chars()
            .map(|c| match c {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => c
                    .to_string()
                    .parse::<usize>()
                    .expect("All other suits should be numbers"),
            })
            .collect::<Vec<_>>();

        hands.push((
            hand.clone(),
            get_type(&hand),
            split[1]
                .parse::<usize>()
                .expect("the bid must be an integer"),
        ));
    }

    // Sort by hand type and *then* by the hand itself.
    hands.sort_by(|(h1, t1, _), (h2, t2, _)| (t1, h1).cmp(&(t2, h2)));

    println!(
        "Part 1 answer: {}",
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, (_, _, b))| { acc + (idx + 1) * b })
    );
}

fn main() {
    solution();
}
