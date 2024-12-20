use std::collections::HashMap;
use anyhow::*;
use crate::Solution;

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

/// A pattern as a sequence of ASCII bytes
type Pattern<'a> = &'a [u8];

/// Collection of patterns
type Patterns<'a> = Vec<Pattern<'a>>;

/// A design as a sequence of ASCII bytes
type Design<'a> = &'a [u8];

/// Collection of designs
type Designs<'a> = Vec<Design<'a>>;

/// Memoization table to avoid resolving the same sub-problems again and again.
/// For each design, save the number of possibilities
type Memo<'a> = HashMap<Design<'a>, usize>;


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Get all the patterns from the puzzle file `content`
fn get_patterns<'a> (content: &'a[&'a str]) -> Result<Patterns<'a>> {
    Ok(
        content [0].split(", ")
            .map(|s| s.as_bytes())
            .collect()
    )
}

/// Get all the designs from the puzzle file `content`
fn get_designs<'a> (content: &'a[&'a str]) -> Result<Designs<'a>> {
    Ok (
        content.iter()
            .skip(2)
            .map(|s| s.as_bytes())
            .collect()
    )
}

/// Generate a shorter design by removing the provided `pattern` from
/// the beginning of the given `design`
fn stripped_design<'a> (design: Design<'a>, pattern: Pattern<'a>) -> Design<'a> {
    &design [pattern.len()..]
}

/// Return `true` if the provided `design` is solvable given the available `patterns`.
///
/// **This function is recursive**
fn can_solve (design: Design, patterns: &Patterns) -> bool {

    // An empty design is solvable by definition
    if design.is_empty() { return true }

    // Try all the patterns matching the beginning of the design,
    // then check if the stripped design is solvable
    patterns.iter()
        .filter(|pattern| design.starts_with(pattern))
        .any (|pattern| {
            can_solve (stripped_design(design, pattern), patterns)
        })
}


/// Count the number of ways a `design` can be made given the available `patterns`.
/// Parameter `memo` is used to save solutions of intermediate sub-problems
///
/// **This function is recursive**
fn count_possibilities<'a> (memo: &mut Memo<'a>, design: Design<'a>, patterns: &Patterns<'a>) -> usize {

    // Check if we already know the answer
    if design.is_empty() { return 1 }
    if let Some (count) = memo.get(design) {
        return *count;
    }

    // Try all the patterns matching the beginning of the design.
    // For each of them, get the number of possibilities for the design remaining part
    let mut tot_count = 0;
    for pattern in patterns.iter ().filter(|pat| design.starts_with(pat)) {
        let count = count_possibilities(memo, stripped_design(design, pattern), patterns);
        tot_count += count;
    }

    // Save the result
    memo.insert(design, tot_count);

    tot_count
}

/// Solve first part of the puzzle
fn part_a (_content: &[&str]) -> Result<usize> {

    // Load the patterns and the design to reproduce
    let patterns = get_patterns(_content)?;
    let designs = get_designs(_content)?;

    // Filter and count the number of solvable designs
    let count = designs.iter()
        .filter(
            |design| can_solve(design, &patterns)
        ).count();

    Ok(count)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Load the patterns and the design to reproduce
    let patterns = get_patterns(content)?;
    let designs = get_designs(content)?;

    // Use the memoization technique to avoid resolving sub-problems we have already seen
    let mut memo = Memo::new();

    // Count and sum the number of possibilities for each pattern
    let count:usize = designs.iter()
        .map(
            |design| count_possibilities(&mut memo, design, &patterns)
        ).sum();

    Ok(count)
}

pub fn day_19 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 6);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 16);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}