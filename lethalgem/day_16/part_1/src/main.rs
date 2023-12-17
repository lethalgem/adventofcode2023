use std::{fs, io, num::ParseIntError, collections::BTreeMap};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day16Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int")]
    ParseIntFailed(#[from] ParseIntError),
}

pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

#[derive(Debug)]
pub enum EncounterType {
    LeftTiltMirror, // \
    RightTiltMirror, // /
    HorizontalSplitter, // -
    VerticalSplitter // |
}

#[derive(Debug)]
struct EncounterMap {
    type_: EncounterType,
    locations: BTreeMap<i32, i32>, // x, y
}

struct Beam {
    current_location: (i32, i32), // x, y
    direction: Direction,
    path_traveled: BTreeMap<i32, i32> // x, y
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day16Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt")?;

    

    // println!("finding sum, time elapsed:{:?}", start.elapsed());
    // let sum = sum_steps(steps);

    // println!("sum: {}, time elapsed: {:?}", sum, start.elapsed());

    Ok(())
}

fn load_input(file_path: &str) -> Result<String, Day16Error> {
    let data = fs::read_to_string(file_path).map_err(Day16Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn locate_all_encounters(input: &str) -> Vec<EncounterMap> {
    let mut left_tilt_mirrors: BTreeMap<i32, i32> = BTreeMap::new();
    let mut right_tilt_mirrors: BTreeMap<i32, i32> = BTreeMap::new();
    let mut vertical_splitter: BTreeMap<i32, i32> = BTreeMap::new();
    let mut horizontal_splitter: BTreeMap<i32, i32> = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        println!("{}", line);
        for (x, c) in line.chars().enumerate() {
            match c {
                '\\' => {left_tilt_mirrors.insert(x as i32, y as i32);},
                '/' => {right_tilt_mirrors.insert(x as i32, y as i32);},
                '|' => {vertical_splitter.insert(x as i32, y as i32);},
                '-' => {horizontal_splitter.insert(x as i32, y as i32);},
                _ => {}
            }
        }   
    }
    let left_tilt_encounters = EncounterMap {
        type_: EncounterType::LeftTiltMirror,
        locations: left_tilt_mirrors
    };
    let right_tilt_encounters = EncounterMap {
        type_: EncounterType::RightTiltMirror,
        locations: right_tilt_mirrors
    };
    let vertical_encounters = EncounterMap {
        type_: EncounterType::VerticalSplitter,
        locations: vertical_splitter
    };
    let horizontal_encounters = EncounterMap {
        type_: EncounterType::HorizontalSplitter,
        locations: horizontal_splitter
    };

    vec![left_tilt_encounters, right_tilt_encounters, vertical_encounters, horizontal_encounters]
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::{locate_all_encounters, load_input};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn test_encounter_locations() {
        // let input = r".|...\....";
        // let result = locate_all_encounters(input);
        // check(&format!("{:?}", result), expect!["[EncounterMap { type_: LeftTiltMirror, locations: {5: 0} }, EncounterMap { type_: RightTiltMirror, locations: {} }, EncounterMap { type_: VerticalSplitter, locations: {1: 0} }, EncounterMap { type_: HorizontalSplitter, locations: {} }]"]);  

        // let input = r".//...\....";
        // let result = locate_all_encounters(input);
        // check(&format!("{:?}", result), expect!["[EncounterMap { type_: LeftTiltMirror, locations: {6: 0} }, EncounterMap { type_: RightTiltMirror, locations: {1: 0, 2: 0} }, EncounterMap { type_: VerticalSplitter, locations: {} }, EncounterMap { type_: HorizontalSplitter, locations: {} }]"]);  


        let input = 
        r"..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let result = locate_all_encounters(input);
        check(&format!("{:?}", result), expect!["[EncounterMap { type_: LeftTiltMirror, locations: {6: 0, 7: 0, 9: 2} }, EncounterMap { type_: RightTiltMirror, locations: {2: 3, 3: 3, 4: 1} }, EncounterMap { type_: VerticalSplitter, locations: {1: 2, 5: 3, 7: 2} }, EncounterMap { type_: HorizontalSplitter, locations: {1: 1, 3: 1, 6: 2} }]"]);  

        // let input = 
        // r".|...\....
        // |.-.\.....";
        // let result = locate_all_encounters(input);
        // check(&format!("{:?}", result), expect!["[EncounterMap { type_: LeftTiltMirror, locations: {5: 0, 12: 1} }, EncounterMap { type_: RightTiltMirror, locations: {} }, EncounterMap { type_: VerticalSplitter, locations: {1: 0, 8: 1} }, EncounterMap { type_: HorizontalSplitter, locations: {10: 1} }]"]);

        // let input = load_input("src/example.txt").unwrap();
        // let result = locate_all_encounters(&input);
        // check(&format!("{:?}", result), expect!["[EncounterMap { type_: LeftTiltMirror, locations: {4: 1, 5: 0, 6: 6, 7: 6, 9: 8} }, EncounterMap { type_: RightTiltMirror, locations: {2: 9, 3: 9, 4: 7} }, EncounterMap { type_: VerticalSplitter, locations: {0: 1, 1: 8, 5: 9, 7: 8, 8: 3} }, EncounterMap { type_: HorizontalSplitter, locations: {1: 7, 2: 1, 3: 7, 6: 8} }]"]);        
    }

}
