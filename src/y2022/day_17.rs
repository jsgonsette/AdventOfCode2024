use std::collections::HashMap;
use anyhow::*;
use itertools::Itertools;
use crate::Solution;

const TEST: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

/// Encodes a single chamber row with a Byte, each bit at 1 is occupied.
type StackRow = u8;

/// Jet direction
#[derive(Debug, Copy, Clone)]
enum JetDirection {
    Left, Right,
}

/// The 5 rock types
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum RockType {
    Horizontal,
    Cross,
    L,
    Vertical,
    Square
}

/// Encodes the content of a rock, and its lateral position, as 4 [StackRow]
#[derive(Debug, Copy, Clone)]
struct Rock ([StackRow; 4]);

/// Models the falling rock chamber
struct Chamber {

    /// Chamber content (except the falling rock)
    rows: Vec<StackRow>,

    /// Next rock to instantiate
    next_rock: RockType,

    /// Current falling rock
    current_rock: Rock,

    /// Height of the stack (index of the first free row)
    top: usize,

    /// Row index of the falling rock bottom
    rock_bottom: usize,

    /// Number of rocks in the stack (except the falling rock)
    rock_counter: u32,
}

/// Encodes the state of the chamber when a block has just stopped moving
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ChamberState {

    /// Next rock to use
    next_rock: RockType,

    /// Encoding of the last 8 rows of the stack (we suppose this is enough)
    stack_top: u64,

    /// Index of the last jet direction used
    jet_index: usize,
}

impl RockType {

    /// Next rock to instantiate
    fn next (&self) -> RockType {
        match self {
            RockType::Horizontal => RockType::Cross,
            RockType::Cross =>  RockType::L,
            RockType::L =>  RockType::Vertical,
            RockType::Vertical =>  RockType::Square,
            RockType::Square =>  RockType::Horizontal,
        }
    }

    /// Materializes a rock
    fn to_rock(&self) -> Rock {
        match self {
            RockType::Horizontal => Rock ([
                0b00_0000_0,
                0b00_0000_0,
                0b00_0000_0,
                0b00_1111_0]),
            RockType::Cross => Rock ([
                0b00_0000_0,
                0b00_0100_0,
                0b00_1110_0,
                0b00_0100_0]),
            RockType::L => Rock ([
                0b00_0000_0,
                0b00_0010_0,
                0b00_0010_0,
                0b00_1110_0]),
            RockType::Vertical => Rock ([
                0b00_1000_0,
                0b00_1000_0,
                0b00_1000_0,
                0b00_1000_0]),
            RockType::Square => Rock ([
                0b00_0000_0,
                0b00_0000_0,
                0b00_1100_0,
                0b00_1100_0]),
        }
    }
}

impl Rock {

    /// Shift the lateral position of the rock to the right, if possible
    fn shifted_right (&self) -> Option<Rock>  {
        if self.0 [0] & 0b1 == 0b1 ||
            self.0 [1] & 0b1 == 0b1 ||
            self.0 [2] & 0b1 == 0b1 ||
            self.0 [3] & 0b1 == 0b1 {
            None
        }
        else {
            Some (Rock ([
                self.0 [0] >> 1,
                self.0 [1] >> 1,
                self.0 [2] >> 1,
                self.0 [3] >> 1,
            ]))
        }
    }

    /// Shift the lateral position of the rock to the left, if possible
    fn shifted_left (&self) -> Option<Rock>  {
        if self.0 [0] & 0b1_000000 == 0b1_000000 ||
            self.0 [1] & 0b1_000000 == 0b1_000000 ||
            self.0 [2] & 0b1_000000 == 0b1_000000 ||
            self.0 [3] & 0b1_000000 == 0b1_000000 {
            None
        }
        else {
            Some (Rock ([
                self.0 [0] << 1,
                self.0 [1] << 1,
                self.0 [2] << 1,
                self.0 [3] << 1,
            ]))
        }
    }
}

impl Chamber {

    /// Instantiate a new empty chamber
    fn new () -> Self {
        Chamber {
            rows: vec![0; 7],
            top: 0,
            rock_bottom: 3,
            rock_counter: 0,
            current_rock: RockType::Horizontal.to_rock(),
            next_rock: RockType::Horizontal.next(),
        }
    }

