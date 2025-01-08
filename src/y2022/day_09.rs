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

#[derive(Debug)]
struct Move (Direction, u8);

struct Rope {
    head: Coo,
    tail: Coo,
    visited: HashSet<Coo>,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}


impl Rope {

    fn new () -> Rope {
        Rope  {
            head: Coo::default(),
            tail: Coo::default(),
            visited: HashSet::from([Coo::default()]),
        }
    }

    fn update (&mut self, mov: &Move) {

        let (mut head, mut tail) = (self.head, self.tail);

        for _ in 0.. mov.1 {
            head = head.next(mov.0);
            //println!(" - head: {:?}", head);

            let dx = head.x - tail.x;
            let dy = head.y - tail.y;

            tail = match (dx.abs(), dy.abs()) {
                (0, 0) | (0, 1) | (1, 0) | (1, 1) => tail,
                (dxa, dya) if dya > dxa => Coo::from((head.x, head.y - dy.signum())),
                (dxa, dya)              => Coo::from((head.x - dx.signum(), head.y)),
            };

            self.visited.insert(tail);
            //println!(" - tail: {:?}", tail);
        }

        //println!("New position: {:?} - {:?}", head, tail);
        self.head = head;
        self.tail = tail;
    }
}

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
fn part_a (_content: &[&str]) -> Result<usize> {

    let mut rope = Rope::new();

    for mov in get_moves(_content) {
        rope.update(&mov?);
    }

    println!("{}", rope.visited.len());
    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_9 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}