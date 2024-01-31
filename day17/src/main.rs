use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<usize>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        matrix.push(line.chars().map(|c| c as usize).collect());
    }
    0
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt") 102);
}
