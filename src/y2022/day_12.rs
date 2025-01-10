use std::cmp::Ordering;
use std::collections::BinaryHeap;
use anyhow::*;
use crate::{Cell, GridCell, Solution};
use crate::tools::{Coo};

const TEST: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

/// Flag on the start or end location
#[derive(Debug, Copy, Clone, PartialEq)]
enum Flag {
    Start, End
}

/// Models a terrain tile
#[derive(Debug, Copy, Clone)]
struct Tile {
    height: u8,
    flag: Option<Flag>,
}

/// Area we try to climb
struct AreaClimber {
    tiles: GridCell<Tile>,
    end: Coo,
}

/// Next element to explore with Dijkstra
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Explore {
    coo: Coo,
    score: usize,
}

/// Dijkstra priority queue
type PriorityQueue = BinaryHeap<Explore>;

/// Ordering for [Explore] elements in the [PriorityQueue]
impl Ord for Explore {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

/// Ordering for [Explore] elements in the [PriorityQueue]
impl PartialOrd for Explore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            height: b'a',
            flag: None,
        }
    }
}

impl Cell for Tile {
    fn from_character(c: char) -> Option<Self> {
        match c {
            'a' ..= 'z' => Some(Tile { height: c as u8, flag: None }),
            'S'         => Some(Tile { height: b'a', flag: Some(Flag::Start) }),
            'E'         => Some(Tile { height: b'z', flag: Some(Flag::End) }),
            _           => None
        }
    }

    fn to_char(&self) -> char {
        match (self.height, self.flag) {
            (_, Some(Flag::Start))  => 'S',
            (_, Some(Flag::End))    => 'E',
            _                       => self.height as char
        }
    }
}

impl AreaClimber {
    fn new (content: &[&str]) -> Result<Self> {
        let tiles: GridCell<Tile> = GridCell::new(content)?;

        let end = tiles.find_cell(|tile| tile.flag == Some (Flag::End))
            .ok_or(anyhow!("No end tile found"))?;

        Ok (AreaClimber {
            tiles,
            end,
        })
    }

    /// Return a list of suitable tiles to explore, adjacent to `coo`, when going down
    /// from the top. We must respect the constraint on the difference of altitude.
    fn get_adjacent_tiles_going_down (&self, coo: Coo) -> Vec<Coo> {

        let height = self.tiles.sample(coo).height;

        coo.iter_adjacent_4().filter(|coo| {
            self.tiles.try_sample(*coo).and_then(|tile| {
                Some (tile.height >= height -1)
            }) == Some (true)
        })
        .collect()
    }

    /// Compute the minimum number of steps to walk from the top to the start.
    fn compute_steps_to_top (&self, find_best_start: bool) -> Option<usize> {

        let fn_adjacency = |coo: Coo| {
            self.get_adjacent_tiles_going_down(coo).into_iter()
        };

        // Iter from the end tile by increasing score (distance)
        for (_coo, cell, score) in self.tiles.iter_dijkstra(self.end, fn_adjacency) {

            // Stop condition
            if cell.height == b'a' {
                if cell.flag == Some (Flag::Start) || find_best_start {
                    return Some (score);
                }
            }
        }
        None
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}


/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let area = AreaClimber::new(content)?;

    let steps = area.compute_steps_to_top(false).ok_or(anyhow!("No path found to top"))?;
    Ok(steps)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let area = AreaClimber::new(content)?;

    let steps = area.compute_steps_to_top(true).ok_or(anyhow!("No path found to top"))?;
    Ok(steps)
}

pub fn day_12 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 31);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 29);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}