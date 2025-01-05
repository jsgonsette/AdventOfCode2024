use std::cmp::Ordering;
use anyhow::*;
use itertools::Itertools;
use crate::{Solution};
use crate::tools::{compute_all_pair_distances};

const TEST: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

/// Characteristic of a valve in the puzzle input
#[derive(Debug, Clone)]
struct Valve<'a> {

    /// Valve's name
    name: &'a str,

    /// Flow
    flow: u32,

    /// Other valves it is connected to
    edges: Vec<&'a str>,
}

/// Distance matrix giving what time is required to navigate between each pair of valves
type DistanceMatrix = Vec<Vec<u32>>;

/// Index of a valve
type ValveIndex = usize;

/// Bit vector of closed valves
#[derive(Debug, Copy, Clone)]
struct ClosedValves (u64);

/// Models the current state of the investigation process
#[derive(Debug, Clone, Copy)]
struct ProcessState {

    /// Current valve where we are
    valve: ValveIndex,

    /// Total pressure delivered until the end of the allocated time given the valves opened so far
    total_pressure: u32,

    /// Time left (minutes)
    time_left: u32,

    /// Valves that are still to be opened
    to_open: ClosedValves,
}

impl ClosedValves {

    /// Return a copy of this instance where the given valve `index` is closed
    fn close (&self, index: ValveIndex) -> ClosedValves {
        let mask = !(1 << index);
        ClosedValves(self.0 & mask)
    }

    /// Return an iterator on the indexes of the valves that are closed and need to be opened
    fn iter_closed(&self) -> impl Iterator<Item=ValveIndex> + '_ {

        let end = 128 - self.0.leading_zeros() as usize;
        let start = self.0.trailing_zeros() as usize;
        let mut mask = 1 << start;

        (start..end).filter_map(move |bit_index| {
            let out = match self.0 & mask {
                0 => None,
                _ => Some(bit_index),
            };
            mask <<= 1;
            out
        })
    }

    fn num_closed(&self) -> u32 {
        self.0.count_ones()
    }
}


impl ProcessState {

