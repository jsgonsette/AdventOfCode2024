use std::collections::HashSet;
use anyhow::*;
use crate::Solution;
use crate::tools::{Coo, Direction};

const TEST: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

/// Models a move instruction
#[derive(Debug)]
struct Move (Direction, u8);

/// Models a rope with `N` nodes
struct Rope<const N: usize> {
    body: [Coo; N],
    visited: HashSet<Coo>,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl<const N: usize> Rope<N> {

    /// New rope instance
    fn new () -> Rope<N> {
        Rope  {
            body: [Coo::default(); N],
            visited: HashSet::from([Coo::default()]),
        }
    }

    /// Follow the rope dynamic, given the instruction `mov`
    fn update (&mut self, mov: &Move) {

        // Iterate on the number of steps
        for _step in 0.. mov.1 {

            // Move the head
            self.body [0] = self.body [0].next(mov.0);

            // The body follows
            for idx in 1..N {
                let front = self.body [idx-1];
                let dx = front.x - self.body [idx].x;
                let dy = front.y - self.body [idx].y;

                if dx.abs() >= 2 || dy.abs() >= 2 {
                    self.body [idx].x += dx.signum();
                    self.body [idx].y += dy.signum();
                }
                else { break; }
            }

            self.visited.insert(self.body [N-1]);
        }
    }
}

/// Return an iterator yielding [Move], based on the puzzle file `content`
fn get_moves<'a> (content: &'a[&'a str]) -> impl Iterator<Item=Result<Move>> + 'a {

    content.iter().map (|&row| {

        let (left, right) = row.split_at(2);

        let direction = match left.as_bytes() [0] {
            b'R' => Direction::Right,
            b'U' => Direction::Up,
            b'L' => Direction::Left,
            b'D' => Direction::Down,
            _ => bail!("Unrecognized direction: {}", left)
        };

        let step = right.parse::<u8>()?;

        Ok (Move (direction, step))
    })
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut rope = Rope::<2>::new();

    for mov in get_moves(content) {
        rope.update(&mov?);
    }

    Ok(rope.visited.len())
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let mut rope = Rope::<10>::new();

    for mov in get_moves(content) {
        rope.update(&mov?);
    }

    Ok(rope.visited.len())
}

pub fn day_9 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 13);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}