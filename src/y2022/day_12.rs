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


#[derive(Debug, Copy, Clone, PartialEq)]
enum Flag {
    Start, End
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    height: u8,
    flag: Option<Flag>,
}

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

    fn compute_steps_to_top (&self, find_best_start: bool) -> Option<usize> {

        let mut visited = vec![vec![false; self.tiles.height()]; self.tiles.width()];
        let mut pq = PriorityQueue::new ();

        let end = Explore { coo: self.end, score: 0 };
        pq.push (end);

        while let Some(item) = pq.pop() {

            let current = self.tiles.sample(item.coo);
            let current_height = current.height;

            if current_height == b'a' {
                if current.flag == Some (Flag::Start) || find_best_start {
                    return Some (item.score);
                }
            }

            for coo_next in item.coo.iter_adjacent_4() {
                if let Some(tile) = self.tiles.try_sample(coo_next) {

                    let (x, y) = (coo_next.x as usize, coo_next.y as usize);
                    if visited [x][y] { continue; }
                    if tile.height < current_height -1 { continue; }

                    visited [x][y] = true;
                    pq.push(Explore { coo: coo_next, score: item.score +1 });
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