    /// Instantiate a new chamber from the provided chamber `state`.
    ///
    /// **This state retains only the last 8 rows!**
    fn from_state (state: &ChamberState) -> Self {
        let mut chamber = Chamber {
            rows: Self::decode_top(state.stack_top),
            top: 0,
            rock_bottom: 0,
            rock_counter: 0,
            current_rock: state.next_rock.next().next().next().next().to_rock(),
            next_rock: state.next_rock,
        };

        chamber.extend();
        chamber
    }

    /// Update the falling rock position by one step:
    /// 1) The rock moves left or right according to `direction`
    /// 2) The rock drops by one row. If not possible a new rock is instantiated
    ///
    /// The function returns `true` if the rock could fall, or `false` in the other case.
    fn do_step(&mut self, direction: JetDirection) -> bool {

        // Move the block left or right if we don't bump into the walls
        let shifted = match direction {
            JetDirection::Left => self.current_rock.shifted_left(),
            JetDirection::Right => self.current_rock.shifted_right(),
        };

        // Check if we would collide in other blocks. If no, update the rock
        if let Some (shifted) = shifted {
            if !self.collide(&shifted, self.rock_bottom) {
                self.current_rock = shifted;
            }
        }

        // Try to move the rock down. We fail if we hit the floor or any other block
        if self.rock_bottom == 0 || self.collide(&self.current_rock, self.rock_bottom -1) {

            // Add the rock to the chamber
            self.add_rock_still(self.current_rock, self.rock_bottom);

            // Instantiate new rock and extend the chamber
            self.current_rock = self.next_rock.to_rock();
            self.next_rock = self.next_rock.next();
            self.extend();

            false
        }
        else {
            // Step down
            self.rock_bottom -= 1;
            true
        }
    }

    /// Extend the chamber with free space below the rock (3) and room for the new falling rock (4)
    fn extend (&mut self) {

        self.top = self.get_stack_height();
        self.rock_bottom = self.top + 3;

        let top_with_room = self.top + 7;
        if self.rows.len() < top_with_room {
            self.rows.extend(std::iter::repeat(0).take(top_with_room - self.rows.len()));
        }
    }

    /// Decode the `encoded` top of stack and returns the 8 corresponding [StackRow]
    fn decode_top (mut encoded: u64) -> Vec<StackRow> {

        let mut v: Vec<StackRow> = (0..8).map(|_| {
            let row = encoded & 0xff;
            encoded >>= 8;

            row as StackRow
        }).collect();

        v.reverse();
        v
    }

    /// Encode the 8 top most [StackRow] of the chamber's stack
    fn encode_top (&self) -> u64 {

        let idx_start = if self.top >= 8 { self.top - 8 } else { 0 };
        let mut encoded = 0;

        for idx in idx_start..self.top {
            encoded <<= 8;
            encoded |= self.rows[idx] as u64;
        }
        encoded
    }

    /// Return the height of the stack
    fn get_stack_height(&self) -> usize {
        for (idx, row) in self.rows.iter().rev().enumerate() {
            if *row != 0 { return self.rows.len() - idx; }
        }

        0
    }

    /// Add a `rock` to the chamber's content. Parameter `rock_bottom` indicates
    /// where the bottom row of the rock must be put.
    fn add_rock_still (&mut self, rock: Rock, rock_bottom: usize) {
        for idx in 0..4 {
            let row_chamber = &mut self.rows[rock_bottom + idx];
            let row_rock = rock.0 [3-idx];
            *row_chamber |= row_rock;
        }
        self.rock_counter += 1;
    }

    /// Return `true` if the provided `rock` at position `rock_bottom` collides with
    /// the chamber's content.
    fn collide (&self, rock: &Rock, rock_bottom: usize) -> bool {

        // Test the 4 rows of the rock
        for idx in 0..4 {
            let row_chamber = self.rows[rock_bottom + idx];
            let row_rock = rock.0 [3-idx];
            if row_chamber & row_rock != 0 { return true; }
        }
        false
    }

