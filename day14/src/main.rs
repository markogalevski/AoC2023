use aoc_utils::{matrix_rotate_ccw, matrix_rotate_cw, Matrix};

use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", run("input.txt", 1000000000));
}

fn run(filename: &str, spin_for: usize) -> i64 {
    let input_matrix = read_matrix(filename);
    let mut matrix = input_matrix.clone();
    let mut hashed_states: HashMap<Matrix, usize> = HashMap::new();
    let mut cycle_and_recycle = None;
    for i in 1..=spin_for {
        matrix = spin_matrix(matrix, 1);
        if let Some(previous_add) = hashed_states.insert(matrix.clone(), i) {
            println!("Cyclic nature found at spin {i}, originally added at {previous_add}");
            cycle_and_recycle = Some((previous_add, i));
            break;
        }
    }
    let matrix = if let Some((cycle_start, recycle_at)) = cycle_and_recycle {
        let offset_spins = cycle_start;
        let new_spins = (spin_for - cycle_start) % (recycle_at - cycle_start);
        println!("About to spin for {offset_spins} + {new_spins} (original - offset % cycle length) cycles instead");
        spin_matrix(input_matrix, offset_spins + new_spins)
    } else {
        matrix
    };
    let matrix = matrix_rotate_cw(matrix);
    let rowlen = matrix.len();
    matrix
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (rowlen - i))
        .sum::<usize>() as i64
}

fn tilt_matrix(matrix: Matrix) -> Matrix {
    matrix
        .iter()
        .map(|row| {
            let new_segments: Matrix = row
                .split(|c| *c == '#')
                .map(|segment| {
                    let mut new_segment: Vec<char> = segment
                        .iter()
                        .filter(|c| **c == 'O')
                        .to_owned()
                        .copied()
                        .collect();
                    new_segment.resize(segment.len(), '.');
                    new_segment
                })
                .collect();
            new_segments.join(&'#')
        })
        .collect()
}

fn read_matrix(filename: &str) -> Matrix {
    let file = File::open(filename).unwrap_or_else(|_| panic!("File {filename} not found!"));
    let reader = BufReader::new(file);

    let mut grid: Matrix = vec![];
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|_| panic!("Unable to read line number {i}"));
        grid.push(line.chars().collect());
    }
    matrix_rotate_ccw(grid)
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt", 1000000000), 64)
}

#[test]
fn test_tilt_cycles() {
    let input_matrix = read_matrix("sample_input.txt");
    let one_cycle = spin_matrix(input_matrix, 1);

    let reference = read_matrix("sample_one_cycle.txt");
    println!("referecne");
    for row in reference.iter() {
        println!("{row:?}");
    }
    println!("");
    assert_eq!(one_cycle, reference);
}

fn spin_matrix(input_matrix: Matrix, num_spins: usize) -> Matrix {
    let mut one_cycle = input_matrix;
    for _ in 0..(4 * num_spins) {
        one_cycle = matrix_rotate_cw(tilt_matrix(one_cycle));
    }
    one_cycle
}

#[test]
fn spin_cycle_test() {
    let matrix = read_matrix("sample_input.txt");
    let three_spins = spin_matrix(matrix.clone(), 3);
    let ten_spins = spin_matrix(matrix, 17);
    assert_eq!(three_spins, ten_spins);
}
