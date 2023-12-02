use std::fs;

fn sum_calibration_values(document: &str) -> u32 {
    document
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10));
            let last_digit = line.chars().rev().find(|c| c.is_digit(10));

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
