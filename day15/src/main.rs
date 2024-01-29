use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", run("input.txt"));
}

#[derive(Clone, Default, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn run(filename: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = Vec::with_capacity(256);
    boxes.resize(256, vec![]);
    let file = File::open(filename).unwrap_or_else(|_| panic!("Unable to find file {filename}"));
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| panic!("Unable to read line"));
        line.split(',').for_each(|part| {
            if part.contains('=') {
                let part_split: Vec<&str> = part.split('=').collect();
                let lens = Lens {
                    label: part_split[0].to_owned(),
                    focal_length: part_split[1].parse().unwrap(),
                };
                let index = hash_algorithm(&lens.label);
                let lens_index = boxes[index].iter().position(|p| p.label == lens.label);
                if let Some(lens_index) = lens_index {
                    boxes[index][lens_index].focal_length = lens.focal_length;
                } else {
                    boxes[index].push(lens);
                }
            }
            if part.contains('-') {
                let part = part.replace('-', "");
                let index = hash_algorithm(&part);
                let found_at = boxes[index].iter().position(|p| p.label == part);
                if let Some(found_at) = found_at {
                    boxes[index].remove(found_at);
                }
            }
        });
    }
    boxes
        .iter()
        .enumerate()
        .map(|(i, lensbox)| {
            lensbox
                .iter()
                .enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum()
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
    assert_eq!(run("sample_input.txt"), 145);
}
