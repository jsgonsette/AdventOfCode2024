use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use anyhow::*;
use itertools::Itertools;
use crate::{Solution};
use crate::tools::RowReader;

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

/// This function is similar to `get_raw_sequences_from_coordinates` but do not return
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
    /// For example, going from `3` to `7` would return those two sequences:
    /// * `^^<<A`
    /// * `<<^^A`
    fn get_sequences_to (&mut self, entry: DirectionalEntry) -> Vec<Vec<DirectionalEntry>> {

        let from = Self::entry_to_row_col(self.pos);
        let to = Self::entry_to_row_col(entry);

        let [sequence_0, sequence_1] =
            get_raw_sequences_from_coordinates(from, to);

        let row_diff = (to.0 as i8 - from.0 as i8).abs () as usize;
        let col_diff = (to.1 as i8 - from.1 as i8).abs () as usize;

        let sequence_0: Vec<DirectionalEntry> = sequence_0.collect();
        let sequence_1: Vec<DirectionalEntry> = sequence_1.collect();

        // Update the robot arm position
        self.pos = entry;

        // Avoid the vert-horz sequence if we would have to go above the empty button
        // (start column and destination row cross above it)
        let avoid_seq_0 = to.0 == 1 && from.1 == 0;

        // Avoid the horz-vert sequence if we would have to go above the empty button
        // (start row and destination column cross above it)
        let avoid_seq_1 = to.1 == 0 && from.0 == 1;

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
    fn new () -> Self {
        Self { pos: NumericalEntry::Activate }
    }

    fn get_sequences_to (&mut self, entry: NumericalEntry) -> Vec<Vec<DirectionalEntry>> {

        let (row_0, col_0) = Self::entry_to_row_col(self.pos);
        let (row_1, col_1) = Self::entry_to_row_col(entry);
        let row_diff = (row_1 as i8 - row_0 as i8).abs () as usize;
        let col_diff = (col_1 as i8 - col_0 as i8).abs () as usize;

        let v_dir = if row_0 < row_1 { DirectionalEntry::Up } else { DirectionalEntry::Down };
        let h_dir = if col_0 < col_1 { DirectionalEntry::Right } else { DirectionalEntry::Left };
        let vertical = iter::repeat(v_dir).take(row_diff);
        let horizontal = iter::repeat(h_dir).take(col_diff);

        let sequence_0: Vec<DirectionalEntry> = vertical.clone().chain(horizontal.clone ()).chain(iter::once(DirectionalEntry::Activate)).collect();
        let sequence_1: Vec<DirectionalEntry> = horizontal.chain(vertical).chain(iter::once(DirectionalEntry::Activate)).collect();

        self.pos = entry;

        match (sequence_0.len (), sequence_1.len ()) {
            (0, 0) => vec![],
            (0, _) => vec![sequence_1],
            (_, 0) => vec![sequence_0],
            _      => {
                if row_diff == 0 || col_diff == 0 { vec![sequence_0] }
                else if row_0 == 0 && col_1 == 0 {
                    vec![sequence_0]
                }
                else if row_1 == 0 && col_0 == 0 {
                    vec![sequence_1]
                }
                else {
                    vec![sequence_0, sequence_1]
                }
            }
        }
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

fn load_codes (content: &[&str]) -> Result<Vec<Code>> {
    let mut reader= RowReader::new(false);

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

fn expand_sequence(sequence: &[DirectionalEntry], robot: &mut DirectionalKeypad) -> Vec<Vec<DirectionalEntry>> {

    let first_entry_seq = robot.get_sequences_to(sequence[0]);
    let remaining_seq = match sequence.len() {
        1 => return first_entry_seq,
        _ => expand_sequence(&sequence[1..], robot),
    };

    first_entry_seq.iter().cartesian_product(remaining_seq.iter())
        .map (|(seq_0, seq_1)| seq_0.iter().chain(seq_1.iter()).cloned().collect())
        .collect()
}

type StartDest = (DirectionalEntry, DirectionalEntry);
type MemoKey = (StartDest, usize);
type Memo = HashMap<MemoKey, usize>;

fn expand_sequence_with_robot_chain (memo: &mut Memo, movement: StartDest, robots: &mut[DirectionalKeypad]) -> usize {

    if robots.len() == 0 { return 1; }

    let memo_key = (movement, robots.len());
    if let Some (length) = memo.get(&memo_key) { return *length; }

    let mut robot = robots [0];
    robot.pos = movement.0;
    let sequences = robot.get_sequences_to(movement.1);

    let min_length = sequences.iter().map (|seq| {

        let seq = iter::once (DirectionalEntry::Activate).chain(seq.iter ().copied());
        seq.tuple_windows().map (|(start, dest)| {
            expand_sequence_with_robot_chain (memo, (start, dest), &mut robots[1..])
        }).sum ()

    }).min().unwrap();

    memo.insert(memo_key, min_length);

    min_length
}

fn compute_min_sequence_length_2 (memo: &mut Memo, code: Code) -> usize {

    let mut num_key = NumericalKeypad::new();
    let mut robots_dir_key = [DirectionalKeypad::new(); 25];

    let digit_seq = code.digits.iter()
        .map(|&d| {NumericalEntry::Digit (d)})
        .chain(iter::once(NumericalEntry::Activate));

    // Sum length for each digit
    let mut total_length = 0;
    digit_seq.for_each(|entry| {

       // println!("{:?}", entry);

        // Get all possible sequences to reach each digit.
        // Take the min of such sequences to get the digit sequence length
        let sequences = num_key.get_sequences_to(entry);
        let length_digit: usize = sequences.iter ().map (|seq| {
         //   println!(" - {:?}", seq);

            // For each sequence, we decompose into a succession of moves. The number of operations
            // is the sum of all the moves needed.
            let seq = iter::once (DirectionalEntry::Activate).chain(seq.iter ().copied());
            seq.tuple_windows().map (|(start, dest)| {
           //     println!("    {:?} -> {:?}", start, dest);
                expand_sequence_with_robot_chain (memo, (start, dest), &mut robots_dir_key)
            }).sum()
            // We expand the current sequence through a chain of `n` robots.
            // This method return the minimal possible sequence length

        }).min().unwrap();

        total_length += length_digit;
    });

  //  println!("total length: {}", total_length);
    total_length
}

fn compute_min_sequence_length (code: Code) -> usize {

    let mut num_key = NumericalKeypad::new();
    let mut dir_key_1 = DirectionalKeypad::new();
    let mut dir_key_2 = DirectionalKeypad::new();

    let digit_seq = code.digits.iter()
        .map(|&d| {NumericalEntry::Digit (d)})
        .chain(iter::once(NumericalEntry::Activate));

    let mut total_length = 0;
    digit_seq.for_each(|entry| {
        println!("{:?}", entry);
        let sequences = num_key.get_sequences_to(entry);
        let length_0 = sequences.iter ().map (|seq| {
            println!(" - {:?}", seq);
            //for ex_seq in expand_sequence(&seq, &mut dir_key_1) {
            let length_1 = expand_sequence(&seq, &mut dir_key_1).iter().map (|ex_seq| {
                println!("    ex {:?} ({})", ex_seq, ex_seq.len ());
                let length_2 = expand_sequence(&ex_seq, &mut dir_key_2).iter().map(|ex_seq_2| {
                    println!("       ex2 {:?} ({})", ex_seq_2, ex_seq_2.len ());
                    ex_seq_2.len()
                }).min().unwrap();
                length_2
            }).min ().unwrap();
            length_1
        }).min().unwrap();

        total_length += length_0;
    });

    println!("total length: {}", total_length);
    total_length
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let codes = load_codes(content)?;
    let mut memo = Memo::new();

    let mut complexity = 0;
    for code in codes {
        let seq_len = compute_min_sequence_length_2(&mut memo, code);
        complexity += seq_len * code.value as usize;
    }

    println!("Memo length: {}", memo.len());
    Ok(complexity)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_21 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 126384);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}