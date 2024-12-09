use std::collections::HashMap;
use std::fmt::Display;
use anyhow::*;
use itertools;
use itertools::Itertools;
use crate::{Cell, CellArea, Solution};

const TEST: &str = "\
2333133121414131402
";

type FileId = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File(FileId),
}

type FileSystem = Vec<Block>;


/// Solve first part of the puzzle
fn part_a (content: &str) -> Result<usize> {

    Ok (1928)
}

/// Solve second part of the puzzle
fn part_b (content: &str) -> Result<usize> {

    Ok (0)
}

pub fn day_9 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&TEST).unwrap_or_default() == 1928);
    debug_assert!(part_b (&TEST).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}