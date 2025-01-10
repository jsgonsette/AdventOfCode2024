use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use anyhow::*;
use itertools::Itertools;
use crate::{Solution};
use crate::tools::IntReader;

const TEST: &str = "\
029A
980A
179A
456A
379A";

/// Digit code representation, as 3-digits and as value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Code {
    digits: [u8; 3],
    value: u32,
}

/// An entry on the numerical keypad
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum NumericalEntry {
    Digit (u8),
    Activate,
}

/// An entry on the directional keypad
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DirectionalEntry {
    Left,
    Right,
    Up,
    Down,
    Activate,
}

/// A single movement between a pair of directional entries
type StartDest = (DirectionalEntry, DirectionalEntry);

/// Memoize on the movements, for different depth of robot indirections
type MemoKey = (StartDest, usize);

/// Memoization of the best sequence length, for different movements and indirection depths
type Memo = HashMap<MemoKey, usize>;

/// Models the *numerical* keypad with the position of the robot's arm manipulating it
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct NumericalKeypad {
    pos: NumericalEntry
}

/// Models a *directional* keypad with the position of the robot's arm manipulating it
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DirectionalKeypad {
    pos: DirectionalEntry
}

/// Given an initial `from` coordinate on a key pad, and a target coordinate `to`, compute
/// the two shortest-path sequence we have to consider to navigate in between:
/// * First sequence: all-vertical, then all-horizontal (e.g, from 0 to 9: `^^^>A`)
/// * Second sequence: all-horizontal, then all-vertical (e.g, from 0 to 9: `>^^^A`)
///
/// There are more than 2 shortest-path sequences, but others could never be part of any global
/// solution. The reason is that, due to upper level of indirection (through robots), breaking
/// a sequence like `>^^^` in something like `^>^^` becomes highly
/// inefficient: At the upper level, the robot would have to navigate on `>` then come back to `^`
/// without benefiting of positions where no movement is required.
///
/// **The two sequences returned by this function include the final activation that is required
/// to actually press the button.**
fn get_raw_sequences_from_coordinates (from: (u8, u8), to: (u8, u8))
    -> [impl Iterator<Item = DirectionalEntry>; 2] {

    // Compute the delta between the pair of keypad coordinates
    let (row_0, col_0) = from;
    let (row_1, col_1) = to;
    let row_diff = (row_1 as i8 - row_0 as i8).abs () as usize;
    let col_diff = (col_1 as i8 - col_0 as i8).abs () as usize;

    // Determine if we need to go up or down in the vertical axis. Same for left and right.
    let v_dir = if row_0 < row_1 { DirectionalEntry::Up } else { DirectionalEntry::Down };
    let h_dir = if col_0 < col_1 { DirectionalEntry::Right } else { DirectionalEntry::Left };

    // Simple sequence of "all vertical" movement. Same for the "all horizontal" displacement.
    let vertical = iter::repeat(v_dir).take(row_diff);
    let horizontal = iter::repeat(h_dir).take(col_diff);

    // We only consider two sequences to reach the requested entry:
    // * an all-vertical then all-horizontal sequence
    // * an all-horizontal then all-vertical sequence
    let sequence_0 = vertical.clone()
        .chain(horizontal.clone ())
        .chain(iter::once(DirectionalEntry::Activate));

    let sequence_1 = horizontal
        .chain(vertical)
        .chain(iter::once(DirectionalEntry::Activate));

    [sequence_0, sequence_1]
}

