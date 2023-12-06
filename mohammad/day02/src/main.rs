use core::cmp::max;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

enum Part {
    One,
    Two,
}

fn solution(part: Part) {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut result = 0;

    for line in BufReader::new(handle).lines() {
        let line = line.unwrap();
        let split: Vec<_> = line.split(&[':', ';']).collect();

        // Get game ID. First, remove the prefix "Game ".
        let game_id = &split.first().expect("must have a game ID").to_string()[5..]
            .parse::<usize>()
            .expect("the remaining characters must form an integer");
        let mut game = Game {
            id: *game_id,
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        // Remaining entries in `split` are game rounds
        for s in &split[1..] {
            let rounds: Vec<_> = s.split(',').collect();
            for r in rounds.iter() {
                let round_data: Vec<_> = r.trim().split(' ').collect();
                let amount = round_data[0]
                    .parse::<usize>()
                    .expect("the amount must form an integer");
                match round_data[1] {
                    "red" => game.max_red = max(game.max_red, amount),
                    "green" => game.max_green = max(game.max_green, amount),
                    "blue" => game.max_blue = max(game.max_blue, amount),
                    _ => unreachable!(),
                }
            }
        }

        match part {
            Part::One => {
                if game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14 {
                    result += game.id;
                }
            }
            Part::Two => result += game.max_red * game.max_green * game.max_blue,
        }
    }

    println!(
        "Part {} answer: {result}",
        match part {
            Part::One => 1,
            Part::Two => 2,
        }
    );
}

fn main() {
    solution(Part::One);
    solution(Part::Two);
}
