use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Finish,
}

fn connected(p: (usize, usize), grid: &[Vec<char>]) -> Vec<((usize, usize), Direction)> {
    let mut connected = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    let current = grid[p.0][p.1];

    if (current == '-' || current == 'S' || current == 'L' || current == 'F') && p.1 < cols - 1 {
        match grid[p.0][p.1 + 1] {
            '-' => connected.push(((p.0, p.1 + 1), Direction::Right)),
            '7' => connected.push(((p.0, p.1 + 1), Direction::DownRight)),
            'J' => connected.push(((p.0, p.1 + 1), Direction::UpRight)),
            'S' => connected.push(((p.0, p.1 + 1), Direction::Finish)),
            _ => {}
        }
    }
    if (current == '-' || current == 'S' || current == '7' || current == 'J') && p.1 > 0 {
        match grid[p.0][p.1 - 1] {
            '-' => connected.push(((p.0, p.1 - 1), Direction::Left)),
            'L' => connected.push(((p.0, p.1 - 1), Direction::UpLeft)),
            'F' => connected.push(((p.0, p.1 - 1), Direction::DownLeft)),
            'S' => connected.push(((p.0, p.1 - 1), Direction::Finish)),
            _ => {}
        }
    }
    if (current == '|' || current == 'S' || current == 'F' || current == '7') && p.0 < rows - 1 {
        match grid[p.0 + 1][p.1] {
            '|' => connected.push(((p.0 + 1, p.1), Direction::Down)),
            'L' => connected.push(((p.0 + 1, p.1), Direction::DownRight)),
            'J' => connected.push(((p.0 + 1, p.1), Direction::DownLeft)),
            'S' => connected.push(((p.0 + 1, p.1), Direction::Finish)),
            _ => {}
        }
    }
    if (current == '|' || current == 'S' || current == 'J' || current == 'L') && p.0 > 0 {
        match grid[p.0 - 1][p.1] {
            '|' => connected.push(((p.0 - 1, p.1), Direction::Up)),
            '7' => connected.push(((p.0 - 1, p.1), Direction::UpLeft)),
            'F' => connected.push(((p.0 - 1, p.1), Direction::UpRight)),
            'S' => connected.push(((p.0 - 1, p.1), Direction::Finish)),
            _ => {}
        }
    }

    connected
}

fn solution() {
    let path: PathBuf = "src/input_large".into();
    let handle = File::open(path.clone()).unwrap();

    let mut part_1_result = 0;
    let mut part_2_result = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let (mut s_i, mut s_j) = (0, 0);
    for (idx, line) in BufReader::new(handle).lines().enumerate() {
        let line = line.expect("line must exist").chars().collect::<Vec<_>>();
        if let Some(i) = line.iter().position(|c| *c == 'S') {
            s_i = idx;
            s_j = i;
        }

        grid.push(line);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Copy of the grid that only contains the loop and `.` otherwise
    let mut simple_grid: Vec<Vec<char>> = grid.clone();
    for col in simple_grid.iter_mut().take(rows) {
        for el in col.iter_mut().take(cols) {
            *el = '.';
        }
    }

    // Matrix of all the directions. Use `Direction::Finish` for non-useful entries
    let mut directions: Vec<Vec<Direction>> = Vec::new();
    for _i in 0..rows {
        let mut v = Vec::new();
        for _j in 0..cols {
            v.push(Direction::Finish);
        }
        directions.push(v);
    }

    // Starting point
    let mut p = (s_i, s_j);
    let connections = connected(p, &grid);
    simple_grid[s_i][s_j] = grid[s_i][s_j];

    // Next point
    let mut previous = p;
    p = connections[0].0;
    simple_grid[p.0][p.1] = grid[p.0][p.1];
    (p, directions[p.0][p.1]) = connections[0].clone();

    part_1_result += 1;
    while p != (s_i, s_j) {
        // Traverse the loop by looking at the connections, until we go back to `S`
        let mut connections = connected(p, &grid);
        connections.retain(|c| previous != c.0); // Make sure not to go back to `previous`
        previous = p;
        p = connections[0].0;
        simple_grid[p.0][p.1] = grid[p.0][p.1];
        (p, directions[p.0][p.1]) = connections[0].clone();

        part_1_result += 1;
    }

    part_1_result /= 2;

    for i in 0..rows {
        let mut inside = false; // Indicates when we're going inside the loop
        let mut row_area: i32 = 0;
        for j in 0..cols {
            use Direction::{Down, DownLeft, DownRight, Up, UpLeft, UpRight};
            match (simple_grid[i][j], &directions[i][j]) {
                ('|', Up) | ('7' | 'L', UpLeft) | ('J' | 'F', UpRight) => inside = true,
                ('|', Down) | ('7' | 'L', DownRight) | ('J' | 'F', DownLeft) => inside = false,
                ('.', _) => {
                    if inside {
                        row_area += 1;
                    }
                }
                _ => {}
            }
        }
        part_2_result += row_area;
    }

    println!("part 1 answer: {part_1_result}");
    println!("part 2 answer: {part_2_result}");
}

fn main() {
    solution();
}
