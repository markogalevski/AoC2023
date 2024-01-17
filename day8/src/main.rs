use num::integer::lcm;
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

fn run(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let (directions, map) = parse_input(reader);
    let current_locations: Vec<String> = map.keys().cloned().filter(|k| k.ends_with("A")).collect();
    let mut circular = directions.iter().cycle();
    /*
    A helpful comment on reddit mentioned LCMs, mainly because of the fact that
    1. 6 various start points will at (usually different times) end in Z, but since they're not going to be ZZZ, they can jump anywhere
    2. i.e. every start point has a period and will reach a thing ending in Z after a certain number of steps before looping.
    3. So taking the LCM of the 6 start points will find the smallest number of cycles that will result in all the periods matching up
    */
    let periods: i64 = current_locations
        .iter()
        .map(|loc| {
            let mut loop_loc = loc.to_owned();
            let mut counter: i64 = 0;
            while !loop_loc.ends_with("Z") {
                let choices = map.get(&loop_loc).unwrap();
                loop_loc = match circular.next().unwrap() {
                    Direction::Right => choices.1.clone(),
                    Direction::Left => choices.0.clone(),
                };
                counter += 1;
            }
            counter
        })
        .fold(1, |acc, x| lcm(acc, x));
    periods
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

#[test]
fn sample_three() {
    assert_eq!(run("sample_input3.txt"), 6);
}
