use anyhow::*;
use itertools::Itertools;

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

const TEST2: &str = "\

";

enum Crane {
    CrateMover9000,
    CrateMover9001,
}

/// A stack of crates
type Stack = Vec<char>;

/// One move operation
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
        ).join("")
    }

    /// Remove a crate from the stack `stack_idx`
    fn pop (&mut self, stack_idx: usize) -> Result<char> {
        let stack_from = self.stacks.get_mut(stack_idx).ok_or(anyhow!("invalid column {}", stack_idx))?;
        stack_from.pop().ok_or(anyhow!("stack is empty"))
    }

    /// Remove `amount` crates from the stack `stack_idx`
    fn pop_n (&mut self, stack_idx: usize, amount: usize) -> Result<Vec<char>> {

        let stack_from = self.stacks.get_mut(stack_idx).ok_or(anyhow!("invalid column {}", stack_idx))?;
        if amount > stack_from.len() { bail!("Not enough elements"); }

        let split_idx = stack_from.len() - amount;
        Ok(stack_from.split_off(split_idx))
    }

    /// Push one crate on top of the stack `stack_idx`
    fn push (&mut self, stack_idx: usize, item: char) -> Result<()> {
        let stack_to = self.stacks.get_mut(stack_idx).ok_or(anyhow!("invalid column {}", stack_idx))?;
        Ok(stack_to.push(item))
    }

    /// Push multiple crates (as given by `items)` on top of the stack `stack_idx`
    fn push_n (&mut self, stack_idx: usize, items: &[char]) -> Result<()> {
        let stack_to = self.stacks.get_mut(stack_idx).ok_or(anyhow!("invalid column {}", stack_idx))?;
        Ok(stack_to.extend(items))
    }

    /// Execute a move crate by crate
    fn make_move_9000(&mut self, mov: Move) -> Result<()> {
        for _ in 0..mov.amount {
            let item = self.pop(mov.from -1)?;
            self.push(mov.to -1, item)?;
        }

        Ok(())
    }

    /// Execute a combo move
    fn make_move_9001(&mut self, mov: Move) -> Result<()> {
        let items = self.pop_n(mov.from -1, mov.amount as usize)?;
        self.push_n(mov.to -1, &items)?;

        Ok(())
    }

    /// Instantiate the `stack_idx`'th [Stack] of crates, given the head of the puzzle file.
    /// Top most crate is as the end of the vector.
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

    /// Determine the number of stacks from the head of the puzzle file
    fn get_num_stacks (crates: &[&str]) -> Result<u32> {
        let height = crates.len() -1;
        let num_stacks = crates [height].trim().as_bytes().last().ok_or(anyhow!("Invalid crates"))?;
        let num_stacks = (*num_stacks as char).to_digit(10).ok_or(anyhow!("Invalid crates"))?;

        Ok (num_stacks)
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn extract_move (row: &str) -> Result<Move> {

    let error = || { anyhow!("Invalid row: {}", row) };

    let tokens: Vec<_> = row.split(' ').collect();
    let amount = tokens.get(1).ok_or(error ())?.parse::<u32>()?;
    let from = tokens.get(3).ok_or(error ())?.parse::<usize>()?;
    let to = tokens.get(5).ok_or(error ())?.parse::<usize>()?;

    Ok(Move {
        from,
        to,
        amount,
    })

    /*let init: Vec<u32> = vec! [0];
    let values = row.as_bytes().iter().skip (5).fold(init, |mut acc, &b| {
        if b.is_ascii_digit() {
            let current = acc.last_mut().unwrap();
            *acc.last_mut().unwrap() = (*current) * 10 + (b - b'0') as u32;
        }
        else if b == b' ' { acc.push(0); }
        acc
    });

    println!("{:?}", values);

    Ok(Move {
        from: values [1] as usize,
        to: values [2] as usize,
        amount: values [0],
    })*/
}

fn solve (content: &[&str], crane: Crane)-> Result<String> {

    // Extract the first lines dedicated to the initial configuration
    let crates: Vec<&str> = content.iter().copied().take_while(
        |&row| !row.is_empty()
    ).collect();

    // Build the stacks of crates
    let mut stacks = Stacks::new(&crates)?;

    // Make the moves
    for row in content.iter().skip(crates.len()+1) {
        let mov = extract_move(row)?;

        match crane {
            Crane::CrateMover9000 => { stacks.make_move_9000(mov)?; }
            Crane::CrateMover9001 => { stacks.make_move_9001(mov)?; }
        }
    }

    Ok(stacks.get_top_row())
}

pub fn day_5 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(solve (&split(TEST), Crane::CrateMover9000).unwrap_or_default() == "CMZ");
    debug_assert!(solve (&split(TEST), Crane::CrateMover9001).unwrap_or_default() == "MCD");

    let ra = solve (content, Crane::CrateMover9000)?;
    let rb = solve (content, Crane::CrateMover9001)?;

    println!("Solution for part a) is {}", ra);
    println!("Solution for part b) is {}", rb);
    Ok((42, 42))
}