    /// Initial process state when starting from valve AA with some exploration budget `time_left`
    fn new(valves: &[Valve], total_time: u32) -> ProcessState {

        // Index of the starting valve
        let valve_start = valves.iter().find_position(|&valve| valve.name == "AA");

        // set a '1' for every valve we try to open (we ignore those with zero flow). lsb <=> valve 0
        let closed = valves.iter().rev().fold(
            0,
            |acc, valve| if valve.flow > 0 { (acc << 1) +1 } else { acc << 1 }
        );

        ProcessState {
            valve: valve_start.unwrap().0,
            total_pressure: 0,
            time_left: total_time,
            to_open: ClosedValves(closed),
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Compare two valves for ordering. Most important valves come first (highest flow).
/// Zero-flow valves are put at the end
fn compare_valves (v1: &Valve, v2: &Valve) -> Ordering {
    v2.flow.cmp(&v1.flow)
        .then(v1.name.cmp(&v2.name))
}

/// Collect the description of all the valves, given the puzzle file `content`
fn collect_valves<'a> (content: &'a [&'a str]) -> Result<Vec<Valve<'a>>> {

    content.iter().map(|&row| {

        let mut upper_and_digit_tokens: Vec<&str> =
            row.split(|c: char| !c.is_ascii_digit() && !c.is_ascii_uppercase())
            .filter(|row| !row.is_empty())
            .collect();

        Ok(Valve {
            name: upper_and_digit_tokens [1],
            flow: upper_and_digit_tokens [2].parse()?,
            edges: upper_and_digit_tokens.drain(3..).collect(),
        })
    }).collect()
}

/// Compute a [DistanceMatrix] giving the distance between each pair of valves. The matrix
/// indexing is the same as the one used in the `valves` vector (e.g. ```matrix [2][7]```
/// gives the shortest distance when going from valve 2 to valve 7)
fn compute_distance_matrix (valves: &[Valve]) -> DistanceMatrix {

    // For some `node` index, iterates on the adjacent valves. For each of them we
    // yield its index and the weight to get there (1 minute in this puzzle)
    let fn_adjacency = |node: usize| {
        valves [node].edges.iter().map (|&adj_valve| {
            let adj_index = valves.iter().position (|valve| valve.name == adj_valve).unwrap();
            (adj_index, 1)
        })
    };

    compute_all_pair_distances(valves.len(), fn_adjacency)
}

/// Find the best sequence with a *Branch and Bound* algorithm, implemented with a DFS queue.
/// Given an initial `state`, the `valves` input and the `distance` matrix, explores the
/// different possible sequences and drop them early when not promising.
///
/// A sequence (and all its children) is dropped when it *most optimistic bound* is below
/// *the best solution* so far. The optimistic bound is given by a [heuristic]. The best solution
/// is tracked and given by the function `f_save_score_and_get_high`.
fn solve_sequence<F> (
    state: &ProcessState,
    valves: &[Valve],
    distances: &DistanceMatrix,
    mut f_save_score_and_get_high: F
) where F: FnMut(ClosedValves, u32) -> u32 {

    let mut dfs_queue = vec! [*state];
    while let Some (state) = dfs_queue.pop() {

        // Each unopened valve in this tate is a potential action ...
        for valve_index in state.to_open.iter_closed() {

            // ... which requires some time to execute (move + open),
            let required_time = distances [state.valve][valve_index] +1;
            if required_time >= state.time_left { continue }

            // and that yields this `new_state`. Total released pressure is anticipated,
            let time_left = state.time_left - required_time;
            let new_state = ProcessState {
                valve: valve_index,
                total_pressure: state.total_pressure + valves[valve_index].flow * time_left,
                time_left,
                to_open: state.to_open.close(valve_index),
            };

            // Track the max pressure among all the investigated solutions.
            let highest_pressure =
                f_save_score_and_get_high (new_state.to_open, new_state.total_pressure);

            // Schedule processing of the new state if some valves are still closed and if
            // the heuristic indicates potential progress against the best solution so far
            if new_state.to_open.0 > 0 &&
                heuristic(new_state, valves, distances) > highest_pressure {
                dfs_queue.push(new_state)
            };
        }
    }
}

/// This function returns an *upper bound* of the total pressure we can reach
/// by opening the `valves` given the current `state`. This bound is computed by assuming
/// that each remaining closed valve can be reached swiftly in sequence.
fn heuristic (mut state: ProcessState, valves: &[Valve], distances: &DistanceMatrix) -> u32 {

    let required_time = state.to_open.iter_closed().map(
        |valve_index| distances[state.valve][valve_index]
    ).min().unwrap() +1;

    // Iterate on all the remaining closed valves, from the most interesting one to the least.
    // We assume we can move to each of those valve in one step and close it (2 minutes)
    for valve_index in state.to_open.iter_closed() {
        if state.time_left <= required_time { break }
        state.time_left -= required_time;
        state.total_pressure += valves[valve_index].flow * state.time_left;
    }

    state.total_pressure
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    // Collect the valves from the input file and sort them
    let mut valves = collect_valves(&content)?;
    valves.sort_unstable_by(compare_valves);

    // Compute all pair distances
    let distances = compute_distance_matrix(&valves);

    // Simple function to track the best solution investigated by function `solve_sequence`
    let mut highest_pressure = 0;
    let f_save_score_and_get_high = |_closed_valves: ClosedValves, score: u32| {
        highest_pressure = highest_pressure.max (score);
        highest_pressure
    };

    // Find the best sequence's max pressure
    let start_state = ProcessState::new(&valves, 30);
    solve_sequence(&start_state, &valves, &distances, f_save_score_and_get_high);

    Ok(highest_pressure as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Collect the valves from the input file and sort them
    let mut valves = collect_valves(&content)?;
    valves.sort_unstable_by(compare_valves);

    // Compute all pair distances
    let distances = compute_distance_matrix(&valves);

    // Initial state and number of valves to close (hopefully, not so many)
    let start_state = ProcessState::new(&valves, 26);
    let num_valves_to_close = start_state.to_open.num_closed();
    let num_sequences = 2usize.pow(num_valves_to_close);
    let mask = num_sequences -1;

    // We keep track of one score (total pressure released) for each
    // possible combination of open/close valves
    let mut all_seq_scores = vec! [0; num_sequences];
    let f_save_score_and_get_high = |closed_valves: ClosedValves, score: u32| {
        let seq_index = closed_valves.0 as usize;
        all_seq_scores [seq_index] = all_seq_scores [seq_index].max (score);
        all_seq_scores [seq_index]
    };

    // Solve for each combination of opened valves and sort them from lowest to highest scores
    solve_sequence(&start_state, &valves, &distances, f_save_score_and_get_high);

    let mut sorted_seq: Vec<(usize, u32)> = all_seq_scores.iter().copied ().enumerate().collect();
    sorted_seq.sort_unstable_by_key(|(_idx, score)| *score);

    // Search the best duo among 2 complementary sequences (no overlap of opened valves)
    let mut highest_pressure = 0;
    for (closed_1, score_1) in sorted_seq.iter().rev() {
        for (closed_2, score_2) in sorted_seq.iter().rev() {

            // Because scores are sorted, we can exit early
            if *score_1 + *score_2 < highest_pressure { break; }
            if !(*closed_1) & !(*closed_2) & mask == 0 {
                highest_pressure = highest_pressure.max (*score_1 + *score_2);
                break;
            }
        }
    }

    Ok (highest_pressure as usize)
}

pub fn day_16 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 1651);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 1707);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}