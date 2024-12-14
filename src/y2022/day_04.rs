use anyhow::*;
use crate::Solution;
use crate::tools::RowReader;

const TEST: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

#[derive(Copy, Clone, Debug)]
struct Range (u32, u32);

impl Range {
    fn is_contained_in (&self, other: &Self) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    fn overlap (&self, other: &Self) -> bool {
        self.1 >= other.0 && self.0 <= other.1
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn read_ranges (row: &str) -> Result<(Range, Range)> {

    let mut reader = RowReader::new ();
    let ranges: [usize; 4] = reader.process_row_fix(row)
        .ok_or(anyhow!("Error reading line {row}"))?;

    let left = Range (ranges [0] as u32, ranges [1] as u32);
    let right = Range (ranges [2] as u32, ranges [3] as u32);
    Ok((left, right))
}

fn part_a (content: &[&str]) -> anyhow::Result<usize> {

    let mut count = 0;

    for row in content.iter() {
        let (left_range, right_range) = read_ranges(row)?;
        if left_range.is_contained_in(&right_range) || right_range.is_contained_in(&left_range) {
            count += 1;
        }
    }

    Ok(count)
}

fn part_b (content: &[&str]) -> anyhow::Result<usize> {

    let mut count = 0;

    for row in content.iter() {
        let (left_range, right_range) = read_ranges(row)?;
        if left_range.overlap(&right_range) {
            count += 1;
        }
    }

    Ok(count)
}

pub fn day_4(content: &[&str]) -> anyhow::Result<(Solution, Solution)> {

    debug_assert!(part_a(&split(TEST)).unwrap_or_default() == 2);
    debug_assert!(part_b(&split(TEST)).unwrap_or_default() == 4);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}