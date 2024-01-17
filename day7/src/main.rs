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
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from_char(c: &char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
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
    ShitHand,
}

impl Hand {
    fn from_cards(cards: &Vec<Card>) -> Self {
        let mut card_map = HashMap::<Card, usize>::new();
        for card in cards {
            *card_map.entry(*card).or_insert(0) += 1;
        }
        let drain_vec: Vec<(Card, usize)> = card_map.drain().collect();
        let mut pair_filter = drain_vec.iter().filter(|(_k, v)| *v == 2);
        if drain_vec.iter().find(|(_k, v)| *v == 5).is_some() {
            Self::FiveOfAKind
        } else if drain_vec.iter().find(|(_k, v)| *v == 4).is_some() {
            Self::FourOfAKind
        } else if drain_vec.iter().find(|(_k, v)| *v == 3).is_some() {
            if drain_vec.iter().find(|(_k, v)| *v == 2).is_some() {
                Self::FullHouse
            } else {
                Self::ThreeOfAKind
            }
        } else if drain_vec.iter().max_by_key(|(_k, v)| v.clone()).unwrap().1 == 1 {
            Self::HighCard
        } else {
            if !pair_filter.next().is_some() {
                return Self::ShitHand;
            }
            if !pair_filter.next().is_some() {
                return Self::OnePair;
            }

            Self::TwoPair
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
    assert_eq!(run("sample_input.txt"), 6440);
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
