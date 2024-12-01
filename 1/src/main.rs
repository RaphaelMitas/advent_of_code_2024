use std::io::{self, BufRead};

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let stdin = io::stdin();
    let mut vec_1 = Vec::new();
    let mut vec_2 = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }
        
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        
        if numbers.len() == 2 {
            vec_1.push(numbers[0]);
            vec_2.push(numbers[1]);
        }
    }
    
    (vec_1, vec_2)
}

fn part_one(mut vec_1: Vec<i32>, mut vec_2: Vec<i32>) {
    let mut distance = 0;

    while !vec_1.is_empty() {
        let min_index = find_min_value_index(&vec_1);
        let min_index_2 = find_min_value_index(&vec_2);

        distance += (vec_1[min_index] - vec_2[min_index_2]).abs();
        vec_1.remove(min_index);
        vec_2.remove(min_index_2);
    }

    println!("The total distance is: {distance}");
}

fn find_min_value_index(arr: &[i32]) -> usize {
    let mut min_value = arr[0];
    let mut min_index = 0;
    for i in 0..arr.len() {
        if arr[i] < min_value {
            min_value = arr[i];
            min_index = i;
        }
    }
    min_index
}

fn part_two(vec_1: Vec<i32>, vec_2: Vec<i32>) {
    let mut score = 0;

    for i in 0..vec_1.len() {
        for j in 0..vec_2.len() {
            if vec_1[i] == vec_2[j] {
                score += vec_1[i];
            }
        }
    }

    println!("The total score is: {score}");
}

fn main() {
    println!("Please enter pairs of numbers (one pair per line).");
    println!("Press Enter twice (empty line) when done.");
    println!();
    
    let (vec_1, vec_2) = read_input();
    part_one(vec_1.clone(), vec_2.clone());
    part_two(vec_1.clone(), vec_2.clone());
}