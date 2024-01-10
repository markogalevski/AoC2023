#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const NUM_REDS: usize = 12;
const NUM_GREENS: usize = 13;
const NUM_BLUES: usize = 14;

#[derive(Default)]
struct Round {
    reds: usize,
    greens: usize,
    blues: usize,
}

impl Round {
    fn from(s: &str) -> Self {
        let string_vec: Vec<&str> = s.split(|c: char| !c.is_alphanumeric()).collect();
        let mut me = Self::default();

        for (i, part) in string_vec.iter().enumerate() {
            if *part == "red" {
                me.reds = string_vec[i - 1].parse().unwrap();
            }
            if *part == "green" {
                me.greens = string_vec[i - 1].parse().unwrap();
            }
            if *part == "blue" {
                me.blues = string_vec[i - 1].parse().unwrap();
            }
        }
        me
    }

    fn is_impossible(&self) -> bool {
        self.reds > NUM_REDS || self.greens > NUM_GREENS || self.blues > NUM_BLUES
    }
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn is_possible(&self) -> bool {
        !self.rounds.iter().any(|round| round.is_impossible())
    }

    fn min_reds(&mut self) -> usize {
        self.rounds
            .sort_by(|a, b| b.reds.partial_cmp(&a.reds).unwrap());
        self.rounds[0].reds
    }
    fn min_greens(&mut self) -> usize {
        self.rounds
            .sort_by(|a, b| b.greens.partial_cmp(&a.greens).unwrap());
        self.rounds[0].greens
    }
    fn min_blues(&mut self) -> usize {
        self.rounds
            .sort_by(|a, b| b.blues.partial_cmp(&a.blues).unwrap());
        self.rounds[0].blues
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let file = BufReader::new(file);
    let mut games: Vec<Game> = vec![];
    for line in file.lines() {
        let input = line.unwrap();
        let game_rounds_split: Vec<&str> = input.split(':').collect();
        games.push(Game {
            id: game_rounds_split[0]
                .replace("Game", " ")
                .trim_start()
                .parse()
                .unwrap(),
            rounds: {
                let rounds: Vec<&str> = game_rounds_split[1].split(';').collect();
                rounds.iter().map(|r| Round::from(r.to_owned())).collect()
            },
        });
    }
    let sum: usize = games
        .iter()
        .map(|g| if g.is_possible() { g.id } else { 0 })
        .sum();

    println!("Sum of possible game ids: {sum}");
    let power: usize = games
        .iter_mut()
        .map(|g| g.min_reds() * g.min_greens() * g.min_blues())
        .sum();
    println!("power: {power}");
}
