use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn parse_game_data(line: &str) -> (i32, Vec<HashMap<String, i32>>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let game_id = parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
    let reveals = parts[1].split("; ")
        .map(|reveal| {
            let mut counts = HashMap::new();
            counts.insert("red".to_string(), 0);
            counts.insert("green".to_string(), 0);
            counts.insert("blue".to_string(), 0);
            for cube in reveal.split(", ") {
                let cube_parts: Vec<&str> = cube.split(" ").collect();
                if cube_parts.len() == 2 {
                    let count = cube_parts[0].parse::<i32>().unwrap();
                    let color = cube_parts[1].to_string();
                    *counts.entry(color).or_insert(0) = std::cmp::max(counts[&color], count);
                }
            }
            counts
        }).collect();
    
    (game_id, reveals)
}

fn calculate_min_cubes(reveals: &Vec<HashMap<String, i32>>) -> HashMap<String, i32> {
    let mut min_cubes = HashMap::new();
    min_cubes.insert("red".to_string(), 0);
    min_cubes.insert("green".to_string(), 0);
    min_cubes.insert("blue".to_string(), 0);

    for reveal in reveals {
        for (color, &count) in reveal {
            let entry = min_cubes.entry(color.to_string()).or_insert(0);
            *entry = std::cmp::max(*entry, count);
        }
    }
    min_cubes
}

fn calculate_power(min_cubes: &HashMap<String, i32>) -> i32 {
    min_cubes.get("red").unwrap_or(&0) * min_cubes.get("green").unwrap_or(&0) * min_cubes.get("blue").unwrap_or(&0)
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt"); // Adjust the path as needed
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_power = 0;
    for line in reader.lines() {
        let line = line?;
        let (_game_id, reveals) = parse_game_data(&line);
        let min_cubes = calculate_min_cubes(&reveals);
        total_power += calculate_power(&min_cubes);
    }

    println!("{}", total_power);
    Ok(())
}
