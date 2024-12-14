use anyhow::*;
use crate::Solution;
use crate::tools::RowReader;

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

type Level = usize;

/// Checks that a sequence of values is safe
struct SafetyChecker {
    is_increasing: Option<bool>,
    previous: Option<Level>,
}

impl SafetyChecker {
    pub fn new() -> SafetyChecker {
        SafetyChecker {
            is_increasing: None,
            previous: None,
        }
    }

    /// Return `true` if the new provided item is safe regarding the previous value
    pub fn process_next (&mut self, item: Level) -> bool {
        let is_safe = match (self.previous, self.is_increasing) {
            (None, _) => {
                true
            },
            (Some(first), None) => {
                self.is_increasing = Some (item > first);
                let delta = item as isize - first as isize;
                delta.abs() >= 1 && delta.abs() <= 3
            }
            (Some (p), Some (is_increasing)) => {
                let delta = item as isize - p as isize;
                let delta_increase = delta > 0;
                delta.abs() >= 1 && delta.abs() <= 3 && delta_increase == is_increasing
            }
        };

        self.previous = Some(item);
        is_safe
    }
}

/// Check that the sequence of values `levels` is safe
fn is_safe<'a, I> (levels: I) -> bool
where I: IntoIterator<Item = &'a Level> {

    let mut checker = SafetyChecker::new();
    levels.into_iter().all(| value | {
        checker.process_next(*value)
    })
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut reader = RowReader::new(false);

    let sum_or_err: Result<Vec<usize>> = content.iter().map(|row| {

        let levels = reader.process_row(row);
        Ok (if is_safe (&levels) { 1usize } else { 0 })
    }).collect ();

    sum_or_err.map (|x| x.iter().copied ().sum())
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let mut reader = RowReader::new(false);
    let sum = content.iter().map(|row| {

        let levels = reader.process_row(row);
        let n = levels.len ();

        let completely_safe = is_safe (&levels);

        let almost_safe = (0..n).any (|idx| {
           let one_off = levels.iter().take (idx).chain(
               levels.iter().skip(idx+1)
           );
           is_safe (one_off)
        });

        if completely_safe || almost_safe { 1 } else { 0 }
    }).sum();

    Ok(sum)
}

pub fn day_2 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 2);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 4);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}