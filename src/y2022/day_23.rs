use std::cmp::PartialEq;
use anyhow::*;
use bit_vec::BitVec;
use itertools::Itertools;
use nalgebra::SimdBool;
use crate::{Cell, GridCell, Solution};
use crate::y2022::day_23::Direction::West;

const TEST: &str = "\
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// The four possible displacements
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North, South, East, West,
}


/// Models the playground with the elves
struct PlayGround {

    field: Vec<BitVec>,

    /// Directions to test for the vote rule
    test_directions: [Direction; 4],
}

impl PlayGround {

    fn new (content: &[&str]) -> Result<PlayGround> {

        let width = content[0].len();
        let height = content.len();

        let field = content.iter().map (|row| {
            BitVec::from_iter(
                row.as_bytes().iter().map(|&c| c != b'.'),
            )
        }).collect();

        Ok (PlayGround {
            field,
            test_directions: [Direction::North, Direction::South, Direction::East, Direction::West],
        })
    }

    fn width (&self) -> usize {
        self.field[0].len()
    }

    fn height (&self) -> usize {
        self.field.len()
    }

    fn _print (&self) {
        for row in self.field.iter() {
            println!();
            for item in row.iter() {
                print!("{}", if item.any() { '#' } else { '.' });
            }
        }
    }

    fn compute_horizontal_occupancy (row: &BitVec) -> BitVec {
        BitVec::from_iter((0..row.len()).into_iter().map(|x| {
            let a = if x > 0 { row [x - 1] } else { false };
            let b = row [x];
            let c = if x < row.len()-1 { row [x + 1] } else { false };
            a | b | c
        }))
    }

    fn compute_occupancies (&mut self) {

        let mut a = BitVec::from_elem(self.width(), false);
        let mut b = self.field [0].clone();
        let mut c = self.field [1].clone();

        for y in 2..self.height() {

            a = b;
            b = c;
            c = self.field[y].clone();

            let up = Self::compute_horizontal_occupancy(&a);
            let down = Self::compute_horizontal_occupancy(&c);
            //let mut vertical = a
            ///let left = vertical.clone();

            println!();
            for item in up.iter() {
                print!("{}", if item.any() { '#' } else { '.' });
            }
            //let up = a
        }
    }
}

/// Solve both parts of the puzzle
fn solve (content: &[&str]) -> Result<(usize, usize)> {

    let mut playground = PlayGround::new(content)?;
    playground._print();
    println!();
    playground.compute_occupancies();

    let mut round = 0;
    let mut empty_area = 0;

    let round_stop = loop {
        round += 1;
        break round;
    };

    Ok((empty_area, round_stop))
}

pub fn day_23 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST)).unwrap_or_default() != (110, 20));

    let (ra, rb) = (0, 0);//solve(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}