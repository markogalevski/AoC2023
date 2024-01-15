use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

fn main() {
    let sum = run("input.txt");
    println!("{sum}");
}

fn calculate_copy_ids(id: usize, card_input: &Vec<&str>) -> Vec<usize> {
    let winning_vector: Vec<usize> = card_input[0]
        .split_whitespace()
        .map(|p| {
            p.parse().expect(&format!(
                "Unable to extract values from  '{}' in '{}'",
                p, card_input[0]
            ))
        })
        .collect();
    let drawn_vector: Vec<usize> = card_input[1]
        .split_whitespace()
        .map(|p| {
            p.parse().expect(&format!(
                "Unable to extract values from '{}' in '{}'",
                p, card_input[1]
            ))
        })
        .collect();
    let winning_numbers: HashSet<usize> = HashSet::from_iter(winning_vector.iter().cloned());
    let drawn_numbers: HashSet<usize> = HashSet::from_iter(drawn_vector.iter().cloned());
    let intersection: HashSet<_> =
        <std::collections::hash_set::Intersection<'_, usize, _> as Iterator>::collect::<
            HashSet<&usize>,
        >(winning_numbers.intersection(&drawn_numbers));
    (id + 1..=(id + intersection.len())).collect()
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    let mut histogram: HashMap<usize, usize> = HashMap::new();
    for line in file.lines() {
        let line = line.unwrap();
        let card_input: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let id: usize = card_input[0]
            .replace("Card", "")
            .trim_start()
            .parse()
            .expect(&format!("Unable to get game id from '{}'", card_input[0]));

        *histogram.entry(id).or_insert(0) += 1;
        let card_input: Vec<&str> = card_input[1].split('|').collect();
        let copy_ids = calculate_copy_ids(id, &card_input);
        for _ in 0..*histogram.get(&id).unwrap() {
            for id in &copy_ids {
                *histogram.entry(*id).or_insert(0) += 1;
            }
        }
    }
    histogram.into_values().sum()
}

#[test]
fn example_test() {
    assert_eq!(run("small_input.txt"), 30);
}
