use std::cmp::Ordering;
use std::ops::Index;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

/// Implements the interval [a, b] with Natural numbers. Bounds are inclusive.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IntInterval(pub isize, pub isize);

/// Implements a set (union) of disjoint intervals
#[derive(Debug, Clone)]
pub struct IntIntervals {

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

    /// Returns `true` if this interval overlaps with `other`
    pub fn overlap_with (&self, other: &IntInterval) -> bool {
        !(other.1 < self.0 || other.0 > self.1)
    }

    /// If this interval overlaps with `other`, then returns a single interval covering both of them.
    pub fn union(&self, other: &IntInterval) -> Option<IntInterval> {
        self.overlap_with(other).then_some(
            IntInterval(self.0.min(other.0), self.1.max(other.1))
        )
    }

    /// If this interval overlaps with `other`, then returns the interval common to both of them.
    pub fn intersection(&self, other: &IntInterval) -> Option<IntInterval> {
        self.overlap_with(other).then(||
            IntInterval(self.0.max(other.0), self.1.min(other.1))
        )
    }
}

impl Index<usize> for IntIntervals {
    type Output = IntInterval;

    fn index(&self, index: usize) -> &Self::Output {
        self.intervals.index(index)
    }
}

impl IntIntervals {

    /// New empty set of intervals
    pub fn new () -> Self {
        IntIntervals { intervals: vec![] }
    }

    /// Returns the total length covered by the underlying intervals
    pub fn length(&self) -> usize {
        self.intervals.iter().map(|inter| (inter.1 - inter.0 +1) as usize).sum()
    }

    pub fn num_disjoints(&self) -> usize {
        self.intervals.len()
    }

    /// Returns `true` if x lays in one of the underlying intervals
    pub fn contains(&self, x: isize) -> bool {
        self.intervals.iter().any(|inter| inter.0 <= x && inter.1 >= x)
    }

    /// Add `interval` to this set, fusing it with existing elements as and when needed.
    pub fn union_single(&mut self, interval: IntInterval) {

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

    pub fn intersection (&self, other: &Self) -> Self {

        // To skip elements of `other` when they could not possibly be part of the solution
        let mut idx_other = 0;

        // Iterate on each element of this interval. Each element can produce multiple
        // intersections that are flatten together
        let iter_it = self.intervals.iter().flat_map (|inter| {

            // Compute an iterator yielding the intersection parts.
            let (skipped, intersection) =
                Self::intersection_single(inter, &other.intervals [idx_other..]);
            idx_other += skipped;
            intersection
        });

        IntIntervals {
            intervals: iter_it.collect(),
        }
    }

    /// Returns an iterator yielding intervals corresponding to the intersection of the
    /// single interval `inter` with the vector `parts`
    fn intersection_single<'a> (inter: &'a IntInterval, parts: &'a [IntInterval]) -> (usize, impl Iterator<Item = IntInterval> + 'a) {

        let mut skipped = 0usize;

        // Skip parts strictly below `inter`
        let skip_before_it = parts
            .iter()
            .skip_while(move | &other_inter | {
                if *other_inter < *inter { *(&mut skipped) += 1; true } else { false }
            });

        // Interact with each remaining overlapping elements
        let common_it = skip_before_it
            .take_while(move | other_inter | {
                *(&mut skipped) += 1;
                inter.overlap_with(other_inter)
            });

        (
            skipped.saturating_sub(1), // We can skip every used part, except the last one
            common_it.flat_map (| other_inter | inter.intersection(other_inter))
        )
    }
}
