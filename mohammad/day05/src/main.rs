use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

enum Part {
    One,
    Two,
}

// Gets a single element from a map of ranges
fn get_from_map(map: &BTreeMap<(u64, u64), (u64, u64)>, attribute: u64) -> u64 {
    let in_range = |val, range: (u64, u64)| val >= range.0 && val <= range.1;
    for (key, value) in map {
        // If within range, then do some offsetting
        if in_range(attribute, *key) && in_range(attribute, *key) {
            return value.0 + (attribute - key.0);
        }
    }
    attribute
}

// Gets a vector of ranges corresponding to a input vector of ranges and a map of ranges
fn get_ranges_from_map(
    map: &BTreeMap<(u64, u64), (u64, u64)>,
    attributes: &Vec<(u64, u64)>,
) -> Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = Vec::new();
    for attribute in attributes {
        result.extend(get_range_from_map(map, *attribute));
    }

    result
}

// Gets a vector of ranges corresponding to an input range and a map of ranges
fn get_range_from_map(
    map: &BTreeMap<(u64, u64), (u64, u64)>,
    attribute: (u64, u64),
) -> Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = Vec::new();
    let in_range = |val, range: (u64, u64)| val >= range.0 && val <= range.1;
    let mut remaining: Vec<(u64, u64)> = Vec::new();
    remaining.push(attribute);
    let mut found;
    while let Some(attribute) = remaining.pop() {
        found = false;
        for (key, value) in map {
            if in_range(attribute.0, *key) && in_range(attribute.1, *key) {
                result.push((
                    value.0 + (attribute.0 - key.0),
                    value.0 + (attribute.1 - key.0),
                ));
                found = true;
            } else if in_range(attribute.0, *key) && !in_range(attribute.1, *key) {
                result.push((value.0 + (attribute.0 - key.0), value.1));
                remaining.push((key.1 + 1, attribute.1));
                found = true;
            } else if !in_range(attribute.0, *key) && in_range(attribute.1, *key) {
                result.push((value.0, value.0 + attribute.1 - key.0));
                remaining.push((attribute.0, key.0 - 1));
                found = true;
            } else if !in_range(attribute.0, *key)
                && !in_range(attribute.1, *key)
                && attribute.0 <= key.0
                && attribute.1 >= key.1
            {
                result.push((value.0, value.1));
                remaining.push((attribute.0, key.0 - 1));
                remaining.push((key.1 + 1, attribute.1));
                found = true;
            }
        }

        if !found {
            result.push((attribute.0, attribute.1));
        }
    }

    result
}

fn solution(part: &Part) {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut seeds = Vec::new();
    let mut maps: Vec<BTreeMap<(u64, u64), (u64, u64)>> = Vec::new();
    let mut current = 0;

    for (idx, line) in BufReader::new(handle).lines().enumerate() {
        let line = line.unwrap();
        // Get the seeds
        if idx == 0 {
            let split: Vec<_> = line.split(':').collect();
            seeds = split[1]
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
        }

        // Get all the maps, encoded as ranges
        match line.trim() {
            "seed-to-soil map:" => {
                maps.push(BTreeMap::new());
                current = 0;
            }
            "soil-to-fertilizer map:" => {
                maps.push(BTreeMap::new());
                current = 1;
            }
            "fertilizer-to-water map:" => {
                maps.push(BTreeMap::new());
                current = 2;
            }
            "water-to-light map:" => {
                maps.push(BTreeMap::new());
                current = 3;
            }
            "light-to-temperature map:" => {
                maps.push(BTreeMap::new());
                current = 4;
            }
            "temperature-to-humidity map:" => {
                maps.push(BTreeMap::new());
                current = 5;
            }
            "humidity-to-location map:" => {
                maps.push(BTreeMap::new());
                current = 6;
            }
            _ if !maps.is_empty() => {
                let v: Vec<_> = line
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<u64>().unwrap_or(0))
                    .collect();
                if v.len() == 3 {
                    maps[current].insert((v[1], v[1] + v[2] - 1), (v[0], v[0] + v[2] - 1));
                }
            }
            _ => {}
        }
    }

    // Walk the maps
    match part {
        Part::One => {
            let mut result = u64::MAX;
            for seed in &seeds {
                let mut attribute = *seed;
                for map in &maps {
                    attribute = get_from_map(map, attribute);
                }
                result = std::cmp::min(result, attribute);
            }
            println!("Part 1 answer: {result}");
        }

        // Walk the maps
        Part::Two => {
            let mut result = u64::MAX;
            for i in 0..seeds.len() / 2 {
                let mut attributes = vec![(seeds[2 * i], seeds[2 * i] + seeds[2 * i + 1])];
                for map in &maps {
                    attributes = get_ranges_from_map(map, &attributes);
                }
                for attribute in attributes {
                    result = std::cmp::min(result, attribute.0);
                }
            }
            println!("Part 2 answer: {result}");
        }
    }
}

fn main() {
    solution(&Part::One);
    solution(&Part::Two);
}
