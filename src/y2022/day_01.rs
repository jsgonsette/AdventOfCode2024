use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

type Calories = u32;

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn group_by_elves (content: &[&str]) -> Vec<Calories> {

    let mut sum = 0;
    let terminator = std::iter::once (&"");

    let elves: Vec<u32> = content.iter().chain (terminator).filter_map(| &row | {
        match row.is_empty() {
            true => {
                let elf = sum;
                sum = 0;
                Some (elf)
            },
            false => {
                sum += row.parse::<u32> ().unwrap();
                None
            }
        }
    }).collect ();

    elves
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut elves = group_by_elves(content);
    elves.sort_unstable();

    let max_calories = elves.iter().rev ().take(1).sum::<u32>();
    Ok (max_calories as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let mut elves = group_by_elves(content);
    elves.sort_unstable();

    let max_calories = elves.iter().rev ().take(3).sum::<u32>();
    Ok (max_calories as usize)
}

pub fn day_1 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 24000);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 45000);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}