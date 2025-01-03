use std::cmp::Ordering;
use anyhow::*;
use itertools::Itertools;
use nalgebra::max;
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
struct ClosedValves (u128);

///
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Where {
    Disabled,
    Room (ValveIndex),
    MovingTo (ValveIndex, u32),
}

#[derive(Debug, Clone, Copy)]
struct ProcessStateDuo {

    you: Where,

    elephant: Where,

    /// Total pressure delivered until the end of the allocated time given the valves opened so far
    total_pressure: u32,

    /// Time left (minutes)
    time_left: u32,

    /// Valves that are still to be opened
    to_open: ClosedValves,
}


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

impl ProcessStateDuo {
    fn new(valves: &[Valve], total_time: u32) -> ProcessStateDuo {

        // Index of the starting valve
        let valve_start = valves.iter().find_position(|&valve| valve.name == "AA");

        // set a '1' for every valve we try to open (we ignore those with zero flow). lsb <=> valve 0
        let closed = valves.iter().rev().fold(
            0,
            |acc, valve| if valve.flow > 0 { (acc << 1) +1 } else { acc << 1 }
        );

        ProcessStateDuo {
            you: Where::Room(valve_start.unwrap().0),
            elephant: Where::MovingTo(valve_start.unwrap().0, 0),
            total_pressure: 0,
            time_left: total_time,
            to_open: ClosedValves(closed),
        }

    }

