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

    fn simulate_with_extra_obstacle(&mut self, obstacle_pos: Position) -> bool {
        if obstacle_pos == self.start_pos {
            return false;
        }

        let mut visited_states = HashSet::with_capacity(self.grid.len() * self.grid[0].len());
        let max_iterations = self.grid.len() * self.grid[0].len() * 4;
        let mut iterations = 0;

        // Reset guard position and direction to start
        self.guard_pos = self.start_pos;
        self.guard_dir = Direction::Up;

        while iterations < max_iterations {
            iterations += 1;
            let state = (self.guard_pos, self.guard_dir);
            
            if visited_states.contains(&state) {
                return true;
            }
            visited_states.insert(state);

            let (delta_row, delta_col) = self.guard_dir.get_delta();
            let next_pos = Position {
                row: self.guard_pos.row + delta_row,
                col: self.guard_pos.col + delta_col,
            };

            if !self.is_within_bounds(&next_pos) {
                return false;
            }

            if self.has_obstacle(&next_pos) || next_pos == obstacle_pos {
                self.guard_dir = self.guard_dir.turn_right();
            } else {
                self.guard_pos = next_pos;
            }
        }
        false
    }

    fn count_loop_causing_positions(&self) -> usize {
        let mut count = 0;
        let total_positions = self.grid.len() * self.grid[0].len();
        let mut checked = 0;
        
        // Pre-allocate the positions to check
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

        let total_to_check = positions.len();
        println!("Total positions to check: {}", total_to_check);

        for pos in positions {
            checked += 1;
            if checked % 100 == 0 {
                println!("Checked {}/{} positions... (Found {} loops)", checked, total_to_check, count);
            }
            
            let mut map_clone = self.clone();
            if map_clone.simulate_with_extra_obstacle(pos) {
                count += 1;
            }
        }
        count
    }
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
