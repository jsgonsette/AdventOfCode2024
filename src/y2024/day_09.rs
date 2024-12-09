use std::io;
use std::io::Write;
use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
2333133121414131402";

type FileId = u16;
type Size = usize;

/// A single block in the [FileSystem]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty(Size),
    File(FileId, Size),
}

/// File system made of [Block]
type FileSystem = Vec<Block>;

/// Print to help debugging
fn _print_file_system (fs: &FileSystem) -> io::Result<()> {
    for block in fs.iter() {
        match block {
            Block::Empty(_size) => print!("."),
            Block::File(file, _size) => print!("{}", file),
        }
    };

    println!();
    io::stdout ().flush()
}

/// Load the puzzle file content a [FileSystem]
fn load_file_system (content: &str) -> Result<FileSystem> {

    let mut is_block = true;
    let mut file_id: FileId = 0;

    let fs: Option<FileSystem> = content.as_bytes().iter().flat_map(|&b| {

        // Get the next digit, or fail
        let digit = (b as char).to_digit(10);
        let Some (digit) = digit else { return vec![None] };

        // Make a file or an empty space one time out of 2
        match is_block {
            false => {
                is_block = true;
                vec! [Some (Block::Empty(digit as Size)); digit as usize]
            },
            true => {
                is_block = false;
                let id = file_id;
                file_id += 1;
                vec! [Some (Block::File(id, digit as Size)); digit as usize]
            }
        }
    }).collect();

    fs.ok_or(anyhow!("Invalid file system map"))
}

/// Starting from `idx`, left to right, return the index of the next free block that
/// belongs to an empty space >= than the requested `need_space`. Return `None` if
/// nothing is found below `idx_limit`.
fn next_free_block (fs: &FileSystem, mut idx: usize, need_space: usize, idx_limit: usize) -> Option<usize> {
    loop {
        match fs.get(idx)? {
            Block::Empty(space) if *space >= need_space => break Some (idx),
            _ => {
                idx += 1;
                if idx >= idx_limit { break None }
            }
        }
    }
}

/// Starting from `idx`, right to left, return the index of the last block belonging
/// to the next file found.
fn prev_file_bock (fs: &FileSystem, mut idx: usize) -> Option<usize> {
    loop {
        match fs.get(idx)? {
            Block::File(_, _) => break Some (idx),
            Block::Empty(_)  => { if idx == 0 { break None } else { idx -= 1;} }
        }
    }
}

/// Reduce the recorded available space of all the free block at position `idx` and on its right.
fn reduce_free_space (fs: &mut FileSystem, mut idx: usize, amount: usize) {
    while let Block::Empty(space) = &mut fs [idx] {
        *space = space.saturating_sub(amount);
        idx += 1;
    }
}

/// Compute the checksum of a [FileSystem]
fn checksum (fs: &FileSystem) -> usize {
    fs.iter().enumerate().map ({
        |(idx, block)| match block {
            Block::Empty(_) => 0,
            Block::File(fileid, _) => *fileid as usize * idx,
        }
    }).sum()
}

/// Solve first part of the puzzle
fn part_a (content: &str) -> Result<usize> {

    let mut fs = load_file_system(content)?;

    let mut idx_free = 0;
    let mut idx_block = fs.len()-1;

    loop {

        //print_file_system (&fs);
        // Get the next free block (left to right) and the next file block (right to left)
        let next_free = next_free_block (&fs, idx_free, 1, idx_block);
        let next_file = prev_file_bock (&fs, idx_block);

        match (next_file, next_free) {

            // If we have both, swap the two blocks
            (Some (next_file), Some (next_free)) if next_file > next_free => {
                idx_free = next_free;
                idx_block = next_file;

                let (left, right) = fs.split_at_mut(idx_block);
                std::mem::swap(&mut left [idx_free], &mut right [0]);
            },

            _ => break,
        }
    }

    Ok (checksum(&fs))
}

/// Solve second part of the puzzle
fn part_b (content: &str) -> Result<usize> {

    // Get the file system
    let mut fs = load_file_system(content)?;

    // Cache to avoid looking for free blocks from the beginning at each time
    let mut idx_free_table = [0usize; 10];

    let mut idx_file = fs.len()-1;
    loop {

        //print_file_system (&fs);
        // Get the block index of the next file to process
        let Some (next_file) = prev_file_bock (&fs, idx_file) else { break };
        let Block::File(_f, space) = &fs [next_file] else { break };
        let space = *space;

        // Look for a free space big enough
        let idx_free = &mut idx_free_table [space];
        let next_free = next_free_block (&fs, *idx_free, space, idx_file);

        // If we have one at some location that is more on the left
        if let Some (next_free) = next_free {
            if next_file > next_free {

                // Move the whole file blocks
                let (left, right) = fs.split_at_mut(next_file - space +1);
                for item in 0..space {
                    std::mem::swap(&mut left [next_free + item], &mut right [item]);
                }

                // Reduce the size of the free space left
                reduce_free_space(&mut fs, next_free + space, space);
                *idx_free = next_free + space;
            }
        }

        // In any case, skip the block we just processed
        if next_file >= space {
            idx_file = next_file - space;
        }
        else { break }
    }

    Ok (checksum(&fs))
}

pub fn day_9 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&TEST).unwrap_or_default() == 1928);
    debug_assert!(part_b (&TEST).unwrap_or_default() == 2858);

    let ra = part_a(content[0])?;
    let rb = part_b(content [0])?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}