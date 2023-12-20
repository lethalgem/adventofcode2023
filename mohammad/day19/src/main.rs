use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Eq, Hash, PartialEq)]
enum Category {
    Cx,
    Cm,
    Ca,
    Cs,
}

#[derive(Debug)]
enum Rule {
    LessThan {
        lhs: Category,
        rhs: usize,
        next: String,
    },
    GreaterThan {
        lhs: Category,
        rhs: usize,
        next: String,
    },
    Always(String),
}

fn part1() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut lines = BufReader::new(handle).lines();
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let split = line
            .split(['{', ',', '}'])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        workflows.insert(
            split[0].to_string(),
            split[1..]
                .iter()
                .map(|r| {
                    let lt = r.split(['<', ':']).collect::<Vec<_>>();
                    let gt = r.split(['>', ':']).collect::<Vec<_>>();
                    if lt.len() == 3 {
                        Rule::LessThan {
                            lhs: match lt[0] {
                                "x" => Category::Cx,
                                "m" => Category::Cm,
                                "a" => Category::Ca,
                                "s" => Category::Cs,
                                _ => unreachable!(),
                            },
                            rhs: lt[1].parse::<usize>().unwrap(),
                            next: lt[2].to_string(),
                        }
                    } else if gt.len() == 3 {
                        Rule::GreaterThan {
                            lhs: match gt[0] {
                                "x" => Category::Cx,
                                "m" => Category::Cm,
                                "a" => Category::Ca,
                                "s" => Category::Cs,
                                _ => unreachable!(),
                            },
                            rhs: gt[1].parse::<usize>().unwrap(),
                            next: gt[2].to_string(),
                        }
                    } else {
                        Rule::Always((*r).to_string())
                    }
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut part_1_result = 0;
    for line in lines {
        let line = line.unwrap();
        let mut parts = HashMap::new();
        line.split(['{', ',', '}'])
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                let split = s.split('=').collect::<Vec<_>>();
                match split[0] {
                    "x" => parts.insert(Category::Cx, split[1].parse::<usize>().unwrap()),
                    "m" => parts.insert(Category::Cm, split[1].parse::<usize>().unwrap()),
                    "a" => parts.insert(Category::Ca, split[1].parse::<usize>().unwrap()),
                    "s" => parts.insert(Category::Cs, split[1].parse::<usize>().unwrap()),
                    _ => unreachable!(),
                };
            });

        let mut current = "in".to_string();
        loop {
            if current == "A" {
                part_1_result += parts.values().sum::<usize>();
                break;
            } else if current == "R" {
                break;
            }
            for r in &workflows[&current] {
                match r {
                    Rule::GreaterThan { lhs, rhs, next } => {
                        if parts[&lhs] > *rhs {
                            current = next.to_string();
                            break;
                        }
                    }
                    Rule::LessThan { lhs, rhs, next } => {
                        if parts[&lhs] < *rhs {
                            current = next.to_string();
                            break;
                        }
                    }
                    Rule::Always(next) => {
                        current = next.to_string();
                        break;
                    }
                }
            }
        }
    }

    println!("Part 1 answer: {part_1_result}");
}

fn main() {
    part1();
}