/// This function is similar to [get_raw_sequences_from_coordinates] but do not return
/// a sequence that would require to move the robot arm above the `empty_button`.
fn get_sequences_from_coordinates (
    from: (u8, u8),
    to: (u8, u8),
    empty_button: (u8, u8))
    -> Vec<Vec<DirectionalEntry>> {

    let [sequence_0, sequence_1] =
        get_raw_sequences_from_coordinates(from, to);

    let row_diff = (to.0 as i8 - from.0 as i8).abs () as usize;
    let col_diff = (to.1 as i8 - from.1 as i8).abs () as usize;

    let sequence_0: Vec<DirectionalEntry> = sequence_0.collect();
    let sequence_1: Vec<DirectionalEntry> = sequence_1.collect();

    // Avoid the vert-horz sequence if we would have to go above the empty button
    // (start column and destination row cross above it)
    let (avoid_row, avoid_col) = empty_button;
    let avoid_seq_0 = from.1 == avoid_col && to.0 == avoid_row;

    // Avoid the horz-vert sequence if we would have to go above the empty button
    // (start row and destination column cross above it)
    let avoid_seq_1 = from.0 == avoid_row && to.1 == avoid_col;

    match (row_diff, col_diff) {

        // Avoid returning two identical sequences for full horizontal or vertical movements
        (0, _) | (_, 0) => vec![sequence_0],

        // Otherwise, return the two sequences provided they do not overlap the empty button
        _ => {
            match (avoid_seq_0, avoid_seq_1) {
                (false, false) => vec![sequence_0, sequence_1],
                (true, false)  => vec![sequence_1],
                (false, true)  => vec![sequence_0],
                _              => unreachable!(),
            }
        }
    }
}

impl DirectionalKeypad {

    /// New directional keypad instance, arm starting on the *Activate* button
    fn new () -> Self {
        Self { pos: DirectionalEntry::Activate }
    }

    /// Given the current robot's arm position, return the different directional sequences
    /// that enable to reach the provided `entry` button and to press it.
    ///
    /// For example, going from `<` to `A` would return this sequence:
    /// * `>>^A`
    ///
    /// **This function updates the current robot's arm position**
    fn get_sequences_to (&mut self, entry: DirectionalEntry) -> Vec<Vec<DirectionalEntry>> {

        // Get the coordinates of the current arm position and of the final position
        let from = Self::entry_to_row_col(self.pos);
        let to = Self::entry_to_row_col(entry);

        // Update the robot arm position
        self.pos = entry;

        const EMPTY_BUTTON: (u8, u8) = (1, 0);
        get_sequences_from_coordinates(from, to, EMPTY_BUTTON)
    }

    /// Return the (row, column) coordinate of a button. The *Left key* is in `(0, 0)`
    fn entry_to_row_col(entry: DirectionalEntry) -> (u8, u8) {
        match entry {
            DirectionalEntry::Activate => (1, 2),
            DirectionalEntry::Left     => (0, 0),
            DirectionalEntry::Right    => (0, 2),
            DirectionalEntry::Down     => (0, 1),
            DirectionalEntry::Up       => (1, 1),
        }
    }
}

impl NumericalKeypad {

    /// New numerical keypad instance, arm starting on the *Activate* button
    fn new () -> Self {
        Self { pos: NumericalEntry::Activate }
    }

    /// Given the current robot's arm position, return the different directional sequences
    /// that enable to reach the provided `entry` button and to press it.
    ///
    /// For example, going from `1` to `8` would return this sequence:
    /// * `>^^A`
    /// * `^^>A`
    ///
    /// **This function updates the current robot's arm position**
    fn get_sequences_to (&mut self, entry: NumericalEntry) -> Vec<Vec<DirectionalEntry>> {

        // Get the coordinates of the current arm position and of the final position
        let from = Self::entry_to_row_col(self.pos);
        let to = Self::entry_to_row_col(entry);

        // Update the robot arm position
        self.pos = entry;

        const EMPTY_BUTTON: (u8, u8) = (0, 0);
        get_sequences_from_coordinates(from, to, EMPTY_BUTTON)
    }