    /// Resolve the case where you and the elephant are in transit, by determining which one
    /// arrives first and how long it takes. Time left is reduced by this amount and
    /// the corresponding action is changed from [Where::MovingTo] to [Where::Room].
    /// In any other case, the state is left unchanged.
    ///
    /// ## Rem
    /// When both should arrive at the same time, *you* becomes `Where::Room(_)` while the
    /// *elephant* becomes `Where::MovingTo(_, 0)`. Therefore, this function cannot yield
    /// a state where both are in the state `Where::Room(_)`.
    fn resolve_arrival (&self) -> Self {

        match (self.you, self.elephant) {

            // When both are moving, resolve which one arrives first
            (Where::MovingTo(valve_0, time_0), Where::MovingTo(valve_1, time_1)) => {
                let time_step = time_0.min (time_1);
                ProcessStateDuo {
                    you:       if time_0 <= time_1 { Where::Room(valve_0) } else { Where::MovingTo(valve_0, time_0-time_step) },
                    elephant:  if time_1 < time_0 { Where::Room(valve_1) } else { Where::MovingTo(valve_1, time_1-time_step) },
                    time_left: self.time_left - time_step,
                    .. *self
                }
            },

            (Where::MovingTo(valve, time), Where::Disabled) => {
                ProcessStateDuo {
                    you:       Where::Room(valve),
                    time_left: self.time_left - time,
                    .. *self
                }
            },

            (Where::Disabled, Where::MovingTo(valve, time)) => {
                ProcessStateDuo {
                    elephant: Where::Room(valve),
                    time_left: self.time_left - time,
                    .. *self
                }
            },

            _ => self.clone(),
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Compare two valves for ordering. Most important valves come first (highest flow).
/// Zero-flow valves are put at the end, among which the starting valve AA comes first.
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
    // yield its index and the weight to get there (1 minute)
    let fn_adjacency = |node: usize| {
        valves [node].edges.iter().map (|&adj_valve| {
            let adj_index = valves.iter().position (|valve| valve.name == adj_valve).unwrap();
            (adj_index, 1)
        })
    };

    compute_all_pair_distances(valves.len(), fn_adjacency)
}

fn solve_sequence_duo (state: &ProcessStateDuo, valves: &[Valve], distances: &DistanceMatrix) -> u32 {

    // To track the max pressure released among all the solutions
    let mut max_pressure = 0;
    let mut steps = 0;

    // DFS queue, starting in the provided `state`
    let mut dfs_queue = vec! [*state];
    while let Some (state) = dfs_queue.pop() {

        steps += 1;

        // The next action is for the one that is not in transit.
        // There can only be one (see function `resolve_arrival`)
        let (valve_start, where_other) = match (state.you, state.elephant) {
            (Where::Room(valve), o@Where::Disabled)       => (valve, o),
            (Where::Room(valve), o@Where::MovingTo(_, _)) => (valve, o),
            (o@Where::MovingTo(_, _), Where::Room(valve)) => (valve, o),
            (o@Where::Disabled,       Where::Room(valve)) => (valve, o),
            _ => unreachable!()
        };

        // Each unopened valve in this `state` is a potential action ...
        let dfs_len = dfs_queue.len();
        for valve_index in state.to_open.iter_closed() {

            // ... which requires some time to execute (move + open),
            let required_time = distances[valve_start][valve_index] + 1;
            if required_time >= state.time_left { continue }

            // This yields this `new_state`. We don't update the `time_left` now!
            // We don't bother to differentiate between you and the elephant.
            // `total_pressure` and valve closure are anticipated (nothing can prevent the action to fail).
            let time_left = state.time_left - required_time;
            let new_state = ProcessStateDuo {
                you: Where::MovingTo(valve_index, required_time),
                elephant: where_other,
                total_pressure: state.total_pressure + valves[valve_index].flow * time_left,
                time_left: state.time_left,
                to_open: state.to_open.close(valve_index),
            };

            // Track the max pressure among all the investigated solutions.
            max_pressure = max_pressure.max (new_state.total_pressure);

            // Schedule processing of the new state if some valves are still closed and if
            // the heuristic indicates potential progress
            if new_state.to_open.0 > 0 &&
                heuristic_duo (new_state, valves, distances, valve_index) > max_pressure {
                dfs_queue.push(new_state.resolve_arrival());
            }
        }

        // Case where you or the elephant could not do anything: we disable the protagonist,
        // but we propagate the state as the other one can maybe still do something
        if dfs_queue.len() == dfs_len {
            if where_other != Where::Disabled {
                let new_state = ProcessStateDuo {
                    you: Where::Disabled,
                    elephant: where_other,
                    .. state
                };
                dfs_queue.push(new_state.resolve_arrival());
            }
        }

    }

    println!("Part B Steps: {}", steps);

    max_pressure
}

fn solve_sequence (state: &ProcessState, valves: &[Valve], distances: &DistanceMatrix) -> u32 {

    let mut max_pressure = 0;
    let mut steps = 0;

    let mut dfs_queue = vec! [*state];
    while let Some (state) = dfs_queue.pop() {

        steps += 1;

        // Each unopened valve in this tate is a potential action ...
        for valve_index in state.to_open.iter_closed() {

            // ... which requires some time to execute (move + open),
            let required_time = distances [state.valve][valve_index] +1;
            if required_time >= state.time_left { continue }

            // and that yields this `new_state`
            let time_left = state.time_left - required_time;
            let new_state = ProcessState {
                valve: valve_index,
                total_pressure: state.total_pressure + valves[valve_index].flow * time_left,
                time_left,
                to_open: state.to_open.close(valve_index),
            };

            // Track the max pressure among all the investigated solutions.
            max_pressure = max_pressure.max (new_state.total_pressure);

            // Schedule processing of the new state if some valves are still closed and if
            // the heuristic indicates potential progress
            if new_state.to_open.0 > 0 &&
                heuristic(new_state, valves, distances) > max_pressure {
                dfs_queue.push(new_state)
            };
        }
    }

    println!("Part A tSteps: {}", steps);

    max_pressure
}

/// This function returns an *upper bound* of the total pressure we can reach
/// by opening the `valves` given the current `state`. This bound is computed by assuming
/// that each remaining closed valve can be reached in 1minute.
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

fn heuristic_duo (mut state: ProcessStateDuo, valves: &[Valve], distances: &DistanceMatrix, valve_index: ValveIndex) -> u32 {

    let required_time = state.to_open.iter_closed().map(
        |valve_index| distances[valve_index][valve_index]
    ).min().unwrap() +1;

    // Iterate on all the remaining closed valves, from the most interesting one to the least.
    // We assume we can move to each of those valve in one step and close it (2 minutes)
    for valve_index in state.to_open.iter_closed() {
        if state.time_left <= 2 { break }
        state.time_left -= 2;
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

    // Find the best sequence's max pressure
    let start_state = ProcessState::new(&valves, 30);
    println!("Num valves to open: {}", start_state.to_open.num_closed());
    let max_pressure = solve_sequence(&start_state, &valves, &distances);

    Ok(max_pressure as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Collect the valves from the input file and sort them
    let mut valves = collect_valves(&content)?;
    valves.sort_unstable_by(compare_valves);

    // Compute all pair distances
    let distances = compute_distance_matrix(&valves);

    let start_state = ProcessStateDuo::new(&valves, 26);
    let max_pressure = solve_sequence_duo(&start_state, &valves, &distances);

    Ok(max_pressure as usize)
}

pub fn day_16 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 1651);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 1707);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}