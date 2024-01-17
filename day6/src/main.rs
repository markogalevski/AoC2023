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

#[derive(Debug)]
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

fn parse_input(reader: BufReader<File>) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let mut times: Vec<usize> = vec![];
    let mut distances: Vec<usize> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(':').collect();
        let tag = split[0];
        let parts = split[1];
        let parts: Vec<&str> = parts.split_whitespace().collect();
        match Tag::from_str(tag) {
            Tag::Time => {
                for part in parts {
                    times.push(part.parse().unwrap());
                }
            }
            Tag::Distance => {
                for part in parts {
                    distances.push(part.parse().unwrap());
                }
            }
        }
        for (time, distance) in times.iter().zip(distances.iter()) {
            races.push(Race {
                time: *time,
                distance: *distance,
            });
        }
    }
    races
}

fn main() {
    let product = run("input.txt");
    println!("{product}");
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let file: BufReader<File> = BufReader::new(file);
    let races = parse_input(file);
    races
        .iter()
        .map(|race| race.calc_num_win_conditions())
        .product()
}

#[test]
fn sample_test() {
    assert_eq!(run("sample_input.txt"), 288);
}
