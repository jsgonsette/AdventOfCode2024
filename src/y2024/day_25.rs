use anyhow::*;
use crate::{Cell, CellArea, Solution};

const TEST: &str = "\
";

type Key = [u8; 5];

type Lock = [u8; 5];

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn load_key (rows: &[&str]) -> Key {
    let mut key = Key::default();

    for &row in rows [0..5].iter().rev() {
        for (idx, b) in row.as_bytes() [0..5].iter().enumerate() {
            match b {
                b'#' => key[idx] += 1,
                b'.' => { },
                _ => panic!(),
            }
        }
    }

    key
}

fn load_lock (rows: &[&str]) -> Lock {
    let mut lock = Lock::default();

    for &row in rows [0..5].iter() {
        for (idx, b) in row.as_bytes() [0..5].iter().enumerate() {
            match b {
                b'#' => lock[idx] += 1,
                b'.' => { },
                _ => panic!(),
            }
        }
    }

    lock
}

fn load_keys_and_locks (content: &[&str]) -> Result<(Vec<Key>, Vec<Lock>)> {

    let mut keys = Vec::<Key>::new();
    let mut locks = Vec::<Lock>::new();

    for idx in (0..content.len()).filter(|&i| i % 8 == 0) {

        if content [idx] == "....." {
            keys.push(load_key(&content [idx+1..]));
        }
        else if content[idx] == "#####" {
            locks.push(load_lock(&content [idx+1..]));
        }
        else {
            bail!("Invalid key or lock head: {}", content[idx]);
        }
    }

    Ok((keys, locks))
}


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

    let (keys, locks) = load_keys_and_locks(content)?;

    let mut num_fits = 0;

    for key in keys.iter () {
        for lock in locks.iter () {
            if fit_key_and_lock(&key, &lock) { num_fits += 1; }
        }
    }
    println!("Number of keys: {}", keys.len());
    println!("Number of locks: {}", locks.len());
    println!("Number of fits: {}", num_fits);

    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_25 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}