use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum SpringError {
    #[error("Cannot check springs with unknown states!")]
    UnknownState,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Spring {
    Okay,
    Damaged,
    Unknown,
}

impl Spring {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Okay,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Unrecognised spring state!!!"),
        }
    }

    const VALUES: [Self; 2] = [Self::Okay, Self::Damaged];
}

fn main() {
    println!("Hello, world!");
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        let springs = split[0];
        let mut springs: Vec<Spring> = springs.chars().map(|c| Spring::from_char(c)).collect();
        let checks: Vec<usize> = split[1].split(',').map(|x| x.parse().unwrap()).collect();
        let num_unknown = springs.iter().filter(|s| **s == Spring::Unknown).count();
        println!("{num_unknown}");
        replace_and_check(&mut springs, &checks);
    }
    0
}

fn replace_and_check(springs: &mut Vec<Spring>, checks: &Vec<usize>) -> bool {
    // TODO: This is wrong and dumb. Permutations get deleted... Fix it!
    // 1. Get indices of first unknown
    if let Some(index) = springs.iter().position(|s| *s == Spring::Unknown) {
        // 2. Call loop for value in Spring::VALUES.iter() {}
        for value in Spring::VALUES.iter() {
            // 3. replace springs[index] = value
            springs[index] = *value;
            if let Err(_) = check_springs(&springs, checks) {
                replace_and_check(springs, checks);
            }
        }
    }
    if let Ok(is_good) = check_springs(&springs, checks) {
        is_good
    } else {
        panic!("SHOULDN'T HAPPEN");
    }

    // 4. check_springs(). If error, recurse again, feeding in the mutable spring reference
}

fn check_springs(springs: &Vec<Spring>, checks: &Vec<usize>) -> Result<bool, SpringError> {
    if springs.iter().any(|s| *s == Spring::Unknown) {
        return Err(SpringError::UnknownState);
    }
    let mut remover = false;
    let springs: Vec<Spring> = springs
        .iter()
        .map(|spring| {
            if *spring == Spring::Okay {
                if remover == false {
                    remover = true;
                    Some(spring)
                } else {
                    None
                }
            } else {
                remover = false;
                Some(spring)
            }
        })
        .filter_map(|s| s)
        .map(|s| *s)
        .collect();
    Ok(springs
        .split(|s| *s == Spring::Okay)
        .zip(checks)
        .all(|(s, c)| s.len() == *c))
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt"), 21);
}

#[test]
fn test_spring_condition() {
    let springs: Vec<Spring> = vec![
        Spring::Damaged,
        Spring::Okay,
        Spring::Damaged,
        Spring::Okay,
        Spring::Damaged,
        Spring::Damaged,
        Spring::Damaged,
    ];
    let checks: Vec<usize> = vec![1, 1, 3];
    assert!(check_springs(&springs, &checks).unwrap())
}
