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

    /// Return `true` if `other` is completely contained in this range
    fn is_contained_in (&self, other: &Self) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    /// Return `true` if `other` overlaps with this range
    fn overlap (&self, other: &Self) -> bool {
        self.1 >= other.0 && self.0 <= other.1
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Iterate on each pair of [Range] at each row of the puzzle file `content`
fn get_range_it<'a> (content: &'a [&'a str]) -> impl Iterator<Item = Result<(Range, Range)>> + 'a {

    let mut reader = RowReader::new (false);
    content.iter().map (move |row| {
        let range_numbers: Vec<u32> = reader.iter_row::<u32>(row).collect();
        let left = Range (range_numbers [0], range_numbers [1]);
        let right = Range (range_numbers [2], range_numbers [3]);

        Ok((left, right))
    })
}

fn part_a (content: &[&str]) -> Result<usize> {

    let mut count = 0;
    for result in get_range_it(content) {

        let (left, right) = result?;
        if left.is_contained_in(&right) || right.is_contained_in(&left) { count += 1 }
    }

    Ok(count)
}

fn part_b (content: &[&str]) -> Result<usize> {

    let mut count = 0;
    for result in get_range_it(content) {

        let (left, right) = result?;
        if left.overlap(&right) { count += 1 }
    }

    Ok(count)
}

pub fn day_4(content: &[&str]) -> Result<(Solution, Solution)> {

    debug_assert!(part_a(&split(TEST)).unwrap_or_default() == 2);
    debug_assert!(part_b(&split(TEST)).unwrap_or_default() == 4);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}