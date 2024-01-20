use core::fmt;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

lazy_static! {
    static ref MOVEMENT_CHOICES: HashMap<PipeShape, (Direction, Direction)> = {
        let mut map = HashMap::new();
        map.insert(PipeShape::NS, (Direction::North, Direction::South));
        map.insert(PipeShape::NE, (Direction::North, Direction::East));
        map.insert(PipeShape::NW, (Direction::North, Direction::West));
        map.insert(PipeShape::EW, (Direction::East, Direction::West));
        map.insert(PipeShape::SE, (Direction::South, Direction::East));
        map.insert(PipeShape::SW, (Direction::South, Direction::West));
        map
    };
}

#[derive(Debug, Default)]
struct Cursor {
    came_from: Option<Direction>,
    coords: Coords,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Coords {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum PipeShape {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    G, //Maybe just None?.. see later
}

impl PipeShape {
    fn from(s: char) -> Self {
        match s {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::G,
            'S' => Self::Start, //Figure out later!
            _ => panic!("Invalid pipe shape!"),
        }
    }
}

impl fmt::Display for PipeShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::NS => "↕️",
            Self::EW => "↔️",
            Self::NE => "↪️",
            Self::NW => "↩️",
            Self::SW => "↖️",
            Self::SE => "↗️",
            Self::G => "⏹️",
            Self::Start => "▶️",
        };
        write!(f, "{}", repr)
    }
}

fn main() {
    println!("{}", run("input.txt"));
}

fn parse_input(filename: &str) -> (Vec<Vec<PipeShape>>, Option<(usize, usize)>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut iterator = reader.lines().peekable();
    let width = iterator.peek().as_ref().unwrap().as_ref().unwrap().len();
    let mut pipe_matrix: Vec<Vec<PipeShape>> = Vec::new();
    pipe_matrix.resize(width, Vec::new());
    for row in pipe_matrix.iter_mut() {
        row.resize(width, PipeShape::from('.'));
    }
    let mut start_idx: Option<(usize, usize)> = None;
    for (row, line) in iterator.enumerate() {
        let line: String = line.unwrap();
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch == 'S' {
                start_idx = Some((row, col));
            }
            pipe_matrix[row][col] = PipeShape::from(ch);
        });
    }
    (pipe_matrix, start_idx)
}

fn run(filename: &str) -> usize {
    let (mut pipe_matrix, start_idx) = parse_input(filename);
    for row in &pipe_matrix {
        for beep in row {
            print!("{beep}   ");
        }
        println!("");
    }
    if let Some(start_idx) = start_idx {
        replace_s(&start_idx, &mut pipe_matrix);
        let start_cursor = Cursor {
            coords: Coords {
                row: start_idx.0,
                col: start_idx.1,
            },
            came_from: None,
        };

        let start_coords = start_cursor.coords;
        let mut cursor = take_one_loop_step(start_cursor, &mut pipe_matrix);
        let mut counter = 1;
        while cursor.coords != start_coords {
            cursor = take_one_loop_step(cursor, &mut pipe_matrix);
            counter += 1;
        }
        counter / 2
    } else {
        0
    }
}

fn replace_s(start_idx: &(usize, usize), pipe_matrix: &mut Vec<Vec<PipeShape>>) {
    let mut connection_directions: Vec<Direction> = vec![];
    if let Some(east) = pipe_matrix[start_idx.0].get(start_idx.1 + 1) {
        if matches!(east, PipeShape::EW | PipeShape::NW | PipeShape::SW) {
            connection_directions.push(Direction::East);
        }
    }
    if let Ok(col_index) = usize::try_from(start_idx.1 as i64 - 1) {
        if let Some(west) = pipe_matrix[start_idx.0].get(col_index) {
            if matches!(west, PipeShape::EW | PipeShape::SE | PipeShape::NE) {
                connection_directions.push(Direction::West);
            }
        }
    }
    if let Ok(row_index) = usize::try_from(start_idx.0 as i64 - 1) {
        if let Some(north) = pipe_matrix.get(row_index) {
            let north = &north[start_idx.1];
            if matches!(north, PipeShape::SW | PipeShape::SE | PipeShape::NS) {
                connection_directions.push(Direction::North);
            }
        }
    }
    if let Some(south) = pipe_matrix.get(start_idx.0 + 1) {
        let south = &south[start_idx.1];
        if matches!(south, PipeShape::NW | PipeShape::NE | PipeShape::NS) {
            connection_directions.push(Direction::South);
        }
    }
    connection_directions.sort();
    let start = &mut pipe_matrix[start_idx.0][start_idx.1];
    *start = match (&connection_directions[0], &connection_directions[1]) {
        (Direction::North, Direction::South) => PipeShape::NS,
        (Direction::North, Direction::East) => PipeShape::NE,
        (Direction::North, Direction::West) => PipeShape::NW,
        (Direction::South, Direction::East) => PipeShape::SE,
        (Direction::South, Direction::West) => PipeShape::SW,
        (Direction::East, Direction::West) => PipeShape::EW,
        _ => panic!("Impossible setup!"),
    };
}

fn take_one_loop_step(cursor: Cursor, pipe_matrix: &Vec<Vec<PipeShape>>) -> Cursor {
    let Cursor { came_from, coords } = cursor;

    let here = &pipe_matrix[coords.row][coords.col];
    let directions_to_check = MOVEMENT_CHOICES.get(here).unwrap();
    let checking = if Some(directions_to_check.0) != came_from {
        directions_to_check.0
    } else {
        directions_to_check.1
    };
    match checking {
        Direction::North => Cursor {
            coords: Coords {
                row: coords.row - 1,
                col: coords.col,
            },
            came_from: Some(Direction::South),
        },
        Direction::South => Cursor {
            coords: Coords {
                row: coords.row + 1,
                col: coords.col,
            },
            came_from: Some(Direction::North),
        },
        Direction::East => Cursor {
            coords: Coords {
                row: coords.row,
                col: coords.col + 1,
            },
            came_from: Some(Direction::West),
        },
        Direction::West => Cursor {
            coords: Coords {
                row: coords.row,
                col: coords.col - 1,
            },
            came_from: Some(Direction::East),
        },
    }
}

#[test]
fn test_sample1() {
    assert_eq!(run("sample_input1.txt"), 4);
}
#[test]
fn test_sample2() {
    assert_eq!(run("sample_input2.txt"), 8);
}

#[test]
fn replace_s_works() {
    let (mut pipe_matrix, start_idx) = parse_input("sample_input1.txt");
    let start_idx = start_idx.unwrap();
    replace_s(&start_idx, &mut pipe_matrix);
    assert_eq!(pipe_matrix[start_idx.0][start_idx.1], PipeShape::SE);
}
