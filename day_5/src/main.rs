use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input file");
    let result1 = solve_part1(&input);
    let result2 = solve_part2(&input);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}

fn solve_part1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;

    for update in updates {
        if is_valid_order(&update, &rules) {
            let middle_idx = update.len() / 2;
            sum += update[middle_idx];
        }
    }

    sum
}

fn solve_part2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;

    for update in updates {
        if !is_valid_order(&update, &rules) {
            let sorted = sort_update(&update, &rules);
            let middle_idx = sorted.len() / 2;
            sum += sorted[middle_idx];
        }
    }

    sum
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut parts = input.split("\n\n");
    let rules_str = parts.next().unwrap();
    let updates_str = parts.next().unwrap();

    // Parse rules into a HashMap where key must come before all values in the set
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in rules_str.lines() {
        if line.is_empty() { continue; }
        let mut parts = line.split('|');
        let before = parts.next().unwrap().parse::<i32>().unwrap();
        let after = parts.next().unwrap().parse::<i32>().unwrap();
        rules.entry(before).or_default().insert(after);
    }

    // Parse updates
    let updates: Vec<Vec<i32>> = updates_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn is_valid_order(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    // For each pair of numbers in the update
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let before = update[i];
            let after = update[j];

            // Check if there's a rule saying after should come before before
            if rules.get(&after).map_or(false, |set| set.contains(&before)) {
                return false;
            }
        }
    }
    true
}

fn sort_update(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut result = update.to_vec();
    
    // Bubble sort with custom comparison based on rules
    for i in 0..result.len() {
        for j in 0..result.len() - 1 - i {
            let a = result[j];
            let b = result[j + 1];
            
            // If b should come before a according to rules, swap them
            if rules.get(&b).map_or(false, |set| set.contains(&a)) {
                result.swap(j, j + 1);
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(solve_part1(input), 143);
    }

    #[test]
    fn test_example_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(solve_part2(input), 123);
    }
}
