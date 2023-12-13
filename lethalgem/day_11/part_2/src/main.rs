use std::{collections::HashMap, fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day9Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int")]
    ParseIntFailed(#[from] ParseIntError),
    #[error("Failed to find corresponding x in map")]
    MissingXMapCoord,
    #[error("Failed to find corresponding y in map")]
    MissingYMapCoord,
    #[error("Failed to find next galaxy for pairing")]
    MissingNextGalaxy,
}

#[derive(Debug, Clone)]
struct GalaxyPair {
    start: Galaxy,
    end: Galaxy,
    shortest_distance: usize,
}

impl GalaxyPair {
    fn new(start: Galaxy, end: Galaxy) -> GalaxyPair {
        GalaxyPair {
            start: start.clone(),
            end: end.clone(),
            shortest_distance: Self::calculate_shortest_distance(start.location, end.location),
        }
    }

    fn calculate_shortest_distance(start_location: Location, end_location: Location) -> usize {
        let x_steps: i64 = start_location.x as i64 - end_location.x as i64;
        let y_steps: i64 = start_location.y as i64 - end_location.y as i64;
        (x_steps.abs() + y_steps.abs()) as usize
    }
}

#[derive(Debug, Clone)]
struct Galaxy {
    id: usize,
    location: Location,
}

impl Galaxy {
    fn update_location(&mut self, location: Location) {
        self.location = location;
    }
}

#[derive(Debug, Clone)]
struct Location {
    x: usize,
    y: usize,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day9Error> {
    let start = std::time::Instant::now();

    let input_data = load_input("src/input.txt".to_string())?;

    println!("finding galaxies, time elapsed:{:?}", start.elapsed());
    let galaxies = find_galaxies(input_data);

    println!("expanding galaxies, time elapsed:{:?}", start.elapsed());
    let expanded_galaxies = expand_universe(galaxies).unwrap();

    println!("creating galaxy pairs, time elapsed:{:?}", start.elapsed());
    let pairs = create_pairs(expanded_galaxies).unwrap();

    println!("finding sum, time elapsed:{:?}", start.elapsed());
    let sum = sum_shortest_distances(pairs);

    println!("sum: {}, time elapsed: {:?}", sum, start.elapsed());

    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day9Error> {
    let data = fs::read_to_string(file_path).map_err(Day9Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn sum_shortest_distances(pairs: Vec<GalaxyPair>) -> i64 {
    pairs.iter().map(|pair| pair.shortest_distance as i64).sum()
}

fn create_pairs(galaxies: Vec<Galaxy>) -> Result<Vec<GalaxyPair>, Day9Error> {
    let mut galaxy_pairs: Vec<GalaxyPair> = Vec::new();
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            galaxy_pairs.push(GalaxyPair::new(
                galaxy.clone(),
                galaxies
                    .get(j)
                    .ok_or_else(|| Day9Error::MissingNextGalaxy)?
                    .clone(),
            ))
        }
    }
    Ok(galaxy_pairs)
}

fn find_galaxies(universe: String) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, line) in universe.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(Galaxy {
                    id: galaxies.len() + 1,
                    location: Location { x, y },
                })
            }
        }
    }

    galaxies
}

fn expand_universe(galaxies: Vec<Galaxy>) -> Result<Vec<Galaxy>, Day9Error> {
    let mut rows_with_galaxies: Vec<usize> =
        galaxies.iter().map(|galaxy| (galaxy.location.y)).collect();
    rows_with_galaxies.dedup();

    let mut cols_with_galaxies: Vec<usize> =
        galaxies.iter().map(|galaxy| (galaxy.location.x)).collect();
    cols_with_galaxies.sort();
    cols_with_galaxies.dedup();

    let mut expanded_rows_map: HashMap<usize, usize> = HashMap::new();
    build_map(rows_with_galaxies, &mut expanded_rows_map);

    let mut expanded_cols_map: HashMap<usize, usize> = HashMap::new();
    build_map(cols_with_galaxies, &mut expanded_cols_map);

    let mut expanded_universe_galaxies: Vec<Galaxy> = Vec::new();
    for galaxy in galaxies {
        expanded_universe_galaxies.push(Galaxy {
            id: galaxy.id,
            location: Location {
                x: *expanded_cols_map
                    .get(&galaxy.location.x)
                    .ok_or_else(|| Day9Error::MissingXMapCoord)?,
                y: *expanded_rows_map
                    .get(&galaxy.location.y)
                    .ok_or_else(|| Day9Error::MissingYMapCoord)?,
            },
        })
    }

    Ok(expanded_universe_galaxies)
}

fn build_map(coord_with_galaxies: Vec<usize>, expanded_rows_map: &mut HashMap<usize, usize>) {
    let mut expansion_count: usize = 0;
    (0_usize..=*coord_with_galaxies.last().unwrap_or(&0)).for_each(|i| {
        if !coord_with_galaxies.contains(&i) {
            expansion_count += 999999;
        } else {
            expanded_rows_map.insert(i, i + expansion_count);
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::{create_pairs, expand_universe, find_galaxies, load_input, sum_shortest_distances};

    fn check(actual: &str, expect: expect_test::Expect) {
        expect.assert_eq(actual);
    }

    #[test]
    fn find_correct_galaxy_locations() {
        let input = load_input("src/initial_universe_example.txt".to_owned()).unwrap();
        let result = find_galaxies(input);
        check(&format!("{:?}", result), expect_test::expect!["[Galaxy { id: 1, location: Location { x: 3, y: 0 } }, Galaxy { id: 2, location: Location { x: 7, y: 1 } }, Galaxy { id: 3, location: Location { x: 0, y: 2 } }, Galaxy { id: 4, location: Location { x: 6, y: 4 } }, Galaxy { id: 5, location: Location { x: 1, y: 5 } }, Galaxy { id: 6, location: Location { x: 9, y: 6 } }, Galaxy { id: 7, location: Location { x: 7, y: 8 } }, Galaxy { id: 8, location: Location { x: 0, y: 9 } }, Galaxy { id: 9, location: Location { x: 4, y: 9 } }]"]);
    }

    #[test]
    fn create_pairs_test() {
        let input = load_input("src/expanded_universe_example.txt".to_owned()).unwrap();
        let galaxies = find_galaxies(input);
        let pairs = create_pairs(galaxies).unwrap();
        let result = pairs.len();
        check(&format!("{:?}", result), expect_test::expect!["36"]);
    }
}
