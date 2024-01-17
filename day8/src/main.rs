use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("invalid direction {c}, check input!"),
        }
    }
}

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let (directions, map) = parse_input(reader);
    let mut current_location = "AAA".to_owned();
    let destination = "ZZZ".to_owned();
    let mut circular = directions.iter().cycle();
    let mut counter = 0;
    while current_location != destination {
        let choices = map.get(&current_location).unwrap();
        current_location = match circular.next().unwrap() {
            Direction::Right => choices.1.clone(),
            Direction::Left => choices.0.clone(),
        };
        counter += 1;
    }
    counter
}

fn parse_input(reader: BufReader<File>) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let mut lines = reader.lines();
    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| Direction::from_char(c))
        .collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        if !line.is_empty() {
            let split: Vec<&str> = line.split('=').collect();
            let key = split[0].trim().to_owned();
            let value: String = split[1].replace('(', "").replace(')', "").replace(',', "");
            let value_vec: Vec<String> = value.split_whitespace().map(|s| s.to_owned()).collect();
            let tuple = (value_vec[0].clone(), value_vec[1].clone());
            map.insert(key, tuple);
        }
    }
    (directions, map)
}

#[test]
fn sample_one() {
    assert_eq!(run("sample_input1.txt"), 2);
}

#[test]
fn sample_two() {
    assert_eq!(run("sample_input2.txt"), 6);
}
