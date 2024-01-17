use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Tag {
    Time,
    Distance,
}

impl Tag {
    fn from_str(s: &str) -> Self {
        match s {
            "Time" => Self::Time,
            "Distance" => Self::Distance,
            &_ => panic!("Unexpected str type: {s}"),
        }
    }
}

#[derive(Debug, Default)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn calc_num_win_conditions(&self) -> usize {
        /*
            (time - x) * x > distance
            x = 1 -> 6 * 1 = 6 < d
            x = 2 -> 5 * 2 = 10 >d
            ...
            x = 3 => 3 * 4 = 12
            x = 4 => 4 * 3 = 12
            ..
        i.e we have symmetry around the middle and only have to calculate to halfway.
        x = 1..=(time / 2), then, if odd, double the solutions. if even: double - 1

        */
        let mut counter = 0;
        for x in 1..=(self.time / 2) {
            if (self.time - x) * x > self.distance {
                counter += 1;
            }
        }
        counter *= 2;
        if self.time % 2 == 0 {
            counter -= 1;
        }
        counter
    }
}

fn parse_input(reader: BufReader<File>) -> Race {
    let mut race = Race::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(':').collect();
        let tag = split[0];
        let parts = split[1];
        let parts: String = parts.split_whitespace().collect::<Vec<&str>>().join("");
        match Tag::from_str(tag) {
            Tag::Time => {
                race.time = parts.parse().unwrap();
            }
            Tag::Distance => {
                race.distance = parts.parse().unwrap();
            }
        }
    }
    race
}

fn main() {
    let product = run("input.txt");
    println!("{product}");
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let file: BufReader<File> = BufReader::new(file);
    let race = parse_input(file);
    race.calc_num_win_conditions()
}

#[test]
fn sample_test() {
    assert_eq!(run("sample_input.txt"), 71503);
}
