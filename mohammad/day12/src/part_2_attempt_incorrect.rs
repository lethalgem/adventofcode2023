use itertools::{iproduct, Itertools};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
enum Type {
    A, // plain
    S, // with suffix
    P, // with prefix
    B, // with both
}
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn is_valid(candidate: &[char], amounts: &Vec<usize>) -> bool {
    &candidate
        .iter()
        .collect::<String>()
        .split('.')
        .filter_map(|s| if s.is_empty() { None } else { Some(s.len()) })
        .collect::<Vec<_>>()
        == amounts
}

fn compute_arrangements(record: &[char], amounts: &Vec<usize>) -> i64 {
    let mut unknowns = Vec::new();
    let mut damaged_count = 0;
    for (idx, c) in record.iter().enumerate() {
        if *c == '?' {
            unknowns.push(idx);
        } else if *c == '#' {
            damaged_count += 1;
        }
    }

    let amounts_sum = amounts.iter().sum::<usize>();
    if amounts_sum < damaged_count {
        return 0;
    }

    let combinations = unknowns
        .iter()
        .combinations(amounts_sum - damaged_count)
        .collect::<Vec<_>>();

    let mut arrangements: i64 = 0;
    for c in combinations {
        let mut candidate = record.to_owned();
        for i in 0..c.len() {
            candidate[*c[i]] = '#';
        }
        for r in &mut candidate {
            if *r == '?' {
                *r = '.';
            }
        }
        if is_valid(&candidate, amounts) {
            arrangements += 1;
        }
    }
    arrangements
}

fn compute_count(list: &[Type], a: i64, s: i64, p: i64, b: i64) -> i64 {
    let result = list.iter().fold(1, |acc, d| {
        acc * match d {
            Type::A => a,
            Type::S => s,
            Type::P => p,
            Type::B => b,
        }
    });
    result
}

fn solution() {
    use Direction::{Left, Right};
    use Type::{A, B, P, S};
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut part_1_result = 0;
    let mut part_2_result = 0;

    for line in BufReader::new(handle).lines() {
        let line = line.unwrap();
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let mut record = split[0].chars().collect::<Vec<_>>();
        let amounts = split[1]
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let a = compute_arrangements(&record, &amounts);
        part_1_result += a;

        // part 2 compuations:
        match record.last().unwrap() {
            '#' => part_2_result += a.pow(5),
            '.' => {
                record.insert(0, '?');
                part_2_result += a * compute_arrangements(&record, &amounts).pow(4);
            }
            '?' => {
                // suffix `#`
                record.push('#');
                let s = compute_arrangements(&record, &amounts);

                // prefix `#`
                record.pop();
                record.insert(0, '#');
                let p = compute_arrangements(&record, &amounts);

                // both a prefix and a suffix
                record.push('#');
                let b = compute_arrangements(&record, &amounts);
                let power = (0..4).powerset().collect::<Vec<_>>();

                let update_list = |list: &mut [Type; 5], dir: &Direction, pow_idx: usize| match dir
                {
                    Left => match list[pow_idx] {
                        A => list[pow_idx] = S,
                        P => list[pow_idx] = B,
                        S | B => unreachable!(),
                    },
                    Right => match list[pow_idx + 1] {
                        A => list[pow_idx + 1] = P,
                        S => list[pow_idx + 1] = B,
                        P | B => unreachable!(),
                    },
                };
                for pow in power {
                    let d = [Left, Right].iter();
                    match pow.len() {
                        0 => part_2_result += a * a * a * a * a,
                        1 => {
                            for dir in &iproduct!(d).collect::<Vec<_>>() {
                                let mut list = [A, A, A, A, A];
                                update_list(&mut list, dir, pow[0]);
                                part_2_result += compute_count(&list, a, s, p, b);
                            }
                        }
                        2 => {
                            for dir in &iproduct!(d.clone(), d).collect::<Vec<_>>() {
                                let mut list = [A, A, A, A, A];
                                update_list(&mut list, dir.0, pow[0]);
                                update_list(&mut list, dir.1, pow[1]);
                                part_2_result += compute_count(&list, a, s, p, b);
                            }
                        }
                        3 => {
                            for dir in &iproduct!(d.clone(), d.clone(), d).collect::<Vec<_>>() {
                                let mut list = [A, A, A, A, A];
                                update_list(&mut list, dir.0, pow[0]);
                                update_list(&mut list, dir.1, pow[1]);
                                update_list(&mut list, dir.2, pow[2]);
                                part_2_result += compute_count(&list, a, s, p, b);
                            }
                        }
                        4 => {
                            for dir in
                                &iproduct!(d.clone(), d.clone(), d.clone(), d).collect::<Vec<_>>()
                            {
                                let mut list = [A, A, A, A, A];
                                update_list(&mut list, dir.0, pow[0]);
                                update_list(&mut list, dir.1, pow[1]);
                                update_list(&mut list, dir.2, pow[2]);
                                update_list(&mut list, dir.3, pow[3]);
                                part_2_result += compute_count(&list, a, s, p, b);
                            }
                        }
                        _ => unreachable!(),
                    };
                }
            }
            _ => {}
        }
    }

    println!("part 1 answer: {part_1_result}");
    println!("part 2 answer: {part_2_result}");
}

fn main() {
    solution();
}
