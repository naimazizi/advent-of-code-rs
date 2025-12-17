// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use std::collections::{hash_map::Entry, HashMap};

use crate::utils;
use anyhow::Result;

// Example template.

pub fn solve() -> Result<()> {
    // Load your input file.
    let input = utils::load_input(2016, 1)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 1 / Year 2016");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// Helper functions to get direction vectors from input
fn get_direction(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input.split(',').map(|s| s.trim()).filter_map(|dir| {
        let mut chars = dir.chars();
        let turn_char = chars.next()?;
        let distance_str = chars.as_str();
        let distance = match distance_str.parse::<i32>() {
            Ok(d) => d,
            Err(_) => return None,
        };
        let turn = match turn_char {
            'R' | 'r' => 1,
            'L' | 'l' => -1,
            _ => return None,
        };
        Some((turn, distance))
    })
}

fn calc_manhattan_distance(pos: (i32, i32)) -> i32 {
    pos.0.abs() + pos.1.abs()
}

fn update_direction(current: (i32, i32), turn: i32) -> (i32, i32) {
    match (current, turn) {
        ((0, 1), 1) => (1, 0),    // N to E
        ((0, 1), -1) => (-1, 0),  // N to W
        ((1, 0), 1) => (0, -1),   // E to S
        ((1, 0), -1) => (0, 1),   // E to N
        ((0, -1), 1) => (-1, 0),  // S to W
        ((0, -1), -1) => (1, 0),  // S to E
        ((-1, 0), 1) => (0, 1),   // W to N
        ((-1, 0), -1) => (0, -1), // W to S
        _ => current,
    }
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut direction = (0, 1); // North, assuming it's x,y coords with y increasing northwards
    let mut position = (0, 0);

    let direction_vectors = get_direction(input);

    for (turn, distance) in direction_vectors {
        // Update direction
        direction = update_direction(direction, turn);
        // Move position
        position.0 += direction.0 * distance;
        position.1 += direction.1 * distance;
    }

    // Calculate Manhattan distance
    let manhattan_distance = calc_manhattan_distance(position);
    Ok(manhattan_distance)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut direction = (0, 1); // North
    let mut position = (0, 0);
    let mut visited_positions: HashMap<(i32, i32), bool> = HashMap::new();
    visited_positions.insert(position, true);

    let direction_vectors = get_direction(input);
    let mut manhattan_distance = 0;

    for (turn, distance) in direction_vectors {
        // Update direction
        direction = update_direction(direction, turn);
        // Move position
        for _i in 0..distance {
            position.0 += direction.0;
            position.1 += direction.1;
            if let Entry::Vacant(e) = visited_positions.entry(position) {
                e.insert(true);
            } else {
                // Found first location visited twice
                manhattan_distance = calc_manhattan_distance(position);
                return Ok(manhattan_distance);
            }
        }
    }

    Ok(manhattan_distance)
}
