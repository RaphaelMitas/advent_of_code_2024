use std::io::{self, BufRead};

fn read_input() -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    let mut levels: Vec<Vec<i32>> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }
        
        let level: Vec<i32> = line.split_whitespace().map(|s| s.parse().expect("Failed to parse number")).collect();
        levels.push(level);
    }

    levels
}

fn part_one(levels: Vec<Vec<i32>>) -> i32 {
    let mut total_safe = 0;
    
    for level in levels {
        let mut is_unsafe = false;
        let mut is_increasing: Option<bool> = None;

        for i in 0..level.len()-1 {
            let first = level[i];
            let second = level[i + 1];
            let diff = second - first;
            
            if diff == 0 {
                is_unsafe = true;
                break;
            }

            if is_increasing.is_none() {
                is_increasing = Some(diff > 0);
            }

            if (is_increasing.unwrap() && (diff <= 0 || diff > 3)) ||
               (!is_increasing.unwrap() && (diff >= 0 || diff < -3)) {
                is_unsafe = true;
                break;
            }
        }

        if !is_unsafe {
            total_safe += 1;
        }
    }

    total_safe
}

fn part_two(levels: Vec<Vec<i32>>) -> i32 {
    let mut total_safe = 0;

    for level in levels {
        let mut is_safe = false;
        for i in 0..level.len() {
            let mut shortened_level = level.clone();
            shortened_level.remove(i);
            if part_one(vec![shortened_level]) == 1 {
                is_safe = true;
                break;
            }
        }
        if is_safe {
            total_safe += 1;
        }
    }

    total_safe
}

fn main() {
    let levels = read_input();
    let total_safe = part_one(levels.clone());
    println!("total safe: {}", total_safe);

    let total_safe_with_tolerance = part_two(levels.clone());
    println!("total safe with tolerance: {}", total_safe_with_tolerance);
}
