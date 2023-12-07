use cond_utils::Between;
use std::{collections::HashMap, fs, io, num::ParseIntError, ops::RangeInclusive};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day5Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int from string")]
    ParseIntFailed(#[from] ParseIntError),
    #[error("There were too many numbers to parse into map")]
    ParsedTooManyNumbers,
    #[error("Did not parse a piece of the map")]
    MissingMapInfo,
    #[error("No first line in input")]
    NoFirstLine,
}

#[derive(Debug, Clone)]
struct Almanac {
    seed_list: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
struct Map {
    bounds: Vec<MapBounds>,
}

#[derive(Debug, Clone)]
struct MapBounds {
    source_range: RangeInclusive<i64>,
    destination_range: RangeInclusive<i64>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day5Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt".to_string())?;

    println!("creating almanacs, time elapsed: {:?}", start.elapsed());
    let almanac = create_almanac(input_data.to_owned()).unwrap();

    println!("traversing locations, time elapsed: {:?}", start.elapsed());
    let seed_locations = traverse_almanac_for_location_list(almanac);

    println!(
        "finding lowest location, time elapsed: {:?}",
        start.elapsed()
    );
    let lowest_location = find_lowest_location(seed_locations);

    println!(
        "lowest location: {:?}, time elapsed: {:?}",
        lowest_location,
        start.elapsed()
    );
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day5Error> {
    let data = fs::read_to_string(file_path).map_err(Day5Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn find_lowest_location(seed_locations: HashMap<u64, u64>) -> Option<u64> {
    seed_locations.values().min().copied()
}

fn traverse_almanac_for_location_list(almanac: Almanac) -> HashMap<u64, u64> {
    let mut seed_locations: HashMap<u64, u64> = HashMap::new();
    for seed in almanac.clone().seed_list {
        let location = traverse_almanac_for_location(seed, almanac.clone());
        seed_locations.insert(seed, location);
    }
    seed_locations
}

fn traverse_almanac_for_location(seed: u64, almanac: Almanac) -> u64 {
    let mut source = seed;

    for map in almanac.maps {
        for map_bounds in map.bounds {
            if source.within(
                *map_bounds.source_range.start() as u64,
                *map_bounds.source_range.end() as u64,
            ) {
                let source_index_in_range = source - *map_bounds.source_range.start() as u64;
                let destination =
                    *map_bounds.destination_range.start() as u64 + source_index_in_range;
                source = destination;
                break;
            }
        }
    }

    source
}

fn create_almanac(input: String) -> Result<Almanac, Day5Error> {
    let mut lines = input.lines();
    let first_line = lines.next().ok_or_else(|| Day5Error::NoFirstLine)?;
    let almanac = Almanac {
        seed_list: parse_seeds_list(first_line.to_owned())?,
        maps: create_map_list(lines.skip(1))?,
    };
    Ok(almanac)
}

fn parse_seeds_list(seed_line: String) -> Result<Vec<u64>, Day5Error> {
    seed_line[6..]
        .split_whitespace()
        .map(|num| num.parse::<u64>().map_err(Day5Error::ParseIntFailed))
        .collect()
}

fn create_map_list(
    all_maps_lines: std::iter::Skip<std::str::Lines<'_>>,
) -> Result<Vec<Map>, Day5Error> {
    let mut map_list: Vec<Map> = Vec::new();
    let mut lines_to_map = String::new();
    for line in all_maps_lines {
        match line {
            "soil-to-fertilizer map:"
            | "fertilizer-to-water map:"
            | "water-to-light map:"
            | "light-to-temperature map:"
            | "temperature-to-humidity map:"
            | "humidity-to-location map:" => {
                map_list.push(Map {
                    bounds: create_map_bounds(lines_to_map.clone())?,
                });
                lines_to_map.clear()
            }
            "" | "seed-to-soil map:" => {}
            _ => lines_to_map.push_str(&format!("{}\n", line)),
        }
    }
    map_list.push(Map {
        bounds: create_map_bounds(lines_to_map.clone())?,
    });

    Ok(map_list)
}

fn create_map_bounds(map_input: String) -> Result<Vec<MapBounds>, Day5Error> {
    let mut map_bounds: Vec<MapBounds> = Vec::new();
    for line in map_input.lines() {
        let mut destination_range_start: Option<i64> = None;
        let mut source_range_start: Option<i64> = None;
        let mut range_length: Option<i64> = None;
        for (i, num) in line.split_whitespace().enumerate() {
            let parsed_num = num.parse::<i64>()?;
            match i {
                0 => destination_range_start = Some(parsed_num),
                1 => source_range_start = Some(parsed_num),
                2 => range_length = Some(parsed_num),
                _ => return Err(Day5Error::ParsedTooManyNumbers),
            }
        }
        if let (Some(destination_range_start), Some(source_range_start), Some(range_length)) =
            (destination_range_start, source_range_start, range_length)
        {
            map_bounds.push(MapBounds {
                source_range: RangeInclusive::new(
                    source_range_start,
                    source_range_start + range_length - 1,
                ),
                destination_range: RangeInclusive::new(
                    destination_range_start,
                    destination_range_start + range_length - 1,
                ),
            });
        } else {
            return Err(Day5Error::MissingMapInfo);
        }
    }
    Ok(map_bounds)
}

#[cfg(test)]
mod tests {
    use crate::{
        create_almanac, create_map_bounds, create_map_list, find_lowest_location, load_input,
        parse_seeds_list, traverse_almanac_for_location, traverse_almanac_for_location_list,
    };

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn parse_correct_seeds_list() {
        let input = "seeds: 79 14 55 13";
        let result = parse_seeds_list(input.to_owned()).unwrap();
        check(
            &format!("{:?}", result),
            expect_test::expect!["[79, 14, 55, 13]"],
        );
    }

    #[test]
    fn create_correct_map_bounds() {
        let input = "50 98 2\n52 50 48\n";
        let result = create_map_bounds(input.to_owned()).unwrap();
        check(&format!("{:?}", result), expect_test::expect![
            "[MapBounds { source_range: 98..=99, destination_range: 50..=51 }, MapBounds { source_range: 50..=97, destination_range: 52..=99 }]"]);
    }

    #[test]
    fn parse_correct_maps_list() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let result = create_map_list(input.lines().skip(1)).unwrap();
        check(&format!("{:?}", result.len()), expect_test::expect!["7"]);
    }

    #[test]
    fn create_correct_almanac() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let result = create_almanac(input.to_owned()).unwrap();
        check(
            &format!("{:?}", result.seed_list),
            expect_test::expect!["[79, 14, 55, 13]"],
        );
        check(
            &format!("{:?}", result.maps.len()),
            expect_test::expect!["7"],
        );
    }

    #[test]
    fn find_correct_locations() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let almanac = create_almanac(input.to_owned()).unwrap();
        check(
            &format!("{:?}", traverse_almanac_for_location(79, almanac.clone())),
            expect_test::expect!["82"],
        );
        check(
            &format!("{:?}", traverse_almanac_for_location(14, almanac.clone())),
            expect_test::expect!["43"],
        );
        check(
            &format!("{:?}", traverse_almanac_for_location(55, almanac.clone())),
            expect_test::expect!["86"],
        );
        check(
            &format!("{:?}", traverse_almanac_for_location(13, almanac.clone())),
            expect_test::expect!["35"],
        );
    }

    #[test]
    fn find_correct_lowest_location() {
        let input = load_input("src/example_1.txt".to_owned()).unwrap();
        let almanac = create_almanac(input.to_owned()).unwrap();
        let seed_locations = traverse_almanac_for_location_list(almanac);
        let result = find_lowest_location(seed_locations);
        check(&format!("{:?}", result), expect_test::expect!["Some(35)"]);
    }
}
