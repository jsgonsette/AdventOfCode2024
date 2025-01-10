use std::collections::HashMap;
use anyhow::*;
use crate::Solution;
use crate::tools::IntReader;

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Create two vectors from the puzzle file content
fn make_two_lists (content: &[&str]) -> Result<(Vec<usize>, Vec<usize>)> {

    let mut reader = IntReader::new(false);

    let pairs = content.iter().map_while (| row | {
        let pair: [usize; 2] = reader.process_row_fix(row)?;
        Some((pair[0], pair[1]))
    });

    let (v_left, v_right): (Vec<_>, Vec<_>) = pairs.unzip();
    if v_left.len() != content.len() || v_right.len() != content.len() {
        bail!("Cannot parse content")
    }
    else {
        Ok((v_left, v_right))
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let (mut v_left, mut v_right) = make_two_lists(content)?;
    v_left.sort_unstable();
    v_right.sort_unstable();

    let sum_diff: isize = v_left.iter().zip(v_right.iter()).map(
        |(a, b)| (*a as isize - *b as isize).abs()).sum();

    Ok(sum_diff as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let (v_left, v_right) = make_two_lists(content)?;

    // Count number of occurrences
    let mut map_count: HashMap<usize, usize> = Default::default();
    for v in v_right.iter() {
        map_count.entry(*v).and_modify(|v| *v += 1).or_insert(1);
    }

    let similarity: usize = v_left.into_iter().map(|v| {
        let num_occurrences = map_count.get(&v).copied().unwrap_or_default();
        v * num_occurrences
    }).sum();

    Ok(similarity)
}

pub fn day_1 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 11);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 31);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}