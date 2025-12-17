// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use crate::utils;
use anyhow::Result;

// Example template.

pub fn solve() -> Result<()> {
    // Load your input file.
    let input = utils::load_input(2015, 1)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 1 / Year 2015");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let iter_chars =
        input.chars().enumerate().fold(0, |acc, (_i, c)| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        });
    Ok(iter_chars)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let result = input.chars().enumerate().try_fold(
        (0, 1),
        |(cumulative, idx), (_i, c)| {
            let next = match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            if cumulative == 0 && next == -1 {
                return Err((cumulative, idx));
            }

            Ok((cumulative + next, idx + 1))
        },
    );
    match result {
        Ok((_cumulative, _idx)) => Ok(0),
        Err((_cumulative, idx)) => Ok(idx),
    }
}
