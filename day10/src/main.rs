use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq)]
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
    if let Some(start_idx) = start_idx {
        replace_s(&start_idx, &mut pipe_matrix);
    }

    for row in pipe_matrix {
        for beep in row {
            print!("{beep}   ");
        }
        println!("");
    }

    0
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ConnectionDirection {
    North,
    South,
    East,
    West,
}

fn replace_s(start_idx: &(usize, usize), pipe_matrix: &mut Vec<Vec<PipeShape>>) {
    let mut connection_directions: Vec<ConnectionDirection> = vec![];
    if let Some(east) = pipe_matrix[start_idx.0].get(start_idx.1 + 1) {
        if matches!(east, PipeShape::EW | PipeShape::NW | PipeShape::SW) {
            connection_directions.push(ConnectionDirection::East);
        }
    }
    if let Some(west) = pipe_matrix[start_idx.0].get(start_idx.1 - 1) {
        if matches!(west, PipeShape::EW | PipeShape::SE | PipeShape::NE) {
            connection_directions.push(ConnectionDirection::West);
        }
    }
    if let Some(north) = pipe_matrix.get(start_idx.0 - 1) {
        let north = &north[start_idx.1];
        if matches!(north, PipeShape::SW | PipeShape::SE | PipeShape::NS) {
            connection_directions.push(ConnectionDirection::North);
        }
    }
    if let Some(south) = pipe_matrix.get(start_idx.0 + 1) {
        let south = &south[start_idx.1];
        if matches!(south, PipeShape::NW | PipeShape::NE | PipeShape::NS) {
            connection_directions.push(ConnectionDirection::South);
        }
    }
    connection_directions.sort();
    let start = &mut pipe_matrix[start_idx.0][start_idx.1];
    *start = match (&connection_directions[0], &connection_directions[1]) {
        (ConnectionDirection::North, ConnectionDirection::South) => PipeShape::NS,
        (ConnectionDirection::North, ConnectionDirection::East) => PipeShape::NE,
        (ConnectionDirection::North, ConnectionDirection::West) => PipeShape::NW,
        (ConnectionDirection::South, ConnectionDirection::East) => PipeShape::SE,
        (ConnectionDirection::South, ConnectionDirection::West) => PipeShape::SW,
        (ConnectionDirection::East, ConnectionDirection::West) => PipeShape::EW,
        _ => panic!("Impossible setup!"),
    };
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
    let (mut pipe_matrix, start_idx) = parse_input("sample_input2.txt");
    let start_idx = start_idx.unwrap();
    replace_s(&start_idx, &mut pipe_matrix);
    assert_eq!(pipe_matrix[start_idx.0][start_idx.1], PipeShape::SE);
}
