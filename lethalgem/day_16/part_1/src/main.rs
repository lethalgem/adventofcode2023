use std::{collections::BTreeMap, fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day16Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to find a bound for x values")]
    FailedToFindXBound,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug)]
pub enum EncounterType {
    LeftTiltMirror,     // \
    RightTiltMirror,    // /
    HorizontalSplitter, // -
    VerticalSplitter,   // |
}

#[derive(Debug, Clone)]
struct Beam {
    current_location: (usize, usize), // x, y
    direction: Direction,
    has_stopped_bouncing: bool,
}

impl Beam {
    fn new(current_location: (usize, usize), direction: Direction) -> Beam {
        Beam {
            current_location,
            direction,
            has_stopped_bouncing: false,
        }
    }

    fn update_current_location(&mut self, location: (usize, usize)) {
        self.current_location = location;
    }

    fn update_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn stop_bouncing(&mut self) {
        self.has_stopped_bouncing = true;
    }
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

fn track_beam(
    encounters: BTreeMap<(usize, usize), EncounterType>,
    wall_bounds: (usize, usize),
) -> usize {
    let mut path_traveled_by_all_beams: Vec<(usize, usize)> = vec![(0, 0)];
    let mut beams = vec![Beam::new((0, 0), Direction::Right)];

    let mut beams_still_bouncing = true;
    while beams_still_bouncing {
        for (i, beam) in beams.clone().into_iter().enumerate() {
            let mut moved_beams = check_beam_location(&encounters, beam.clone(), wall_bounds);

            for moved_beam in moved_beams.clone() {
                if !path_traveled_by_all_beams.contains(&moved_beam.current_location) {
                    path_traveled_by_all_beams.push(moved_beam.current_location)
                }
            }

            beams.remove(i);
            beams.append(&mut moved_beams);
        }
        if !beams.iter().any(|beam| !beam.has_stopped_bouncing) {
            beams_still_bouncing = false;
        }
    }

    path_traveled_by_all_beams.len()
}

fn check_beam_location(
    encounters: &BTreeMap<(usize, usize), EncounterType>,
    mut beam: Beam,
    wall_bounds: (usize, usize),
) -> Vec<Beam> {
    let current_location_encounter = encounters.get(&beam.current_location);
    match current_location_encounter {
        Some(EncounterType::RightTiltMirror) => match beam.direction {
            Direction::Up => {
                beam.update_direction(Direction::Right);
                move_beam(&mut beam, wall_bounds);
            }
            Direction::Right => {
                beam.update_direction(Direction::Up);
                move_beam(&mut beam, wall_bounds);
            }
            Direction::Left => {
                beam.update_direction(Direction::Down);
                move_beam(&mut beam, wall_bounds);
            }
            Direction::Down => {
                beam.update_direction(Direction::Left);
                move_beam(&mut beam, wall_bounds);
            }
        },
        Some(EncounterType::LeftTiltMirror) => match beam.direction {
            Direction::Up => {
                beam.update_direction(Direction::Left);
                move_beam(&mut beam, wall_bounds);
            }
            Direction::Right => {
                beam.update_direction(Direction::Down);
                move_beam(&mut beam, wall_bounds);
            }
            Direction::Left => {
                beam.update_direction(Direction::Up);
                move_beam(&mut beam, wall_bounds);
            }
            Direction::Down => {
                beam.update_direction(Direction::Right);
                move_beam(&mut beam, wall_bounds);
            }
        },
        Some(EncounterType::VerticalSplitter) => println!(""),
        Some(EncounterType::HorizontalSplitter) => println!(""),
        _ => move_beam(&mut beam, wall_bounds),
    }
    vec![beam]
}

fn move_beam(beam: &mut Beam, wall_bounds: (usize, usize)) {
    fn update_beam_to_new_location(
        new_location: (i32, i32),
        wall_bounds: (usize, usize),
        beam: &mut Beam,
    ) {
        if new_location.0 > wall_bounds.0 as i32
            || new_location.0 < 0
            || new_location.1 > wall_bounds.1 as i32
            || new_location.1 < 0
        {
            beam.stop_bouncing()
        } else {
            beam.update_current_location((new_location.0 as usize, new_location.1 as usize))
        }
    }

    match beam.direction {
        Direction::Up => {
            let new_location = (
                beam.current_location.0 as i32,
                beam.current_location.1 as i32 - 1,
            );
            update_beam_to_new_location(new_location, wall_bounds, beam);
        }
        Direction::Right => {
            let new_location = (
                beam.current_location.0 as i32 + 1,
                beam.current_location.1 as i32,
            );
            update_beam_to_new_location(new_location, wall_bounds, beam);
        }
        Direction::Left => {
            let new_location = (
                beam.current_location.0 as i32 - 1,
                beam.current_location.1 as i32,
            );
            update_beam_to_new_location(new_location, wall_bounds, beam);
        }
        Direction::Down => {
            let new_location = (
                beam.current_location.0 as i32,
                beam.current_location.1 as i32 + 1,
            );
            update_beam_to_new_location(new_location, wall_bounds, beam);
        }
    }
}

fn find_wall_bounds(input: &str) -> Result<(usize, usize), Day16Error> {
    let y = input.lines().count() - 1;
    let x = input
        .lines()
        .next()
        .ok_or_else(|| Day16Error::FailedToFindXBound)?
        .chars()
        .count()
        - 1;

    Ok((x, y))
}

fn locate_all_encounters(input: &str) -> BTreeMap<(usize, usize), EncounterType> {
    let mut encounters: BTreeMap<(usize, usize), EncounterType> = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        println!("{}", line);
        for (x, c) in line.chars().enumerate() {
            match c {
                '\\' => {
                    encounters.insert((x, y), EncounterType::LeftTiltMirror);
                }
                '/' => {
                    encounters.insert((x, y), EncounterType::RightTiltMirror);
                }
                '|' => {
                    encounters.insert((x, y), EncounterType::VerticalSplitter);
                }
                '-' => {
                    encounters.insert((x, y), EncounterType::HorizontalSplitter);
                }
                _ => {}
            }
        }
    }

