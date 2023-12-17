use std::{collections::HashMap, fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day15Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int")]
    ParseIntFailed(#[from] ParseIntError),
}

#[derive(Debug, Clone)]
struct Step {
    lens_box: u32,
    label: String,
    focal_length: Option<u32>,
    hash_result: u32,
}

impl Step {
    fn new(string: &str) -> Result<Step, Day15Error> {
        let label = Self::get_label(string);
        Ok(Step {
            lens_box: Self::calculate_hash(&label),
            label,
            focal_length: Self::get_focal_length(string),
            hash_result: Self::calculate_hash(string),
        })
    }

    fn get_label(string: &str) -> String {
        let mut label: String = String::new();
        for c in string.chars() {
            if c != '-' && c != '=' {
                label.push(c)
            } else {
                break;
            }
        }
        label
    }

    fn get_focal_length(string: &str) -> Option<u32> {
        match string.chars().last() {
            Some(char) => {
                if char == '-' {
                    None
                } else {
                    char.to_digit(10)
                }
            }
            None => None,
        }
    }

    fn calculate_hash(string: &str) -> u32 {
        let mut current_hash_value = 0;
        for c in string.chars() {
            current_hash_value += c as u32;
            current_hash_value *= 17;
            current_hash_value %= 256;
        }
        current_hash_value
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day15Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt")?;

    println!(
        "finding hashes for steps, time elapsed:{:?}",
        start.elapsed()
    );
    let steps = extract_steps(&input_data)?;

    println!("sorting lenses, time elapsed:{:?}", start.elapsed());
    let boxes = sort_lenses(steps);

    println!("finding focusing power, time elapsed:{:?}", start.elapsed());
    let sum = calc_focusing_power(boxes);

    println!(
        "focusing power: {}, time elapsed: {:?}",
        sum,
        start.elapsed()
    );

    Ok(())
}

fn load_input(file_path: &str) -> Result<String, Day15Error> {
    let data = fs::read_to_string(file_path).map_err(Day15Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn calc_focusing_power(boxes: HashMap<u32, Vec<Step>>) -> u32 {
    let mut total_focusing_power = 0;
    for box_ in boxes.iter() {
        for (i, lens) in box_.1.iter().enumerate() {
            if let Some(focal_length) = lens.focal_length {
                total_focusing_power += (lens.lens_box + 1) * (i as u32 + 1) * focal_length;
            }
        }
    }
    total_focusing_power
}

fn sort_lenses(steps: Vec<Step>) -> HashMap<u32, Vec<Step>> {
    let mut boxes: HashMap<u32, Vec<Step>> = HashMap::new();

    for step in steps {
        if step.focal_length.is_some() {
            if let Some(lenses_in_box) = boxes.clone().get(&step.lens_box) {
                replace_or_insert_lens(lenses_in_box, &step, &mut boxes);
            } else {
                boxes.insert(step.lens_box, vec![step]);
            }
        } else if let Some(lenses_in_box) = boxes.clone().get(&step.lens_box) {
            remove_lens(lenses_in_box, step, &mut boxes);
        }
    }

    boxes
}

fn replace_or_insert_lens(
    lenses_in_box: &Vec<Step>,
    step: &Step,
    boxes: &mut HashMap<u32, Vec<Step>>,
) {
    let mut updated_lenses_in_box = lenses_in_box.clone();

    if let Some(existing_lens_index) = updated_lenses_in_box
        .iter()
        .position(|lens| lens.label == step.label)
    {
        updated_lenses_in_box.remove(existing_lens_index);
        updated_lenses_in_box.insert(existing_lens_index, step.clone())
    } else {
        updated_lenses_in_box.push(step.clone());
    }

    boxes.insert(step.lens_box, updated_lenses_in_box.to_vec());
}

fn remove_lens(lenses_in_box: &Vec<Step>, step: Step, boxes: &mut HashMap<u32, Vec<Step>>) {
    let mut updated_lenses_in_box = lenses_in_box.clone();
    if let Some(lens_index_to_remove) = updated_lenses_in_box
        .iter()
        .position(|lens| lens.label == step.label)
    {
        updated_lenses_in_box.remove(lens_index_to_remove);
    }
    boxes.insert(step.lens_box, updated_lenses_in_box.to_vec());
}

fn extract_steps(input: &str) -> Result<Vec<Step>, Day15Error> {
    input
        .split(',')
        .map(Step::new)
        .collect::<Result<Vec<Step>, Day15Error>>()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use expect_test::expect;

    use crate::{calc_focusing_power, extract_steps, load_input, sort_lenses, Step};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn test_hashing() {
        let input = "H";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["200"]);

        let input = "HA";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["153"]);

        let input = "HAS";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["172"]);

        let input = "HASH";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["52"]);

        let input = "rn=1";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["30"]);

        let input = "rn";
        let result = Step::calculate_hash(input);
        check(&format!("{}", result), expect!["0"]);
    }

    #[test]
    fn test_extraction() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = extract_steps(input);
        check(
            &format!("{:?}", result),
            expect![[
                r#"Ok([Step { lens_box: 0, label: "rn", focal_length: Some(1), hash_result: 30 }, Step { lens_box: 0, label: "cm", focal_length: None, hash_result: 253 }, Step { lens_box: 1, label: "qp", focal_length: Some(3), hash_result: 97 }, Step { lens_box: 0, label: "cm", focal_length: Some(2), hash_result: 47 }, Step { lens_box: 1, label: "qp", focal_length: None, hash_result: 14 }, Step { lens_box: 3, label: "pc", focal_length: Some(4), hash_result: 180 }, Step { lens_box: 3, label: "ot", focal_length: Some(9), hash_result: 9 }, Step { lens_box: 3, label: "ab", focal_length: Some(5), hash_result: 197 }, Step { lens_box: 3, label: "pc", focal_length: None, hash_result: 48 }, Step { lens_box: 3, label: "pc", focal_length: Some(6), hash_result: 214 }, Step { lens_box: 3, label: "ot", focal_length: Some(7), hash_result: 231 }])"#
            ]],
        );
    }

    #[test]
    fn test_sorting() {
        let input = "rn=1,cm-";
        let steps = extract_steps(input).unwrap();
        let boxes = sort_lenses(steps);
        let mut result: BTreeMap<u32, Vec<String>> = BTreeMap::new();
        for box_ in boxes.iter() {
            let mut labels: Vec<String> = Vec::new();
            for step in box_.1 {
                labels.push(step.clone().label)
            }
            result.insert(*box_.0, labels);
        }
        check(&format!("{:?}", result), expect![r#"{0: ["rn"]}"#]);

        let input = "rn=1,cm-,qp=3,cm=2,qp-,";
        let steps = extract_steps(input).unwrap();
        let boxes = sort_lenses(steps);
        let mut result: BTreeMap<u32, Vec<String>> = BTreeMap::new();
        for box_ in boxes.iter() {
            let mut labels: Vec<String> = Vec::new();
            for step in box_.1 {
                labels.push(step.clone().label)
            }
            result.insert(*box_.0, labels);
        }
        check(
            &format!("{:?}", result),
            expect![r#"{0: ["rn", "cm"], 1: []}"#],
        );

        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let steps = extract_steps(input).unwrap();
        let boxes = sort_lenses(steps);
        let mut result: BTreeMap<u32, Vec<String>> = BTreeMap::new();
        for box_ in boxes.iter() {
            let mut labels: Vec<String> = Vec::new();
            for step in box_.1 {
                labels.push(step.clone().label)
            }
            result.insert(*box_.0, labels);
        }
        check(
            &format!("{:?}", result),
            expect![r#"{0: ["rn", "cm"], 1: [], 3: ["ot", "ab", "pc"]}"#],
        );
    }

    #[test]
    fn test_focusing_power() {
        let input = load_input("src/example.txt").unwrap();
        let steps = extract_steps(&input).unwrap();
        let boxes = sort_lenses(steps);
        let result = calc_focusing_power(boxes);
        check(&format!("{:?}", result), expect!["145"]);
    }
}
