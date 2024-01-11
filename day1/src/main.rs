use load_input::load_input;
use std::io::BufRead;

fn main() {
    let file = load_input();

    let mut sum = 0;
    let textual = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in file.lines() {
        let mut input: String = line.unwrap();
        for (i, word) in textual.iter().rev().enumerate() {
            /* Absolutely stupid solution to avoid overwriting overlapping number names */
            for (j, _found) in input.clone().match_indices(word) {
                input.replace_range(j + 1..j + 2, &(9 - i).to_string());
            }
        }
        let nums: String = input.chars().filter(|c| c.is_numeric()).collect();
        let value: usize = [
            nums.chars().next().unwrap(),
            nums.chars().next_back().unwrap(),
        ]
        .to_vec()
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
        sum += value;
    }

    println!("{sum}");
}
