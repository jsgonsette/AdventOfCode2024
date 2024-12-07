use anyhow::*;
use crate::Solution;

const TEST: &str = "\
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn part_a (content: &[&str]) -> Result<usize> {
    Ok (0)
}

fn part_b (content: &[&str]) -> Result<usize> {
    Ok (0)
}

pub fn day_8 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//solve(content, false)?;
    let rb = 0;//solve(content, true)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}