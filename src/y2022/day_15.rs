use std::collections::{HashMap, HashSet};
use anyhow::*;
use itertools::Itertools;
use num::Integer;
use crate::Solution;
use crate::tools::{Coo, IntInterval, IntIntervals, IntReader};

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

impl Pair {

    /// Returns the Manhattan distance between the beacon and its sensor
    fn distance_to_beacon(&self) -> isize {
        (self.beacon.x - self.sensor.x).abs () + (self.beacon.y - self.sensor.y).abs()
    }

    /// Manhattan distance between the sensor and a given `coo`
    fn distance_to (&self, coo: Coo) -> isize {
        (coo.x - self.sensor.x).abs () + (coo.y - self.sensor.y).abs()
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Collect all the beacon-sensor pairs from the puzzle file `content`
fn collect_device_pairs (content: &[&str]) -> Result<Vec<Pair>> {

    let mut reader = IntReader::new(true);

    content.iter().map (|&row| {
        let raw: [isize; 4] = reader.process_row_fix(row)
            .ok_or(anyhow!("could not parse row {}", row))?;

        Ok (Pair {
            sensor: Coo::from((raw [0], raw [1])),
            beacon: Coo::from((raw [2], raw [3])),
        })
    }).collect()
}

/// For a given row `row_y`, iterate on all the sensor-beacon pairs and build the coverage
/// where the lost beacon cannot be. This coverage is modeled with [IntIntervals].
fn compute_row_intervals (row_y: isize, pairs: &[Pair]) -> IntIntervals {

    // Process each beacon-device pair
    let mut intervals = IntIntervals::new();
    for pair in pairs.iter() {

        // For each pair, we can construct an interval covering the positions where
        // the beacon cannot be on the row `row_y`
        let x = pair.sensor.x;
        let dist_y = (pair.sensor.y - row_y).abs();
        let radius = pair.distance_to_beacon();

        let half_width = radius - dist_y;
        let interval = (half_width >= 0).then_some(
            IntInterval(x - half_width, x + half_width)
        );

        // Fuse all those intervals together
        if let Some (inter) = interval {
            intervals.union_single(inter);
        }
    }

    intervals
}

/// Given a diamond shape `#` at some `center` location and `radius`
/// ((3,2) and 1 in the example below),
/// this function returns the location of the **outside** top left diagonal edge of this diamond (`^`).
/// This edge is given by:
/// * the y-intercept of the diagonal (1 in this example)
/// * the covering interval along x ((1-3) in this example)
/// ```
///  y
///  |   ^
///  |  ^#
///  2 ^###
///  |   #
///  |
///  +---3-- x
/// ```
fn diamond_outside_top_left (center: Coo, radius: usize) -> (isize, IntInterval) {

    let y_outside_top = center.y + radius as isize +1;
    let intercept = y_outside_top - center.x;
    let x1 = center.x - radius as isize -1;
    let x2 = center.x;

    (intercept, IntInterval(x1, x2))
}

/// This function is similar to [diamond_outside_top_left],
/// but returns the top right outside edge instead
fn diamond_outside_top_right (center: Coo, radius: usize) -> (isize, IntInterval) {

    let y_outside_top = center.y + radius as isize +1;
    let intercept = y_outside_top + center.x;
    let x2 = center.x + radius as isize +1;
    let x1 = center.x;

    (intercept, IntInterval(x1, x2))
}

/// This function is similar to [diamond_outside_top_left],
/// but returns the bottom right outside edge instead
fn diamond_outside_bottom_right (center: Coo, radius: usize) -> (isize, IntInterval) {

    let y_outside_bottom = center.y - radius as isize -1;
    let intercept = y_outside_bottom - center.x;
    let x2 = center.x + radius as isize +1;
    let x1 = center.x;

    (intercept, IntInterval(x1, x2))
}

/// This function is similar to [diamond_outside_top_left],
/// but returns the bottom left outside edge instead
fn diamond_outside_bottom_left (center: Coo, radius: usize) -> (isize, IntInterval) {

    let y_outside_bottom = center.y - radius as isize -1;
    let intercept = y_outside_bottom + center.x;
    let x1 = center.x - radius as isize -1;
    let x2 = center.x;

    (intercept, IntInterval(x1, x2))
}

/// Compute the unique coordinate (if any), resulting from the intersection between:
/// * a line of equation y =  x + `b1` whose valid x values are given by `intervals_1`
/// * a line of equation y = -x + `b2` whose valid x values are given by `intervals_2`
fn orthogonal_intersection (
    b1: isize,
    intervals_1: &IntIntervals,
    b2: isize,
    intervals_2: &IntIntervals
) -> Option<Coo> {

    let _2x = b2 - b1;
    _2x.is_even().then(|| {
        let x = _2x / 2;
        let y = x + b1;

        (intervals_1.contains(x) && intervals_2.contains(x)).then_some(Coo::from((x, y)))
    }).flatten()
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

/// The second part of the puzzle is solved here by first noticing that there are not so many
/// beacon-sensor pairs. this means it's better to iterate on this set than on the million rows.
///
/// Secondly, the lonely beacon must be squeezed between the edges of at least 4 diamond shapes
/// (a diamond shape is the space where we know the beacon cannot be in. There is one such
/// diamond by beacon-sensor pair.). For example, consider below 4 diamonds `A, B, C, D` and
/// the lonely beacon `.` in between.
///
/// ```
///       C
///      CCCB
///      ACBBB
///     AAA.BD
///    AAAAADDD
///     AAADDDDD
///      A  DDD
/// ```
///
/// The outside edges of the diamonds can be described by the intercepts of orthogonal
/// lines y = x +b or y = -x +b and by valid range values along those lines. We know that
/// the beacon is along one of those edges. Therefore:
///
/// 1) Top-left edges with the same intercept are union in the same [IntIntervals], and the
/// same can be done for the other edges.
/// 2) By intersecting top-left and bottom right intervals that have the same intercept, we
/// reduce the search space. Same for top-right and bottom left intervals.
/// 3) We can further narrow down the search space by looking at the intersections of those
/// orthogonal location intervals.
fn part_b (content: &[&str]) -> Result<usize> {

    let pairs = collect_device_pairs (content)?;

    // Location candidates for the lost beacon. It must be stuck between (at least) 4 diamonds.
    let mut tl_outside = HashMap::<isize, IntIntervals>::new ();
    let mut tr_outside = HashMap::<isize, IntIntervals>::new ();
    let mut bl_outside = HashMap::<isize, IntIntervals>::new ();
    let mut br_outside = HashMap::<isize, IntIntervals>::new ();

    // Each pair generates a diamond shape of some `center` and `radius`
    for pair in pairs.iter() {
        let center = pair.sensor;
        let radius = pair.distance_to_beacon();

        // Get a description of the outside top left edge of this diamond (intercept and interval).
        // Then, union this interval with all other ones belonging to the same intercept.
        // The content of those intervals are location candidates for the lost beacon.
        let (intercept, interval) = diamond_outside_top_left(center, radius as usize);
        tl_outside.entry(intercept).or_insert(IntIntervals::new()).union_single(interval);

        // Same with top right, bottom left and bottom right edges.
        let (intercept, interval) = diamond_outside_top_right(center, radius as usize);
        tr_outside.entry(intercept).or_insert(IntIntervals::new()).union_single(interval);

        let (intercept, interval) = diamond_outside_bottom_left(center, radius as usize);
        bl_outside.entry(intercept).or_insert(IntIntervals::new()).union_single(interval);

        let (intercept, interval) = diamond_outside_bottom_right(center, radius as usize);
        br_outside.entry(intercept).or_insert(IntIntervals::new()).union_single(interval);
    }

    // For each intercept, compute the intersection between the top left and
    // bottom right edges intervals. This reduces the number of potential locations
    let it_diag_1 = tl_outside.iter ().filter_map(|(intercept, tl_interval)| {
        br_outside.get (intercept).and_then(|br_interval| {
            let common = tl_interval.intersection(br_interval);
            (common.num_disjoints() > 0).then_some((*intercept, common))
        })
    });

    // Same for top right and bottom left intervals
    let it_diag_2 = tr_outside.iter ().filter_map(|(intercept, tr_interval)| {
        bl_outside.get (intercept).and_then(|bl_interval| {
            let common = tr_interval.intersection(bl_interval);
            (common.num_disjoints () > 0).then_some((*intercept, common))
        })
    });

    // tr/bl location candidates are orthogonal to the tl/br ones. As the lost beacon
    // must belong to both, test all the possible intersections
    for ((b1, intervals_1), (b2, intervals_2)) in it_diag_1.cartesian_product(it_diag_2) {

        // Test the line y = x + b1 with the line y = -x + b2
        if let Some (coo) = orthogonal_intersection(b1, &intervals_1, b2, &intervals_2) {

            // If we have a candidate, we must now check this is not a false positive.
            // For that we simply check the distance with all the beacon-sensor pairs.
            if pairs.iter().all (
                |pair| pair.distance_to(coo) > pair.distance_to_beacon()
            ) {
                let tuning_freq = coo.x * 4000000 + coo.y;
                return Ok(tuning_freq as usize);
            }
        }
    }

    Err(anyhow!("no solution found"))
}

/// Solve second part of the puzzle, **slowly**.
/// The idea here just consists in testing all the 4.10^6 possible rows, one by one,
/// the same way as in part 1. See function [part_b] for a better way.
fn part_b_slow (content: &[&str]) -> Result<usize> {

    let pairs = collect_device_pairs (content)?;

    for y in 0..= 4000000 {
        let intervals = compute_row_intervals(y, &pairs);

        // The lonely beacon must be surrounded by 2 plain intervals
        if intervals.num_disjoints() == 2 {
            let x = intervals [0].1 +1;
            let tuning_freq = x * 4000000 + y;
            return Ok(tuning_freq as usize);
        }
    }

    Err(anyhow!("no solution found"))
}

pub fn day_15 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST), 10).unwrap_or_default() == 26);
    debug_assert!(part_b_slow (&split(TEST)).unwrap_or_default() == 56000011);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 56000011);

    let ra = part_a(content, 2000000)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}