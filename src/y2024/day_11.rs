use std::collections::{HashMap};
use anyhow::*;
use crate::{Solution};
use crate::tools::RowReader;

const TEST: &str = "125 17";

/// Return the number of digits in a number (in base 10)
fn num_digits (number: usize) -> u32 {
    number.ilog10()+1
}

/// Return `true` if the number as an even number of digits
fn num_digits_even (number: usize) -> bool {
    num_digits(number) % 2 == 0
}

/// Split a number in two, assuming they have an even number of digits
/// Ex: 1275 -> (12, 75)
fn split_digits (number: usize) -> (usize, usize) {
    let num_digits = num_digits (number);
    let modulo = 10usize.pow(num_digits/2);
    let right = number % modulo;
    let left = number / modulo;

    (left, right)
}

/// Determine the number of children of some ancestor Pebbles `number` after
/// some number of steps `num_blinks`. This value is returned immediately if
/// we already know it (`memo`). Otherwise, it is computed by calling function `count_children`.
fn get_or_count_children(memo: &mut Memoization, number: usize, num_blinks: u8) -> usize {

    if let Some (count) = memo.get_count(number, num_blinks) { count }
    else {
        let count = count_children(memo, number, num_blinks);
        memo.insert_count(number, num_blinks, count);
        count
    }
}

/// Determine the number of children of some ancestor Pebbles `number` after
/// some number of steps `num_blinks`. This value is determined by making
/// the pebble evolve by one blink, then by computing *recursively* the number
/// of children of the one or two pebbles it has yielded.
///
/// **Table `memo` avoids recomputing things we have already computed in the past**
fn count_children (memo: &mut Memoization, number: usize, num_blinks: u8) -> usize {

    // Stop if no more blinks allowed
    if num_blinks == 0 { return 1 }

    // Evolve
    let (left, right) = match number {
        0                        => (1, None),
        x if num_digits_even (x) => {
            let (left, right) = split_digits(x);
            (left, Some(right))
        },
        x                        => (x*2024, None),
    };

    // Recurse
    let count_left = get_or_count_children(memo, left, num_blinks -1);
    let count_right = if let Some(right) = right { get_or_count_children(memo, right, num_blinks -1) } else { 0 };
    count_left + count_right
}

/// Memoization of the number of children of pebbles after some blinks
struct Memoization {
    table: HashMap<(usize, u8), usize>,
}

impl Memoization {

    fn new() -> Memoization {
        Self {
            table: HashMap::new(),
        }
    }

    /// Insert the number of children `num_children` resulting from evolving the
    /// pebble `number` by `step_count` blinks.
    fn insert_count (&mut self, number: usize, step_count: u8, num_children: usize) {
        self.table.insert((number, step_count), num_children);
    }

    /// Get the number of children resulting from evolving the
    /// pebble `number` by `step_count` blinks, if we know it.
    fn get_count (&self, number: usize, step_count: u8) -> Option<usize> {
        self.table.get(&(number, step_count)).copied()
    }
}

/// Solve the puzzle for a given number of steps `num_blinks`
fn solve (row: &str, num_blinks: u8) -> Result<usize> {

    let mut reader = RowReader::new();
    let mut numbers_it = reader.iter_row(row);
    let mut memo = Memoization::new();

    let length = numbers_it.map(
        |n| count_children (&mut memo, n, num_blinks)
    ).sum();

    Ok(length)
}

pub fn day_11 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (TEST, 25).unwrap_or_default() == 55312);

    let ra = solve(content [0], 25)?;
    let rb = solve(content [0], 75)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}