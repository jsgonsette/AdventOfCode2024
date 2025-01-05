use std::cmp::{Ordering, PartialEq};
use std::collections::HashSet;
use anyhow::*;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use crate::Solution;
use crate::tools::{Coo, RowReader};

const TEST: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[derive(Debug, Copy, Clone)]
struct Pair {
    sensor: Coo,
    beacon: Coo,
}

/// Implements the interval [a, b] with Natural numbers. Bounds are inclusive.
#[derive(Debug, Copy, Clone, PartialEq)]
struct IntInterval(isize, isize);

/// Implements a set (union) of disjoint intervals
#[derive(Debug, Clone)]
struct IntIntervals {

    /// Disjoint intervals, ordered from left to right
    intervals: Vec<IntInterval>,
}

impl PartialOrd for IntInterval {

    /// Partial comparison with other intervals.
    /// The ordering returns `none` in case of overlap.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1 < other.0 { Some (Ordering::Less) }
        else if self.0 > other.1 { Some (Ordering::Greater) }
        else if *self == *other { Some (Ordering::Equal) }
        else { None }
    }
}

impl IntInterval {

    fn from (x: isize, dist_y: isize, radius: isize) -> Option<Self> {
        assert!(dist_y >= 0 && radius >= 1);
        let half_width = radius - dist_y;

        (half_width >= 0).then(
            || IntInterval(x - half_width, x + half_width)
        )
    }

    /// Returns `true` if this interval overlaps with `other`
    fn overlap_with (&self, other: &IntInterval) -> bool {
        !(other.1 < self.0 || other.0 > self.1)
    }

    /// If this interval overlaps with `other`, then returns a single interval covering both of them.
    fn union(&self, other: &IntInterval) -> Option<IntInterval> {
        self.overlap_with(other).then(||
            IntInterval(self.0.min(other.0), self.1.max(self.1))
        )
    }

    /// If this interval overlaps with `other`, then returns the interval common to both of them.
    fn intersection(&self, other: &IntInterval) -> Option<IntInterval> {
        self.overlap_with(other).then(||
            IntInterval(self.0.max(other.0), self.1.min(self.1))
        )
    }
}

impl IntIntervals {

    /// New empty set of intervals
    fn new () -> Self {
        IntIntervals { intervals: vec![] }
    }

    /// Returns the total length covered by the underlying intervals
    fn length(&self) -> usize {
        self.intervals.iter().map(|inter| (inter.1 - inter.0 +1) as usize).sum()
    }

    /// Returns `true` if x lays in one of the underlying intervals
    fn contains(&self, x: isize) -> bool {
        self.intervals.iter().any(|inter| inter.0 <= x && inter.1 >= x)
    }

    /// Add `interval` to this set.
    fn add(&mut self, interval: IntInterval) {

        // Index of first part that is not strictly < than `interval`
        let insertion_start = self.intervals.partition_point(
            |other| *other < interval
        );

        // Index of first part strictly > than `interval`
        let insertion_end = self.intervals [insertion_start..]
            .iter()
            .fold_while(insertion_start, |count, other| {
                if interval.overlap_with(other) { Continue (count + 1) } else { Done (count) }
            }).into_inner();

        // Fuse everything between `insertion_start` and `insertion_end` with `interval`
        let fused = self.intervals[insertion_start..insertion_end]
            .iter()
            .fold(interval, |fused, other| {
                fused.union(other).unwrap()
            });

        // All the parts strictly > than `interval`
        let tail: Vec<_> = self.intervals[insertion_end..].iter().copied().collect();

        self.intervals.truncate(insertion_start);
        self.intervals.push(fused);
        self.intervals.extend(tail);
    }

    fn intersection (&self, other: &Self) -> Self {

        let mut idx_other = 0;

        let iter_it = self.intervals.iter().flat_map (|inter| {
            let (skipped, intersection) =
                Self::intersection_single(inter, &other.intervals [idx_other..]);
            idx_other += skipped;
            intersection
        });

        IntIntervals {
            intervals: iter_it.collect(),
        }
    }

