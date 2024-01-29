use aoc_utils::{matrix_transpose, Matrix};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> i64 {
    let matrix = read_matrix(filename);
    matrix
        .iter()
        .map(|row| {
            let row_length = row.len();
            let mut start_point: i64 = row_length.try_into().unwrap();
            row.split(|c| *c == '#')
                .map(|segment| {
                    let end_point =
                        start_point - segment.iter().filter(|c| **c == 'O').count() as i64;
                    let segment_sum = ((end_point + 1)..=start_point).sum::<i64>();
                    start_point -= i64::try_from(segment.len()).unwrap() + 1;
                    segment_sum
                })
                .sum::<i64>()
        })
        .sum()
}

fn read_matrix(filename: &str) -> Matrix {
    let file = File::open(filename).expect(&format!("File {filename} not found!"));
    let reader = BufReader::new(file);

    let mut grid: Matrix = vec![];
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect(&format!("Unable to read line number {i}"));
        grid.push(line.chars().collect());
    }
    /* working with rows is easier than dealing with columns */
    matrix_transpose(grid)
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt"), 136)
}
