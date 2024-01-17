use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Copy, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn from_char(c: &char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'J' => Self::Joker,
            &_ => panic!("Not a valid card! {c}"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn from_cards(cards: &Vec<Card>) -> Self {
        let mut card_map = HashMap::<Card, usize>::new();
        for card in cards {
            *card_map.entry(*card).or_insert(0) += 1;
        }
        let num_jokers = if let Some(entry) = card_map.remove_entry(&Card::Joker) {
            entry.1
        } else {
            0
        };
        let mut values: Vec<usize> = card_map.into_values().collect();
        values.sort();
        let values: Vec<usize> = values.into_iter().rev().collect();
        let max_occuring_card = *values.get(0).unwrap_or(&0);

        if max_occuring_card + num_jokers >= 5 {
            Self::FiveOfAKind
        } else if max_occuring_card + num_jokers >= 4 {
            Self::FourOfAKind
        } else if max_occuring_card + num_jokers >= 3 {
            if values[1..].iter().any(|count| *count >= 2) {
                Self::FullHouse
            } else {
                Self::ThreeOfAKind
            }
        } else if max_occuring_card + num_jokers >= 2 {
            let num_jokers = if max_occuring_card == 1 {
                num_jokers - 1
            } else {
                num_jokers
            };
            if values[1] + num_jokers == 2 {
                Self::TwoPair
            } else {
                Self::OnePair
            }
        } else {
            Self::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cards(Vec<Card>);

#[derive(Debug)]
struct Player {
    hand: Hand,
    bet: usize,
    cards: Cards,
}

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).expect(&format!("Unable to open file {filename}"));
    let reader = BufReader::new(file);
    let mut players = parse_players(reader);
    players.sort_by(|a, b| {
        let result = a.hand.cmp(&b.hand);
        match result {
            Ordering::Equal => a.cards.0.cmp(&b.cards.0),
            _ => result,
        }
    });
    let players: Vec<Player> = players.into_iter().rev().collect();
    players
        .iter()
        .enumerate()
        .map(|(i, p)| p.bet * (i + 1))
        .sum()
}

fn parse_players(reader: BufReader<File>) -> Vec<Player> {
    let mut players: Vec<Player> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cards: Vec<Card> = parts[0].chars().map(|c| Card::from_char(&c)).collect();
        players.push(Player {
            hand: Hand::from_cards(&cards),
            bet: parts[1].parse().unwrap(),
            cards: Cards(cards),
        })
    }
    players
}

#[test]
fn sample_test() {
    assert_eq!(run("sample_input.txt"), 5905);
}

#[test]
fn test_num_players_sample() {
    let file = File::open("sample_input.txt").unwrap();
    let reader = BufReader::new(file);
    assert_eq!(parse_players(reader).len(), 5);
}

#[test]
fn test_num_players() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    assert_eq!(parse_players(reader).len(), 1000);
}
