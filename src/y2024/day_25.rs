use anyhow::*;
use crate::Solution;

/// A key, with its height in 5 positions
type Key = [u8; 5];

/// A Lock, with its height in 5 positions
type Lock = [u8; 5];

/// Given the 5 first chars in `row`, increment the `heights` at the corresponding
/// positions for each encountered `#`.
fn inc_height_from_row_it (heights: &mut [u8; 5], row: &str) {
    for (idx, b) in row.as_bytes() [0..5].iter().enumerate() {
        match b {
            b'#' => heights[idx] += 1,
            b'.' => { },
            _ => panic!(),
        }
    }
}

/// Load a [Key] from its description in the puzzle file content `rows`.
fn load_key (rows: &[&str]) -> Key {

    let mut key = Key::default();
    for &row in rows [0..5].iter().rev() {
        inc_height_from_row_it (&mut key, row);
    }

    key
}

/// Load a [Lock] from its description in the puzzle file content `rows`.
fn load_lock (rows: &[&str]) -> Lock {
    let mut lock = Lock::default();

    for &row in rows [0..5].iter() {
        inc_height_from_row_it (&mut lock, row);
    }

    lock
}

/// Loads [Key]s and [Lock]s from the puzzle file content
fn load_keys_and_locks (content: &[&str]) -> Result<(Vec<Key>, Vec<Lock>)> {

    let mut keys = Vec::<Key>::new();
    let mut locks = Vec::<Lock>::new();

    // Each key or lock is exactly 7 rows height, + one empty line in between
    for idx in (0..content.len()).filter(|&i| i % 8 == 0) {

        // A key starts with this empty pattern
        if content [idx] == "....." {
            keys.push(load_key(&content [idx+1..]));
        }
        // A lock starts with this plain pattern
        else if content[idx] == "#####" {
            locks.push(load_lock(&content [idx+1..]));
        }
        // Anything else is an error
        else {
            bail!("Invalid key or lock head: {}", content[idx]);
        }
    }

    Ok((keys, locks))
}

/// Check if a key and a lock fit with each other, (there must have no overlap)
fn fit_key_and_lock (key: &Key, lock: &Lock) -> bool {

    if key [0] + lock [0] > 5 { false }
    else if key [1] + lock [1] > 5 { false }
    else if key [2] + lock [2] > 5 { false }
    else if key [3] + lock [3] > 5 { false }
    else if key [4] + lock [4] > 5 { false }
    else { true }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    // Load the keys and locks
    let (keys, locks) = load_keys_and_locks(content)?;

    // Count the number of fits
    let mut num_fits = 0;
    for key in keys.iter () {
        for lock in locks.iter () {
            if fit_key_and_lock(&key, &lock) { num_fits += 1; }
        }
    }

    Ok(num_fits)
}

pub fn day_25 (content: &[&str]) -> Result <(Solution, Solution)> {

    let ra = part_a(content)?;
    Ok((Solution::Unsigned(ra), Solution::Unsigned(0)))
}