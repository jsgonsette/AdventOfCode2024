use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {
    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {
    Ok(0)
}

pub fn day_11 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split (TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split (TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content, false)?;
    let rb = 0;//part_b(content, true)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}