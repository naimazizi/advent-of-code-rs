// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use crate::utils;
use anyhow::Result;

// Example template.

pub fn solve() -> Result<()> {
    // Load your input file.
    let input = utils::load_input(2015, 2)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 2 / Year 2015");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn get_ranges(input: &str) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
    input
        .split(',')
        .map(|pair| pair.split('x'))
        .filter_map(|mut split| {
            let x = split.next()?.trim().parse::<i32>().ok()?;
            let y = split.next()?.trim().parse::<i32>().ok()?;
            let z = split.next()?.trim().parse::<i32>().ok()?;
            Some((x, y, z))
        })
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let ranges = get_ranges(input);
    ranges.for_each(|(x, y, z)| {
        println!("x: {}, y: {}, z: {}", x, y, z);
    });
    Ok(0)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let result = solve_part1("2x3x4,1x1x10").unwrap();
    }
}
