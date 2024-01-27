use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type MirrorMatrix = Vec<Vec<char>>;

fn matrix_transpose(matrix: MirrorMatrix) -> MirrorMatrix {
    let new_row: Vec<char> = vec!['x'; matrix.len()];
    let mut new_matrix: MirrorMatrix = vec![new_row; matrix[0].len()];
    matrix.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, element)| {
            new_matrix[j][i] = *element;
        })
    });
    new_matrix
}

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut acc = 0;
    let mut matrix: MirrorMatrix = Vec::new();
    for line in reader.lines() {
        let line: String = line.unwrap();
        if line.is_empty() {
            acc += calculate_matrix(&matrix);
            matrix.clear();
        } else {
            matrix.push(line.chars().collect());
        }
    }
    acc += calculate_matrix(&matrix);
    acc
}

fn calculate_matrix(matrix: &Vec<Vec<char>>) -> usize {
    if let Some(symmetry_line) = find_symmetry_line(matrix) {
        100 * symmetry_line
    } else {
        let matrix = matrix_transpose(matrix.clone());
        find_symmetry_line(&matrix).expect("really expect symmetry at this point")
    }
}

fn find_symmetry_line(matrix: &Vec<Vec<char>>) -> Option<usize> {
    let mut prev: Vec<char> = Vec::new();
    let mut symmetry_line = None;
    for (i, row) in matrix.iter().enumerate() {
        if smudged_equality(row, &prev).is_some() {
            symmetry_line = check_symmetry(matrix, i);
        }
        if symmetry_line.is_some() {
            break;
        }
        prev = row.clone();
    }
    symmetry_line
}

fn smudged_equality(row1: &[char], row2: &[char]) -> Option<usize> {
    let num_mismatched = row1.len() - row1.iter().zip(row2.iter()).filter(|(a, b)| a == b).count();
    (num_mismatched <= 1).then_some(num_mismatched)
}

#[derive(Debug)]
enum MeasurementFrame {
    PreLine,
    PostLine,
}

fn check_symmetry(matrix: &MirrorMatrix, symmetry_line: usize) -> Option<usize> {
    let pre_post_line = if symmetry_line <= matrix.len() / 2 {
        MeasurementFrame::PreLine
    } else {
        MeasurementFrame::PostLine
    };
    let mut symmetry_slice;
    let opposite_slice;
    match pre_post_line {
        MeasurementFrame::PreLine => {
            symmetry_slice = matrix[0..symmetry_line].to_vec();
            symmetry_slice.reverse();
            opposite_slice = matrix[symmetry_line..symmetry_line * 2].to_vec();
        }
        MeasurementFrame::PostLine => {
            symmetry_slice = matrix[symmetry_line..].to_vec();
            symmetry_slice.reverse();
            opposite_slice = matrix[symmetry_line - symmetry_slice.len()..symmetry_line].to_vec();
        }
    }

    let mut equality_map = symmetry_slice
        .iter()
        .zip(opposite_slice.iter())
        .map(|(row1, row2)| smudged_equality(row1, row2));
    let smudge_sum: usize = equality_map.clone().flatten().sum();

    if equality_map.any(|x| x.is_none()) {
        return None;
    }
    (smudge_sum == 1).then_some(symmetry_line)
}

#[test]
fn test_sample() {
    assert_eq!(run("sample_input.txt"), 400);
}

#[test]
fn test_input() {
    assert_eq!(run("input.txt"), 35554);
}

#[test]
fn test_matrix_transpose() {
    let a = vec!['a', 'b', 'c', 'd'];
    let b = vec!['e', 'f', 'g', 'h'];
    let matrix: MirrorMatrix = vec![a, b];
    let matrix_t = matrix_transpose(matrix);
    let reference: MirrorMatrix = vec![
        vec!['a', 'e'],
        vec!['b', 'f'],
        vec!['c', 'g'],
        vec!['d', 'h'],
    ];

    assert_eq!(matrix_t, reference);
}
