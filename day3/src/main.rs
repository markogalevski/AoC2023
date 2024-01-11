#![allow(dead_code)]

use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
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

#[derive(Debug)]
struct Dimensions {
    row: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct SpecialSymbol {
    symbol: String,
    row: usize,
    column: usize,
}

impl SpecialSymbol {
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

    let mut special_symbols: Vec<SpecialSymbol> = vec![];
    let mut parts: Vec<Part> = vec![];
    for (row, line) in file.lines().enumerate() {
        let input = line.unwrap();
        special_symbols.extend(
            input
                .chars()
                .enumerate()
                .filter(|(_, c)| !c.is_numeric() && *c != '.')
                .map(|(column, char)| SpecialSymbol {
                    symbol: char.to_string(),
                    row,
                    column,
                })
                .collect::<Vec<SpecialSymbol>>(),
        );

        let re = Regex::new("[0-9]+").unwrap();
        let mut i = 0;
        loop {
            if let Some(found) = re.find_at(&input, i) {
                let part = Part {
                    number: found.as_str().parse().unwrap(),
                    dimensions: Dimensions {
                        row,
                        start: found.start(),
                        end: found.end(),
                    },
                };
                parts.push(part);
                i = found.start() + found.len();
            } else {
                break;
            }
        }
    }

    let parts: Vec<&Part> = parts
        .iter()
        .filter(|p| special_symbols.iter().any(|s| s.is_adjacent_to(&p)))
        .collect();
    let sum: usize = parts.iter().map(|p| p.number).sum();
    println!("{sum}");
}
