use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    guard_pos: Position,
    guard_dir: Direction,
    start_pos: Position,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut guard_pos = Position { row: 0, col: 0 };
        let mut guard_dir = Direction::Up;

        // Find guard's starting position and direction
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '^' {
                    guard_pos = Position { row: i as i32, col: j as i32 };
                    guard_dir = Direction::Up;
                }
            }
        }

        Map {
            grid,
            guard_pos,
            guard_dir,
            start_pos: guard_pos,
        }
    }

    fn is_within_bounds(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < self.grid.len() as i32 &&
        pos.col >= 0 && pos.col < self.grid[0].len() as i32
    }

    fn has_obstacle(&self, pos: &Position) -> bool {
        if !self.is_within_bounds(pos) {
            return true;
        }
        self.grid[pos.row as usize][pos.col as usize] == '#'
    }

    fn simulate_guard_path(&mut self) -> usize {
        let mut visited = HashSet::new();
        visited.insert(self.guard_pos);

        loop {
            // Check position in front of guard
            let (delta_row, delta_col) = self.guard_dir.get_delta();
            let next_pos = Position {
                row: self.guard_pos.row + delta_row,
                col: self.guard_pos.col + delta_col,
            };

            if !self.is_within_bounds(&next_pos) {
                break;
            }

            if self.has_obstacle(&next_pos) {
                // Turn right if there's an obstacle
                self.guard_dir = self.guard_dir.turn_right();
            } else {
                // Move forward
                self.guard_pos = next_pos;
                visited.insert(self.guard_pos);
            }
        }

        visited.len()
    }

    fn count_loop_causing_positions(&self) -> usize {
        // First, calculate the guard's normal path - O(M)
        let mut normal_path = Vec::new();
        let mut visited_states = HashSet::new();
        let mut guard_pos = self.start_pos;
        let mut guard_dir = Direction::Up;
        
        // Store the complete path until guard exits or loops
        loop {
            let state = (guard_pos, guard_dir);
            if visited_states.contains(&state) {
                break;
            }
            normal_path.push(state);
            visited_states.insert(state);

            let (delta_row, delta_col) = guard_dir.get_delta();
            let next_pos = Position {
                row: guard_pos.row + delta_row,
                col: guard_pos.col + delta_col,
            };

            if !self.is_within_bounds(&next_pos) {
                break;
            }

            if self.has_obstacle(&next_pos) {
                guard_dir = guard_dir.turn_right();
            } else {
                guard_pos = next_pos;
            }
        }

        // Collect empty positions
        let positions: Vec<Position> = (0..self.grid.len())
            .flat_map(|row| {
                (0..self.grid[0].len()).filter_map(move |col| {
                    if self.grid[row][col] == '.' {
                        Some(Position { row: row as i32, col: col as i32 })
                    } else {
                        None
                    }
                })
            })
            .collect();

        // Process positions in parallel
        let count = positions.par_iter()
            .enumerate()
            .map(|(_checked, &pos)| {

                // For each position, check if it intersects with the normal path
                for &(path_pos, path_dir) in &normal_path {
                    let (delta_row, delta_col) = path_dir.get_delta();
                    let next_pos = Position {
                        row: path_pos.row + delta_row,
                        col: path_pos.col + delta_col,
                    };

                    if next_pos == pos {
                        // This position would cause the guard to turn right
                        return would_create_loop(path_pos, path_dir.turn_right(), pos, &self.grid);
                    }
                }
                false
            })
            .filter(|&creates_loop| creates_loop)
            .count();

        count
    }
}

fn would_create_loop(start_pos: Position, start_dir: Direction, obstacle_pos: Position, grid: &Vec<Vec<char>>) -> bool {
    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;
    let max_steps = grid.len() * grid[0].len() * 4;

    for _ in 0..max_steps {
        let state = (pos, dir);
        if visited.contains(&state) {
            return true;
        }
        visited.insert(state);

        let (delta_row, delta_col) = dir.get_delta();
        let next_pos = Position {
            row: pos.row + delta_row,
            col: pos.col + delta_col,
        };

        if !is_within_bounds(&next_pos, grid) {
            return false;
        }

        if has_obstacle(&next_pos, grid) || next_pos == obstacle_pos {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
        }
    }
    false
}

fn is_within_bounds(pos: &Position, grid: &Vec<Vec<char>>) -> bool {
    pos.row >= 0 && pos.row < grid.len() as i32 &&
    pos.col >= 0 && pos.col < grid[0].len() as i32
}

fn has_obstacle(pos: &Position, grid: &Vec<Vec<char>>) -> bool {
    if !is_within_bounds(pos, grid) {
        return true;
    }
    grid[pos.row as usize][pos.col as usize] == '#'
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input file");
    
    // Part 1
    let mut map = Map::from_input(&input);
    let result1 = map.simulate_guard_path();
    println!("Part 1 - Number of distinct positions visited: {}", result1);

    // Part 2
    let map = Map::from_input(&input);
    let result2 = map.count_loop_causing_positions();
    println!("Part 2 - Number of possible loop-causing positions: {}", result2);
}
