use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn part1() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut total_sum = 0;
    for line in BufReader::new(handle).lines() {
        let digits = line
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<_>>();
        total_sum += 10 * digits.first().unwrap().to_digit(10).unwrap()
            + digits.last().unwrap().to_digit(10).unwrap();
    }

    println!("part 1 total sum = {total_sum}");
}

fn replace_first(line: &String) -> String {
    let indices = [
        line.find("one").unwrap_or(line.len()),
        line.find("two").unwrap_or(line.len()),
        line.find("three").unwrap_or(line.len()),
        line.find("four").unwrap_or(line.len()),
        line.find("five").unwrap_or(line.len()),
        line.find("six").unwrap_or(line.len()),
        line.find("seven").unwrap_or(line.len()),
        line.find("eight").unwrap_or(line.len()),
        line.find("nine").unwrap_or(line.len()),
    ];
    let (mut min, mut min_index) = (indices[0], 0);
    for (i, index) in indices.iter().enumerate() {
        if index <= &min {
            (min, min_index) = (*index, i);
        }
    }

    if min < line.len() {
        match min_index {
            0 => line.replace("one", "1"),
            1 => line.replace("two", "2"),
            2 => line.replace("three", "3"),
            3 => line.replace("four", "4"),
            4 => line.replace("five", "5"),
            5 => line.replace("six", "6"),
            6 => line.replace("seven", "7"),
            7 => line.replace("eight", "8"),
            8 => line.replace("nine", "9"),
            _ => "".to_string(),
        }
    } else {
        "".to_string()
    }
}

fn replace_last(line: &String) -> String {
    let indices = [
        line.rfind("one").unwrap_or(0),
        line.rfind("two").unwrap_or(0),
        line.rfind("three").unwrap_or(0),
        line.rfind("four").unwrap_or(0),
        line.rfind("five").unwrap_or(0),
        line.rfind("six").unwrap_or(0),
        line.rfind("seven").unwrap_or(0),
        line.rfind("eight").unwrap_or(0),
        line.rfind("nine").unwrap_or(0),
    ];
    let (mut max, mut max_index) = (indices[0], 0);
    for (i, index) in indices.iter().enumerate() {
        if index >= &max && index < &line.len() {
            (max, max_index) = (*index, i);
        }
    }

    if max < line.len() {
        match max_index {
            0 => line.replace("one", "1"),
            1 => line.replace("two", "2"),
            2 => line.replace("three", "3"),
            3 => line.replace("four", "4"),
            4 => line.replace("five", "5"),
            5 => line.replace("six", "6"),
            6 => line.replace("seven", "7"),
            7 => line.replace("eight", "8"),
            8 => line.replace("nine", "9"),
            _ => "".to_string(),
        }
    } else {
        "".to_string()
    }
}

fn part2() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut total_sum = 0;
    for line in BufReader::new(handle).lines() {
        let mut line = line.unwrap().clone();
        let replaced1 = replace_first(&line);
        let replaced2 = replace_last(&line);
        line = replaced1 + &replaced2;
        let digits = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<_>>();
        let new = 10 * digits.first().unwrap().to_digit(10).unwrap()
            + digits.last().unwrap().to_digit(10).unwrap();
        total_sum += new;
    }

    println!("part 2 total sum = {total_sum}");
}

fn main() {
    part1();
    part2();
}
