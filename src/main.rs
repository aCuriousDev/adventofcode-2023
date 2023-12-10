use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let board: Vec<&str> = content.lines().collect();
    let mut chars: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut sum_part_numbers = 0;

    let re = Regex::new(r"\d+").unwrap();

    for (r, row) in board.iter().enumerate() {
        for cap in re.find_iter(row) {
            let num: i32 = row[cap.start()..cap.end()].parse().unwrap();
            let mut added = false;

            for rr in (r as i32 - 1)..=(r as i32 + 1) {
                for cc in (cap.start() as i32 - 1)..=(cap.end() as i32) {
                    if rr < 0
                        || cc < 0
                        || rr as usize >= board.len()
                        || cc as usize >= board[rr as usize].len()
                    {
                        continue;
                    }

                    let ch = board[rr as usize].chars().nth(cc as usize).unwrap();
                    if ch != '.' {
                        if !added {
                            sum_part_numbers += num;
                            added = true;
                        }
                        chars
                            .entry((rr as usize, cc as usize))
                            .or_insert_with(Vec::new)
                            .push(num);
                    }
                }
            }
        }
    }

    let gear_ratios_sum: i32 = chars
        .values()
        .filter(|p| p.len() == 2)
        .map(|p| p.iter().product::<i32>())
        .sum();

    println!("Part I: {}", sum_part_numbers);
    println!("Part II: {}", gear_ratios_sum);
}
