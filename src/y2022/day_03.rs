use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

/// Bitset: 26 first lsb for the items 'a' to 'z', and 26 more for the items 'A' to 'Z'
type RuckSackContent = u64;

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Encode some raw `content` into a [RuckSackContent] bitset.
fn to_rucksack (content: &str) -> RuckSackContent {

    // Each of the 52 possible characters is one-hot encoded
    content.as_bytes().iter().fold (0, |content, b| {
        let offset = match b  {
            b'a'..=b'z' => b - b'a',
            b'A'..=b'Z' => b - b'A' + 26,
            _ => unreachable!()
        };

        content | (1 << offset)
    })
}

/// Assuming `rucksack` contains only one bit to 1, find its position and deduce the
/// priority of the corresponding element.
fn to_priority (rucksack: RuckSackContent) -> usize {

    let left_position = rucksack.leading_zeros() as usize;
    assert!(left_position >= 64 - 52 && left_position <= 63);

    64 - left_position
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let priorities = content.iter ().map (| row | {
        let (left, right) = row.split_at(row.len() / 2);

        // Encode the left and right part of the rucksack and find the priority of the common element
        let common = to_rucksack(left) & to_rucksack(right);
        to_priority(common)
    }).sum();

    Ok(priorities)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let priorities = content.chunks_exact(3).map (| group | {
        let a = to_rucksack(group [0]);
        let b = to_rucksack(group [1]);
        let c = to_rucksack(group [2]);
        to_priority(a & b & c)
    }).sum ();

    Ok(priorities)
}

pub fn day_3 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 157);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 70);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}