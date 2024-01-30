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
        let checks: Vec<usize> = split[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);
        let states = create_states(checks);
        let mut input = split[0].to_owned();
        input.push_str("?");
        input = input.repeat(5);
        let len = input.len();
        input.remove(len - 1);

        let configs = count(input, &states);
        acc += configs;
    }
    acc
}

/* DFA method obtained from this article:
https://alexoxorn.github.io/posts/aoc-day12-regular_languages/
What I learned using this method:
    * All regex can be represented as a DFA
    * It's easier and safer to work with Option'd indexes to a vector in Rust
    * Sometimes you don't need to create 800 new types and enums to solve a simple problem
*/
fn create_states(checks: Vec<usize>) -> Vec<State> {
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
    states
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
    assert_eq!(run("sample_input.txt"), 525152);
}
