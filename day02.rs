use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn parse_game_data(line: &str) -> (i32, Vec<HashMap<String, i32>>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let _game_id = parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
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
    
    (_game_id, reveals)
}

fn is_game_possible(reveals: &Vec<HashMap<String, i32>>, max_cubes: &HashMap<String, i32>) -> bool {
    for reveal in reveals {
        for (color, &count) in reveal {
            if count > *max_cubes.get(color).unwrap() {
                return false;
            }
        }
    }
    true
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt"); // Adjust the path as needed
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut games = Vec::new();
    for line in reader.lines() {
        let line = line?;
        games.push(parse_game_data(&line));
    }

    let mut max_cubes = HashMap::new();
    max_cubes.insert("red".to_string(), 12);
    max_cubes.insert("green".to_string(), 13);
    max_cubes.insert("blue".to_string(), 14);

    let possible_games_sum: i32 = games.iter()
        .filter(|&&(_game_id, ref reveals)| is_game_possible(reveals, &max_cubes))
        .map(|&(_game_id, _)| _game_id)
        .sum();

    println!("{}", possible_games_sum);
    Ok(())
}
