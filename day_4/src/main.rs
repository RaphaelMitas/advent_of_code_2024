use std::io::{self, BufRead};

fn read_input() -> Vec<Vec<char>> {
    let stdin = io::stdin();
    let mut lines: Vec<Vec<char>> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }
        
        let chars: Vec<char> = line.chars().collect();
        lines.push(chars);
    }

    lines
}



fn part_one(input: Vec<Vec<char>>, check_word: &str) -> i32 {
    let mut total_matches = 0;

    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == check_word.chars().nth(0).unwrap() {
                //right
                if check_word.len() <= input[x].len()-y {
                    let word = input[x][y..y+check_word.len()].to_vec();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
                //left
                if y >= check_word.len()-1 {
                    let word = input[x][y-(check_word.len()-1)..y+1].to_vec().into_iter().rev().collect::<Vec<char>>();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
                //up
                if x >= check_word.len() {
                    let word: Vec<char> = (0..check_word.len())
                        .map(|i| input[x-i][y])
                        .collect();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
                //down
                if x+check_word.len() <= input.len() {
                    let word: Vec<char> = (0..check_word.len())
                        .map(|i| input[x+i][y])
                        .collect();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
                //diagonal right down
                if x+check_word.len() <= input.len() && y+check_word.len() <= input[x].len() {
                    let word: Vec<char> = (0..check_word.len())
                        .map(|i| input[x+i][y+i])
                        .collect();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
                //diagonal left down
                if x+check_word.len() <= input.len() && y >= check_word.len()-1 {
                    let word: Vec<char> = (0..check_word.len())
                        .map(|i| input[x+i][y-i])
                        .collect();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
                //diagonal right up
                if x >= check_word.len()-1 && y+check_word.len() <= input[x].len() {
                    let word: Vec<char> = (0..check_word.len())
                        .map(|i| input[x-i][y+i])
                        .collect();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
                //diagonal left up
                if x >= check_word.len()-1 && y >= check_word.len()-1 {
                    let word: Vec<char> = (0..check_word.len())
                        .map(|i| input[x-i][y-i])
                        .collect();
                    if word == check_word.chars().collect::<Vec<char>>() {
                        total_matches += 1;
                    }
                }
            }
        }
    }

    total_matches
}

fn part_two(input: Vec<Vec<char>>, check_word: &str) -> i32 {
    let mut total_matches = 0;
    let middle_letter = check_word.chars().nth(check_word.len()/2).unwrap();
    
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == middle_letter && x>=1 && y>=1 && x+1<input.len() && y+1<input[x].len() {
                let mut matches_on_this_letter = 0;
                //check diagonal right down
                let word = (0..check_word.len())
                    .map(|i| input[x+i-1][y+i-1])
                    .collect::<Vec<char>>();
                if word == check_word.chars().collect::<Vec<char>>() {
                    matches_on_this_letter += 1;
                }

                //check diagonal left down
                let word = (0..check_word.len())
                    .map(|i| input[x+i-1][y+1-i])
                    .collect::<Vec<char>>();
                if word == check_word.chars().collect::<Vec<char>>() {
                    matches_on_this_letter += 1;
                }

                //check diagonal right up
                let word = (0..check_word.len())
                    .map(|i| input[x+1-i][y+i-1])
                    .collect::<Vec<char>>();
                if word == check_word.chars().collect::<Vec<char>>() {
                    matches_on_this_letter += 1;
                }

                //check diagonal left up
                let word = (0..check_word.len())
                    .map(|i| input[x+1-i][y+1-i])
                    .collect::<Vec<char>>();
                if word == check_word.chars().collect::<Vec<char>>() {
                    matches_on_this_letter += 1;
                }

                if matches_on_this_letter == 2 {
                    total_matches += 1;
                }
            }
        }
    }

    total_matches
}

fn main() {
    let input = read_input();
    const CHECK_WORD: &str = "XMAS";
    const CHECK_WORD_TWO: &str = "MAS";
    let total_matches = part_one(input.clone(), CHECK_WORD);
    println!("Total matches: {}", total_matches);
    let total_matches_two = part_two(input.clone(), CHECK_WORD_TWO);
    println!("Total matches two: {}", total_matches_two);
}