    /// Debug print the chamber content
    fn _print (&self) {
        let above_rock = self.rows.len() - self.rock_bottom - 4;
        for (idx, row) in self.rows.iter().rev ().enumerate() {
            let row_rock = if idx >= above_rock && idx < above_rock +4 {
                self.current_rock.0 [idx - above_rock]
            } else {
                0
            };

            Self::_print_row(*row, row_rock);
            if idx > 20 {
                println!("  (...)");
                break;
            }
        }
        println!("+-------+");
    }

    fn _print_row (row: StackRow, rock_row: StackRow) {
        let content = (0..7).map (|idx| {
            let mask = 0b1000000 >> idx;
            match (row & mask, rock_row & mask) {
                (0, 0) => '.',
                (_, 0) => '#',
                (0, _) => '@',
                _ => '*'
            }
        }).join("");
        print!("|");
        print!("{}", content);
        println!("|");
    }
}

/// This iterator never ends and yields a new [ChamberState] and *stack height* pair
/// each time a block has finished fallen
fn infinite_tower_it (jet_pattern: &str) -> impl Iterator<Item=(ChamberState, u32)> + '_ {

    let mut chamber = Chamber::new();

    jet_pattern.as_bytes().iter().enumerate().cycle ().filter_map(move |(jet_index, pattern)| {

        let move_block = match pattern {
            b'<' => chamber.do_step(JetDirection::Left),
            b'>' => chamber.do_step(JetDirection::Right),
            _ => panic!("invalid pattern in chamber"),
        };

        if !move_block {
            let state = ChamberState {
                next_rock: chamber.next_rock,
                stack_top: chamber.encode_top(),
                jet_index,
            };

            Some ((state, chamber.top as u32))
        }
        else { None }
    })
}

/// Given the `jet_pattern` and a `chamber`, iterates until `num_blocks` have fallen.
/// If not 0, parameter `jet_index` enables to start later in the jet sequence.
fn drop_blocks (jet_pattern: &str, chamber: &mut Chamber, jet_index: usize, num_blocks: u32) -> Result<()> {

    for pattern in jet_pattern.as_bytes().iter().cycle().skip(jet_index) {
        match pattern {
            b'<' => chamber.do_step(JetDirection::Left),
            b'>' => chamber.do_step(JetDirection::Right),
            _ => bail!("Invalid character in pattern '{}'", pattern),
        };

        if chamber.rock_counter >= num_blocks { break }
    }

    Ok(())
}

/// Solve first part of the puzzle
fn part_a (jet_pattern: &str) -> Result<usize> {

    let mut chamber = Chamber::new();

    drop_blocks(jet_pattern, &mut chamber, 0, 2022)?;
    let height = chamber.get_stack_height();

    Ok(height)
}

/// Solve second part of the puzzle
fn part_b (jet_pattern: &str) -> Result<usize> {

    type Index = usize;
    type Height = u32;
    type Info = (Index, Height);

    // To collect the states we have already seen
    let mut states = HashMap::<ChamberState, Info>::new();

    // Iterate as long as we encounter new states
    for (idx, (state, height)) in infinite_tower_it(jet_pattern).enumerate() {

        // If not a new state ...
        if let Some ((first_idx, first_height)) = states.get(&state) {

            // Do some maths to deduce the characteristics of the cycle
            let num_before_cycle = first_idx +1;
            let cycle_len = idx - first_idx;
            let cycle_height = height - first_height;
            let num_cycles = (1000_000_000_000 - num_before_cycle) / cycle_len;
            let remaining = (1000_000_000_000 - num_before_cycle) % cycle_len;

            // Instantiate a chamber to simulate the `remaining` rocks in the last cycle
            let mut chamber = Chamber::from_state(&state);
            let init_height = chamber.get_stack_height();
            drop_blocks(jet_pattern, &mut chamber, state.jet_index +1, remaining as u32)?;
            let final_height = chamber.get_stack_height();

            let total_height = *first_height as usize +
                cycle_height as usize * num_cycles +
                final_height - init_height;

            return Ok(total_height);
        }

        states.insert(state, (idx, height));
        if idx > 100000 { break }
    }

    bail!("No cycle found");
}

pub fn day_17 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (TEST).unwrap_or_default() == 3068);
    debug_assert!(part_b (TEST).unwrap_or_default() == 1514285714288);

    let ra = part_a(content [0])?;
    let rb = part_b(content [0])?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}