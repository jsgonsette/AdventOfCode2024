use std::collections::{HashMap, HashSet};
use anyhow::*;
use itertools::{Itertools};
use crate::{Solution};
use crate::tools::RowReader;

const TEST: &str = "\
1
10
100
2024";

const TEST_2: &str = "\
1
2
3
2024";


/// A sequence of 4 price increases
type Sequence = (i8, i8, i8, i8);

/// Best total price for each [Sequence]
type SequencePrice = HashMap<Sequence, u32>;

/// Banana sell price
type Price = u8;


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Load the monkey seeds from the puzzle file content
fn load_seeds (content: &[&str]) -> Result<Vec<usize>> {

    let mut reader= RowReader::new(false);

    content.iter().map(|&row| {
        let raw: [usize; 1] = reader.process_row_fix(row)
            .ok_or(anyhow!("Invalid seed: {}", row))?;

        Ok (raw[0])
    }).collect()
}

/// Compute the next secret number from an initial `seed` value
fn secret_step(seed: usize) -> usize {

    let mix = |secret: usize, value: usize| { secret ^ value };
    let prune = |secret: usize| { secret % 16777216 };

    let step_1 = | secret: usize | { prune (mix (secret, secret << 6)) };
    let step_2 = | secret: usize | { prune (mix (secret, secret >> 5)) };
    let step_3 = | secret: usize | { prune (mix (secret, secret << 11)) };

    step_3 (step_2 (step_1 (seed)))
}

/// Return an iterator on the next 2000 price increases
fn price_increase_it (seed: usize) -> impl Iterator<Item=(Price, i8)> {

    // The price is the last digit
    let price = | seed: usize | { (seed % 10) as i8 };

    let mut secret = seed;
    (0..2000).map (move |_| {
        let new_secret = secret_step(secret);
        let increase = price (new_secret) - price (secret);
        secret = new_secret;

        (price (new_secret) as u8, increase)
    })
}

/// Return an iterator on sequences of four price increase, with the associated sell price.
fn four_changes_it (seed: usize) -> impl Iterator<Item=(Price, Sequence)> {

    let price_it = price_increase_it(seed)
        .map (|(price, _increase)| price)
        .skip(3);

    let four_seq_increase_it = price_increase_it(seed)
        .map(|(_price, increase)| increase)
        .tuple_windows::<Sequence>();

    price_it.zip(four_seq_increase_it)
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    // Load the seeds
    let monkey_seeds = load_seeds(content)?;

    // Sum the 2000th generated secret for each seed
    let mut sum = 0;
    for seed in monkey_seeds {
        let secret_2000 = (0..2000).fold(seed, |secret, _| secret_step(secret));
        sum += secret_2000;
    }

    Ok(sum)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Load the seeds
    let seeds = load_seeds(content)?;

    // Save the best price for each sequence, and the best price overall
    let mut best_prices = SequencePrice::new();
    let mut best_price = 0;

    // For each monkey seed
    for seed in seeds {

        // Keep track of sequences we have already seen (we can sell only once)
        let mut seq_dones = HashSet::<Sequence>::new();

        // for each associated sequence
        for (price, sequence) in four_changes_it(seed) {

            // Skip if seen already
            if seq_dones.contains(&sequence) { continue; }
            seq_dones.insert(sequence);

            // Increase the price of this sequence
            let entry = best_prices.entry(sequence).or_insert(0);
            *entry += price as u32;
            best_price = best_price.max(*entry);
        }
    }

    Ok(best_price as usize)
}

pub fn day_22 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 37327623);
    debug_assert!(part_b (&split(TEST_2)).unwrap_or_default() == 23);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}