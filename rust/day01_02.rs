use std::collections::HashMap;
use std::fs;

fn word_to_digit(word: &str) -> Option<i32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => word.parse().ok(),
    }
}

fn find_digit(line: &str, digit_map: &HashMap<String, i32>) -> (Option<i32>, Option<i32>) {
    let mut first_digit = None;
    let mut last_digit = None;

    for word in line.split_whitespace() {
        if let Some(&digit) = digit_map.get(word) {
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        }
    }

    (first_digit, last_digit)
}

fn main() {
    let data = fs::read_to_string("../input").expect("Unable to read file");
    let lines = data.lines();

    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut digit_map = HashMap::new();
    for word in &digit_words {
        digit_map.insert(word.to_string(), word_to_digit(word).unwrap());
    }

    let total_sum: i32 = lines
        .map(|line| {
            let (first_digit, last_digit) = find_digit(line, &digit_map);
            first_digit.unwrap_or(0) * 10 + last_digit.unwrap_or(0)
        })
        .sum();

    println!("Total sum of calibration values: {}", total_sum);
}
