use std::{
    cmp::max,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    loc: (i32, i32),
    dir: Direction,
}

impl Beam {
    fn oob(&self, dim: usize) -> bool {
        let loc = self.loc;
        !(loc.0 >= 0i32 && loc.0 < dim as i32 && loc.1 >= 0i32 && loc.1 < dim as i32)
    }

    fn next(&mut self) {
        let loc = self.loc;
        self.loc = match self.dir {
            Direction::Left => (loc.0, loc.1 - 1),
            Direction::Right => (loc.0, loc.1 + 1),
            Direction::Up => (loc.0 - 1, loc.1),
            Direction::Down => (loc.0 + 1, loc.1),
        }
    }
}

fn energized(grid: &Vec<Vec<char>>, start: &mut Beam) -> usize {
    use Direction::{Down, Left, Right, Up};
    let dim = grid.len();
    let mut beams = vec![start.clone()];
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();
    while let Some(mut beam) = beams.pop() {
        if visited.contains(&beam) {
            continue;
        }
        while !beam.oob(dim) {
            visited.insert(beam.clone());
            let loc = beam.loc;
            energized.insert(loc);
            match grid[loc.0 as usize][loc.1 as usize] {
                '.' => beam.next(),
                '\\' => {
                    beam.dir = match beam.dir {
                        Left => Up,
                        Right => Down,
                        Up => Left,
                        Down => Right,
                    };
                    beam.next();
                }
                '/' => {
                    beam.dir = match beam.dir {
                        Left => Down,
                        Right => Up,
                        Up => Right,
                        Down => Left,
                    };
                    beam.next();
                }
                '-' => {
                    match beam.dir {
                        Up | Down => {
                            beams.push(Beam {
                                loc: beam.loc,
                                dir: Left,
                            });
                            beams.push(Beam {
                                loc: beam.loc,
                                dir: Right,
                            });
                            break;
                        }
                        Left | Right => beam.next(),
                    };
                }
                '|' => {
                    match beam.dir {
                        Left | Right => {
                            beams.push(Beam {
                                loc: beam.loc,
                                dir: Up,
                            });
                            beams.push(Beam {
                                loc: beam.loc,
                                dir: Down,
                            });
                            break;
                        }
                        Up | Down => beam.next(),
                    };
                }
                _ => {}
            }
        }
    }
    energized.len()
}

fn part1() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in BufReader::new(handle).lines() {
        grid.push(line.unwrap().chars().collect::<Vec<_>>());
    }

    println!(
        "part 1 answer: {}",
        energized(
            &grid,
            &mut Beam {
                loc: (0, 0),
                dir: Direction::Right,
            }
        )
    );
}

fn part2() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in BufReader::new(handle).lines() {
        grid.push(line.unwrap().chars().collect::<Vec<_>>());
    }

    let dim = grid.len();

    let mut part_2_result = 0;
    for i in 0..dim {
        part_2_result = max(
            part_2_result,
            energized(
                &grid,
                &mut Beam {
                    loc: (0, i as i32),
                    dir: Direction::Down,
                },
            ),
        );
        part_2_result = max(
            part_2_result,
            energized(
                &grid,
                &mut Beam {
                    loc: (i as i32, 0),
                    dir: Direction::Right,
                },
            ),
        );
        part_2_result = max(
            part_2_result,
            energized(
                &grid,
                &mut Beam {
                    loc: (i as i32, dim as i32 - 1i32),
                    dir: Direction::Left,
                },
            ),
        );
        part_2_result = max(
            part_2_result,
            energized(
                &grid,
                &mut Beam {
                    loc: (dim as i32 - 1i32, i as i32),
                    dir: Direction::Up,
                },
            ),
        );
    }

    println!("part 2 answer: {part_2_result}");
}

fn main() {
    part1();
    part2();
}
