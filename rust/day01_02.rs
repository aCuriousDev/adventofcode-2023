use std::fs;

fn sum_calibration_values(document: &str) -> u32 {
    let digit_map = vec![
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    document
        .lines()
        .filter_map(|line| {
            let processed_line = digit_map
                .iter()
                .fold(line.to_string(), |acc, &(word, digit)| {
                    acc.replace(word, &digit.to_string())
                });

            let first_digit = processed_line.chars().find(|c| c.is_digit(10));
            let last_digit = processed_line.chars().rev().find(|c| c.is_digit(10));

            match (first_digit, last_digit) {
                (Some(f), Some(l)) => Some(f.to_digit(10).unwrap() * 10 + l.to_digit(10).unwrap()),
                _ => None,
            }
        })
        .sum()
}

fn main() {
    let document = fs::read_to_string("../input").expect("Failed to read 'input' file");

    let total_sum = sum_calibration_values(&document);
    println!("Total sum of calibration values: {}", total_sum);
}
