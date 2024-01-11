#![allow(dead_code)]

use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Part {
    number: usize,
    dimensions: Dimensions,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} in row {}, start: {}, end: {}",
            self.number, self.dimensions.row, self.dimensions.start, self.dimensions.end
        )
    }
}

#[derive(Debug, Clone)]
struct Dimensions {
    row: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Gear {
    row: usize,
    column: usize,
    attached_parts: Vec<Part>,
}

impl Gear {
    fn is_adjacent_to(&self, part: &Part) -> bool {
        (self.row as isize - part.dimensions.row as isize).abs() <= 1
            && isize::try_from(part.dimensions.start).unwrap() - 1 <= self.column as isize
            && self.column <= part.dimensions.end
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let file = BufReader::new(file);

    let mut gears: Vec<Gear> = vec![];
    let mut parts: Vec<Part> = vec![];
    for (row, line) in file.lines().enumerate() {
        let input = line.unwrap();
        gears.extend(
            input
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(|(column, _)| Gear {
                    row,
                    column,
                    attached_parts: vec![],
                })
                .collect::<Vec<Gear>>(),
        );

        let re = Regex::new("[0-9]+").unwrap();
        let mut i = 0;
        loop {
            if let Some(found) = re.find_at(&input, i) {
                parts.push(Part {
                    number: found.as_str().parse().unwrap(),
                    dimensions: Dimensions {
                        row,
                        start: found.start(),
                        end: found.end(),
                    },
                });
                i = found.end();
            } else {
                break;
            }
        }
    }
    for part in parts.iter_mut() {
        for gear in gears.iter_mut() {
            if gear.is_adjacent_to(&part) {
                gear.attached_parts.push(part.clone());
            }
        }
    }
    let sum: usize = gears
        .iter()
        .filter(|g| g.attached_parts.len() > 1)
        .map(|g| g.attached_parts.iter().map(|p| p.number).product::<usize>())
        .sum();
    println!("{sum}");
}