    encounters
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::{find_wall_bounds, load_input, locate_all_encounters, track_beam};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn test_encounter_locations() {
        let input = r".|...\....";
        let result = locate_all_encounters(input);
        check(
            &format!("{:?}", result),
            expect!["{(1, 0): VerticalSplitter, (5, 0): LeftTiltMirror}"],
        );

        let input = r".//...\....";
        let result = locate_all_encounters(input);
        check(
            &format!("{:?}", result),
            expect!["{(1, 0): RightTiltMirror, (2, 0): RightTiltMirror, (6, 0): LeftTiltMirror}"],
        );

        let input = r"..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";
        let result = locate_all_encounters(input);
        check(&format!("{:?}", result), expect!["{(4, 0): RightTiltMirror, (6, 0): LeftTiltMirror, (7, 0): LeftTiltMirror, (9, 1): HorizontalSplitter, (9, 2): VerticalSplitter, (10, 3): RightTiltMirror, (11, 1): HorizontalSplitter, (11, 3): RightTiltMirror, (12, 1): RightTiltMirror, (13, 3): VerticalSplitter, (14, 2): HorizontalSplitter, (15, 1): VerticalSplitter, (15, 2): VerticalSplitter, (17, 2): LeftTiltMirror}"]);

        let input = r".|...\....
        |.-.\.....";
        let result = locate_all_encounters(input);
        check(&format!("{:?}", result), expect!["{(1, 0): VerticalSplitter, (5, 0): LeftTiltMirror, (8, 1): VerticalSplitter, (10, 1): HorizontalSplitter, (12, 1): LeftTiltMirror}"]);

        let input = load_input("src/example.txt").unwrap();
        let result = locate_all_encounters(&input);
        check(&format!("{:?}", result), expect!["{(0, 1): VerticalSplitter, (1, 0): VerticalSplitter, (1, 7): HorizontalSplitter, (1, 8): VerticalSplitter, (2, 1): HorizontalSplitter, (2, 9): RightTiltMirror, (3, 7): HorizontalSplitter, (3, 9): RightTiltMirror, (4, 1): LeftTiltMirror, (4, 6): RightTiltMirror, (4, 7): RightTiltMirror, (5, 0): LeftTiltMirror, (5, 2): VerticalSplitter, (5, 9): VerticalSplitter, (6, 2): HorizontalSplitter, (6, 6): LeftTiltMirror, (6, 8): HorizontalSplitter, (7, 6): LeftTiltMirror, (7, 7): VerticalSplitter, (7, 8): VerticalSplitter, (8, 3): VerticalSplitter, (9, 5): LeftTiltMirror, (9, 8): LeftTiltMirror}"]);
    }

    #[test]
    fn test_wall_bounds() {
        let input = r"..........";
        let result = find_wall_bounds(input).unwrap();
        check(&format!("{:?}", result), expect!["(9, 0)"]);

        let input = r"..........
        ..........
        ..........
        ..........";
        let result = find_wall_bounds(input).unwrap();
        check(&format!("{:?}", result), expect!["(9, 3)"]);

        let input = load_input("src/example.txt").unwrap();
        let result = find_wall_bounds(&input).unwrap();
        check(&format!("{:?}", result), expect!["(9, 9)"]);
    }

    #[test]
    fn test_beam_tracking() {
        let input = r"..........";
        let encounters = locate_all_encounters(input);
        let wall_bounds = find_wall_bounds(input).unwrap();
        let result = track_beam(encounters, wall_bounds);
        check(&format!("{:?}", result), expect!["10"]);

        let input = r"..../.....";
        let encounters = locate_all_encounters(input);
        let wall_bounds = find_wall_bounds(input).unwrap();
        let result = track_beam(encounters, wall_bounds);
        check(&format!("{:?}", result), expect!["5"]);

        let input = r".......\..";
        let encounters = locate_all_encounters(input);
        let wall_bounds = find_wall_bounds(input).unwrap();
        let result = track_beam(encounters, wall_bounds);
        check(&format!("{:?}", result), expect!["8"]);

        let input = r"..\..../..
        ..........
        ..\..../..
        ..........";
        let encounters = locate_all_encounters(input);
        let wall_bounds = find_wall_bounds(input).unwrap();
        let result = track_beam(encounters, wall_bounds);
        check(&format!("{:?}", result), expect!["14"]);
    }
}
