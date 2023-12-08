use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

enum Part {
    One,
    Two,
}

fn solution(part: &Part) {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();
    let mut lines = BufReader::new(handle).lines();

    let mut result = 1;

    let races = match part {
        Part::One => lines
            .next()
            .unwrap()
            .unwrap()
            .split(':')
            .collect::<Vec<_>>()[1]
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .zip(
                lines
                    .next()
                    .unwrap()
                    .unwrap()
                    .split(':')
                    .collect::<Vec<_>>()[1]
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap()),
            )
            .collect::<Vec<_>>(),
        Part::Two => {
            let mut times = lines
                .next()
                .unwrap()
                .unwrap()
                .split(':')
                .collect::<Vec<_>>()[1]
                .to_string();
            times.retain(|c| c.is_ascii_digit());

            let mut distances = lines
                .next()
                .unwrap()
                .unwrap()
                .split(':')
                .collect::<Vec<_>>()[1]
                .to_string();
            distances.retain(|c| c.is_ascii_digit());

            vec![(
                times.parse::<u64>().unwrap(),
                distances.parse::<u64>().unwrap(),
            )]
        }
    };

    for (time, distance) in races {
        let mut wins = 0;
        for t in 0..time {
            wins += u64::from(t * (time - t) > distance);
        }
        result *= wins;
    }

    match part {
        Part::One => println!("Part 1 answer: {result}"),
        Part::Two => println!("Part 2 answer: {result}"),
    };
}

fn main() {
    solution(&Part::One);
    solution(&Part::Two);
}