    fn intersection_single<'a> (inter: &'a IntInterval, parts: &'a [IntInterval]) -> (usize, impl Iterator<Item = IntInterval> + 'a) {

        let mut skipped = 0usize;

        let skip_before_it = parts
            .iter()
            .skip_while(move | &other_inter | {
                if *other_inter < *inter { *(&mut skipped) += 1; true } else { false }
            });

        let common_it = skip_before_it
            .take_while(move | other_inter | {
                *(&mut skipped) += 1;
                inter.overlap_with(other_inter)
            });

        (
            skipped.saturating_sub(1),
            common_it.flat_map (| other_inter | inter.intersection(other_inter))
        )
    }
}

impl Pair {

    /// Returns the Manhattan distance between the beacon and its sensor
    fn distance (&self) -> isize {
        (self.beacon.x - self.sensor.x).abs () + (self.beacon.y - self.sensor.y).abs()
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Collect all the beacon-sensor pairs from the puzzle file `content`
fn collect_device_pairs (content: &[&str]) -> Result<Vec<Pair>> {

    let mut reader = RowReader::new(true);

    content.iter().map (|&row| {
        let raw: [isize; 4] = reader.process_row_fix(row)
            .ok_or(anyhow!("could not parse row {}", row))?;

        Ok (Pair {
            sensor: Coo::from((raw [0], raw [1])),
            beacon: Coo::from((raw [2], raw [3])),
        })
    }).collect()
}

fn compute_row_intervals (row_y: isize, pairs: &[Pair]) -> IntIntervals {

    // Process each beacon-device pair
    let mut intervals = IntIntervals::new();
    for pair in pairs.iter() {

        // For each pair, we can construct an interval covering the positions where the beacon cannot be
        let interval = IntInterval::from (
            pair.sensor.x,
            (pair.sensor.y - row_y).abs(),
            pair.distance()
        );

        // Fuse all those intervals together
        if let Some (inter) = interval {
            intervals.add(inter);
        }
    }

    intervals
}

/// Solve first part of the puzzle
fn part_a (content: &[&str], target_row: isize) -> Result<usize> {

    let pairs = collect_device_pairs (content)?;

    // Compute the coverage at row `target_row` by processing each beacon-sensor pair
    let intervals = compute_row_intervals(target_row, &pairs);

    // Beacons on the target rows
    let beacon_on_rows = HashSet::<isize>::from_iter(
        pairs.iter ().filter_map (
            |p| (p.beacon.y == target_row).then (
                || intervals.contains(p.beacon.x).then(|| p.beacon.x)
            ).flatten()
        )
    );

    Ok(intervals.length() - beacon_on_rows.len ())
}

///     BBBB
///     BBBB
///     BBBB
///   AABBBCCC
///   AAAA.CCC
///   AAADDDCC
///      DDD
///      DDD

/// Solve second part of the puzzle, **slowly**.
/// The idea here just consist in testing all the 4.10^6 possible rows, one by one,
/// the same way as in part 1
fn part_b (content: &[&str]) -> Result<usize> {

    let pairs = collect_device_pairs (content)?;

    for y in 0..= 4000000 {
        let intervals = compute_row_intervals(y, &pairs);

        // The lonely beacon must be surrounded by 2 plain intervals
        if intervals.intervals.len() == 2 {
            let x = intervals.intervals [0].1 +1;
            let tuning_freq = x * 4000000 + y;
            return Ok(tuning_freq as usize);
        }
    }

    Err(anyhow!("no solution found"))
}

pub fn day_15 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST), 10).unwrap_or_default() == 26);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 56000011);

   /* let a = IntIntervals::new();
    let b = IntIntervals::new();
    let c = a.intersection(&b);

    println!("C: {:?}", c.intervals);*/

    let ra = part_a(content, 2000000)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}