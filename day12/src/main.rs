use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    dot: Option<usize>,
    hash: Option<usize>,
}

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut acc = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        let checks: Vec<usize> = split[1].split(',').map(|x| x.parse().unwrap()).collect();
        let mut states: Vec<State> = vec![State {
            dot: Some(0),
            hash: Some(1),
        }];
        let states_size = checks.iter().sum::<usize>() + checks.len() + 1;
        states.reserve(states_size);
        for check in &checks {
            for _ in 1..*check {
                states.push(State {
                    dot: None,
                    hash: Some(states.len() + 1),
                });
            }
            if states.len() + 1 < states_size {
                states.push(State {
                    dot: Some(states.len() + 1),
                    hash: None,
                });
                states.push(State {
                    dot: Some(states.len()),
                    hash: Some(states.len() + 1),
                });
            }
        }
        states.remove(states_size - 1);
        let new_len = states.len();
        states[new_len - 1].hash = None;
        states[new_len - 1].dot = Some(new_len - 1);
        let configs = count(split[0].to_owned(), &states);
        acc += configs;
    }
    acc
}

fn count(input: String, states: &Vec<State>) -> usize {
    let mut curr_map: HashMap<State, usize> = HashMap::from([(states[0], 1)]);
    for c in input.chars() {
        let mut next_map: HashMap<State, usize> = HashMap::new();
        curr_map.iter().for_each(|(key, value)| {
            if let Some(dot) = key.dot {
                if c == '.' || c == '?' {
                    *next_map.entry(states[dot]).or_insert(0) += value;
                }
            }
            if let Some(hash) = key.hash {
                if c == '#' || c == '?' {
                    *next_map.entry(states[hash]).or_insert(0) += value;
                }
            }
        });
        curr_map = next_map;
    }
    let last_state = states[states.len() - 1];
    *curr_map.get(&last_state).unwrap_or(&0)
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt"), 21);
}
