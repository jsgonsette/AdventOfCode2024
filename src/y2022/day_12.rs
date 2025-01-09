use anyhow::*;
use crate::{Cell, CellArea, Solution};

const TEST: &str = "\
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}


/// Solve first part of the puzzle
fn part_a (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_12 (_content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}