    /// Return the (row, column) coordinate of a button. The *Activation key* is in `(0, 2)`
    fn entry_to_row_col(entry: NumericalEntry) -> (u8, u8) {
        match entry {
            NumericalEntry::Activate => (0, 2),
            NumericalEntry::Digit (0) => (0, 1),
            NumericalEntry::Digit (d) => {
                let row = 1 + (d-1) / 3;
                let col = (d-1) % 3;
                (row, col)
            },
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Load the different codes we have to deal with from the puzzle file `content`
fn load_codes (content: &[&str]) -> Result<Vec<Code>> {
    let mut reader= IntReader::new(false);

    content.iter().map(|&row| {
        let raw: [u32; 1] = reader.process_row_fix(row)
            .ok_or(anyhow!("Invalid code: {}", row))?;

        let digits = [
            (raw[0] / 100) as u8,
            ((raw[0] / 10) % 10) as u8,
            (raw[0] % 10) as u8,
        ];

        Ok(Code { digits, value: raw [0] })
    }).collect()
}

/// Given a `current_pos` and a `sequence` of entries on a keypad, return an iterator
/// over the different movement to execute.
/// ## Example
/// ```
/// current_pos: A
/// sequence: [1, 8, 0]
/// return: [(A, 1), (1, 8), (8, 0)]
/// ```
fn sequence_to_movements<'a> (current_pos: DirectionalEntry, sequence: &'a [DirectionalEntry]) -> impl Iterator<Item = StartDest> + 'a{
    let seq = iter::once (current_pos).chain(sequence.iter ().copied());
    seq.tuple_windows()
}

/// Compute the length of the shortest sequence that enables to make a single `movement` on the
/// numerical keypad through a chain of `depth` robots.
fn compute_move_length_through_robot_chain(memo: &mut Memo, movement: StartDest, depth: usize) -> usize {

    // No robot to consider, we can do the movement ourselves in one step
    if depth == 0 { return 1; }

    // Consult the table and return the value if we know it
    let memo_key = (movement, depth);
    if let Some (length) = memo.get(&memo_key) { return *length; }

    // Otherwise, consider the first robot from the chain and get the different
    // sequences that would enable to execute the movement
    let mut robot = DirectionalKeypad::new();
    robot.pos = movement.0;
    let sequences = robot.get_sequences_to(movement.1);

    // Analyze each of such sequence and keep the best one
    let min_length = sequences.iter().map (|seq| {

        // The current sequence is split into a succession of movements.
        // We recurse on each of them and sum up everything
        sequence_to_movements(DirectionalEntry::Activate, seq).map (|movement| {
            compute_move_length_through_robot_chain(memo, movement, depth-1)
        }).sum ()

    }).min().unwrap();

    // Save the computed value
    memo.insert(memo_key, min_length);

    // And return the shortest sequence length
    min_length
}

/// Compute the length of the shortest sequence that enables to enter the provided `code`
/// on the numerical keypad. Parameter `depth` gives the number of intermediate robots
/// between the final numerical keypad and the robot we manipulate ourselves.
/// (i.e: 2 for part 1, 25 for part 2)
fn compute_min_sequence_length(memo: &mut Memo, code: Code, depth: usize) -> usize {

    let mut num_key = NumericalKeypad::new();

    // Set up the sequence of buttons to press on the numerical keypad to enter the code
    let digit_seq = code.digits.iter()
        .map(|&d| {NumericalEntry::Digit (d)})
        .chain(iter::once(NumericalEntry::Activate));

    // Sum the length required for each digit
    let mut total_length = 0;
    digit_seq.for_each(|entry| {

        // Get all possible sequences to reach each digit.
        // Take the min of such sequences to get the digit sequence length
        let sequences = num_key.get_sequences_to(entry);
        let min_length: usize = sequences.iter ().map (|seq| {

            // For each sequence, we decompose into a succession of moves. We sum
            // the length of the best sequence for each move
            sequence_to_movements(DirectionalEntry::Activate, seq).map (|movement| {
                compute_move_length_through_robot_chain(memo, movement, depth)
            }).sum ()

        }).min().unwrap();

        total_length += min_length;
    });

    total_length
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    const DEPTH: usize = 2;

    let codes = load_codes(content)?;
    let mut memo = Memo::new();

    let mut complexity = 0;
    for code in codes {
        let seq_len = compute_min_sequence_length(&mut memo, code, DEPTH);
        complexity += seq_len * code.value as usize;
    }

    Ok(complexity)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    const DEPTH: usize = 25;

    let codes = load_codes(content)?;
    let mut memo = Memo::new();

    let mut complexity = 0;
    for code in codes {
        let seq_len = compute_min_sequence_length(&mut memo, code, DEPTH);
        complexity += seq_len * code.value as usize;
    }

    Ok(complexity)
}

pub fn day_21 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 126384);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}