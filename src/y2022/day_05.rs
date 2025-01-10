use anyhow::*;
use crate::Solution;
use crate::tools::IntReader;

const TEST: &str = "\
....[D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";


enum Crane {
    CrateMover9000,
    CrateMover9001,
}

/// A stack of crates
type Stack = Vec<char>;

/// One move operation
#[derive(Debug, Copy, Clone)]
struct Move {
    from: usize,
    to: usize,
    amount: u32,
}

/// All the stacks
#[derive(Debug)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {

    /// Given the raw top lines `crates` of the puzzle file content, instantiate the different
    /// stacks of crates.
    fn new(crates: &[&str]) -> Result<Stacks> {

        let num_stacks = Self::get_num_stacks(crates)?;
        let stacks = (0..num_stacks).map(
            |i| Self::init_stack_of_crates(crates, i as usize)
        ).collect();

        Ok (Stacks {
            stacks,
        })
    }

    /// Return a description of the topmost crate of each stack
    fn get_top_row (&self) -> String {
        self.stacks.iter().map (
            |stack| stack.last().copied().unwrap_or_default()
        ).collect ()
    }

    /// Execute a `mov` with the Crate Mover 9000
    fn make_move_9000 (&mut self, mov: Move) {

        let mut intermediate = vec! [];

        // Extract the vector's top elements in reverse into the `intermediate` vector
        let from = &mut self.stacks [mov.from -1];
        let first = from.len() - mov.amount as usize;
        intermediate.extend (from.drain (first..).rev ());

        // Move the `intermediate` content on top of the destination stack
        let to = &mut self.stacks [mov.to -1];
        to.extend(intermediate);
    }

    /// Execute a `mov` with the Crate Mover 9000
    fn make_move_9001 (&mut self, mov: Move) {
        let mut intermediate = vec! [];

        // Extract the vector's top elements (same order) into the `intermediate` vector
        let from = &mut self.stacks [mov.from -1];
        let first = from.len() - mov.amount as usize;
        intermediate.extend(from.drain(first..));

        // Move the `intermediate` content on top of the destination stack
        let to = &mut self.stacks [mov.to -1];
        to.extend(intermediate);
    }

    /// Instantiate the `stack_idx`'th [Stack] of crates, given the head of the puzzle file.
    /// Top most crate is at the end of the vector.
    fn init_stack_of_crates(crates: &[&str], stack_idx: usize) -> Stack {

        let sample_crate_name = |x: usize, y: usize|-> Option<char> {
            let row = crates [y].as_bytes();
            let maybe_char = row.get (x).map(|c| *c as char);
            match maybe_char {
                Some(c) if c.is_ascii_alphabetic() => Some(c),
                _ => None,
            }
        };

        let height = crates.len() -1;
        let x = stack_idx*4 + 1;
        (0 .. height).rev ().flat_map(|y| sample_crate_name (x, y)).collect()
    }

    /// Determine the number of stacks from the head of the puzzle file.
    fn get_num_stacks (crates: &[&str]) -> Result<u32> {

        // Read the last number written below the stacks schema
        let height = crates.len() -1;
        let num_stacks = crates [height].trim().as_bytes().last().ok_or(anyhow!("Invalid crates"))?;
        let num_stacks = (*num_stacks as char).to_digit(10).ok_or(anyhow!("Invalid crates"))?;

        Ok (num_stacks)
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Return an iterator on the moves defined in the puzzle file content. This content
/// is given by `rows` and corresponds to the second part of the file.
fn get_move_it<'a> (rows: &'a[&'a str]) -> impl Iterator<Item = Result<Move>> + '_ {

    let mut reader = IntReader::new(false);

    rows.iter().map (move |row| {
        let raw_move: Vec<u32> = reader.iter_row::<u32>(row).collect();

        Ok(Move {
            from: raw_move [1] as usize,
            to: raw_move [2] as usize,
            amount: raw_move [0],
        })
    })
}

/// Follow the sequence of operations and return the final sequence of top crates.
fn solve (content: &[&str], crane: Crane)-> Result<String> {

    // Extract the first lines dedicated to the initial configuration
    let crates: Vec<&str> = content.iter().copied().take_while(
        |&row| !row.is_empty()
    ).collect();

    // Build the stacks of crates
    let mut stacks = Stacks::new(&crates)?;

    // Perform the moves
    for mov in get_move_it(&content [crates.len()+1..]) {

        let mov = mov?;
        if mov.from < 1 || mov.to > stacks.stacks.len() { bail!("Invalid move {:?}", mov); }

        match crane {
            Crane::CrateMover9000 => { stacks.make_move_9000 (mov); }
            Crane::CrateMover9001 => { stacks.make_move_9001 (mov); }
        }
    }

    Ok(stacks.get_top_row())
}

pub fn day_5 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST), Crane::CrateMover9000).unwrap_or_default() == "CMZ");
    debug_assert!(solve (&split(TEST), Crane::CrateMover9001).unwrap_or_default() == "MCD");

    let ra = solve (content, Crane::CrateMover9000)?;
    let rb = solve (content, Crane::CrateMover9001)?;

    Ok((Solution::Text(ra), Solution::Text(rb)))
}