use std::fs;
use std::io::Write;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn extract_numbers(line: &str) -> Vec<i64> {
    let line = line.replace(":", "");
    let numbers: Vec<i64> = line.split_whitespace()
        .map(|s| {
            match s.parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Failed to parse number '{}': {}", s, e);
                    0
                }
            }
        })
        .collect();
    numbers
}

fn find_valid_operations(expected_result: i64, operations: Vec<i64>, result: i64) -> i64 {
    let mut valid_operations = 0;
    if operations.len() == 0 {
        if result == expected_result {
            valid_operations = 1;
        }
        return valid_operations;
    }
    let mut operations = operations.clone();

    let last_operation = operations.remove(0);
    let valid_mul_operation = find_valid_operations(expected_result, operations.clone(), result * last_operation);
    let valid_add_operation = find_valid_operations(expected_result, operations, result + last_operation);

   
   valid_mul_operation + valid_add_operation
}

fn find_valid_operations_part_2(expected_result: i64, operations: Vec<i64>, result: i64) -> i64 {
    
    let mut valid_operations = 0;
    if operations.len() == 0 {
        if result == expected_result {
            valid_operations = 1;
        }
        return valid_operations;
    }
    let mut operations = operations.clone();

    let last_operation = operations.remove(0);

    let valid_mul_operation = find_valid_operations_part_2(expected_result, operations.clone(), result * last_operation);
    let valid_add_operation = find_valid_operations_part_2(expected_result, operations.clone(), result + last_operation);
    let valid_concat_operation = find_valid_operations_part_2(expected_result, operations, (result.to_string() + &last_operation.to_string()).parse::<i64>().unwrap());

   valid_mul_operation + valid_add_operation + valid_concat_operation
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input file");
    let mut p1_total_valid_operations = 0;
    let mut p1_total_result = 0;
    let mut p2_total_valid_operations = 0;
    let mut p2_total_result = 0;

    let lines: Vec<_> = input.lines().collect();
    let number_of_lines = lines.len();
    let processed_lines = AtomicUsize::new(0);
    let results: Vec<_> = lines.par_iter().enumerate().map(|(_i, line)| {
        let numbers = extract_numbers(line);
        let p1_number_of_valid_operations = find_valid_operations(numbers[0], numbers[1..].to_vec(), 0);
        let p2_number_of_valid_operations = find_valid_operations_part_2(numbers[0], numbers[1..].to_vec(), 0);
        
        let current = processed_lines.fetch_add(1, Ordering::Relaxed);
        print!("\rprocessed: {}%", current * 100 / number_of_lines);
        std::io::stdout().flush().unwrap();

        (numbers[0], p1_number_of_valid_operations, p2_number_of_valid_operations)
    }).collect();

    for (number, p1_ops, p2_ops) in results {
        p1_total_valid_operations += p1_ops;
        if p1_ops > 0 {
            p1_total_result += number;
        }

        p2_total_valid_operations += p2_ops;
        if p2_ops > 0 {
            p2_total_result += number;
        }
    }
    println!();

    println!("part 1: total_valid_operations: {}", p1_total_valid_operations);
    println!("part 1: total_result: {}", p1_total_result);
    println!("part 2: total_valid_operations: {}", p2_total_valid_operations);
    println!("part 2: total_result: {}", p2_total_result);

}
