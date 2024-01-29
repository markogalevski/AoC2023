use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap_or_else(|_| panic!("Unable to find file {filename}"));
    let reader = BufReader::new(file);
    let mut acc: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| panic!("Unable to read line"));
        acc += line
            .split(',')
            .map(|part| hash_algorithm(part))
            .sum::<usize>();
    }
    acc
}

fn hash_algorithm(input: &str) -> usize {
    let mut current_value = 0;
    input.chars().for_each(|c| {
        let ascii: usize = if c.is_uppercase() {
            c.to_ascii_uppercase() as usize
        } else {
            c.to_ascii_lowercase() as usize
        };
        current_value += ascii;
        current_value *= 17;
        current_value %= 256;
    });
    current_value
}

#[test]
fn test_hash() {
    assert_eq!(hash_algorithm("HASH"), 52);
}

#[test]
fn test_sample_input() {
    assert_eq!(run("sample_input.txt"), 1320);
}
