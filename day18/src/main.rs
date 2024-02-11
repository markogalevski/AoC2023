use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_utils::polygon::{
    circumference, picks_theorem_num_internal_points, shoelace, Point, Polygon,
};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::convert::From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            &_ => panic!("unexpected direction!"),
        }
    }
}

impl std::convert::From<i64> for Direction {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("unexpected direction!"),
        }
    }
}

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let start_point = Point::new(0, 0);
    let mut polygon: Polygon = vec![start_point];
    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split_whitespace().collect();
        let code: String = line[2].chars().filter(|c| c.is_alphanumeric()).collect();
        let code: i64 = i64::from_str_radix(&code, 16).unwrap();
        let direction = Direction::from(code & 0xF);
        let steps = code >> 4;
        let latest_point = polygon.iter().last().unwrap();
        let offset = compute_offset(direction, steps);
        polygon.push(latest_point.add(&offset));
    }
    let area = shoelace(&polygon);
    let circumference = circumference(&polygon) as i64;
    let internal_points = picks_theorem_num_internal_points(area, circumference);
    circumference + internal_points
}

fn compute_offset(dir: Direction, steps: i64) -> Point {
    let mut offset = Point::new(0, 0);
    match dir {
        Direction::Right => offset.col += steps,
        Direction::Left => offset.col -= steps,
        Direction::Up => offset.row -= steps,
        Direction::Down => offset.row += steps,
    };
    offset
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt"), 952408144115);
}
