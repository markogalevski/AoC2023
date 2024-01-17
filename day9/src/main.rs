use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sums: i64 = 0;
    for line in reader.lines() {
        let mut histories: Vec<Vec<i64>> = vec![];
        let line = line.unwrap();
        let mut history: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        histories.push(history.clone());
        while !history.iter().all(|x| *x == 0) {
            history = history.clone().windows(2).map(|sl| sl[1] - sl[0]).collect();
            histories.push(history.clone());
        }

        sums += histories[0][0]
            - histories[1..]
                .iter()
                .enumerate()
                .map(|(i, history)| {
                    let val = history.iter().next().unwrap() * ((-1_i64).pow((i) as u32));
                    val
                })
                .reduce(|acc, x| acc + x)
                .unwrap();
    }
    sums
}

#[test]
fn sample_test() {
    assert_eq!(run("sample_input.txt"), 2);
}
