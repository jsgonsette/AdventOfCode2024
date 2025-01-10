#[cfg(debug_assertions)]
use std::collections::{HashMap};

use anyhow::*;
use crate::{Cell, GridCell, Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

/// Models the different possible tiles in the [Maze],
/// along with the time needed to reach them
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MazeTile {
    Empty(u32),
    Wall,
    Start,
    End(u32),
}

/// Models the maze, as a set of tiles and start location
struct Maze {
    tiles: GridCell<MazeTile>,
    start: Coo,
}

impl MazeTile {

    /// Return the (uncheated) time to reach this tile
    fn track_time (&self) -> Option<u32> {
        match self {
            MazeTile::Empty(time) => Some(*time),
            MazeTile::End(time) => Some(*time),
            MazeTile::Start => Some(0),
            _ => None,
        }
    }

    /// True if not a wall
    fn is_empty (&self) -> bool {
        match self {
            MazeTile::Wall => false,
            _ => true,
        }
    }
}

impl Default for MazeTile {
    fn default() -> Self {
        MazeTile::Empty(0)
    }
}


impl Cell for MazeTile {
    fn from_character (c: char) -> Option<MazeTile> {
        match c {
            '.' => Some(MazeTile::Empty(0)),
            '#' => Some(MazeTile::Wall),
            'E' => Some(MazeTile::End(0)),
            'S' => Some(MazeTile::Start),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            MazeTile::Empty(_) => '.',
            MazeTile::Wall  => '#',
            MazeTile::Start  => 'S',
            MazeTile::End(_)  => 'E',
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl Maze {

    /// Create a new maze instance based on the puzzle file `content`
    fn new(content: &[&str]) -> Result<Self> {

        // Load the tiles
        let area = GridCell::new(content)?;

        // Recover the start location
        let (xs, ys, _) = area.iter_cells().find(
            |(_x, _y, &cell)| cell == MazeTile::Start
        ).ok_or(anyhow!("No start loc found"))?;

        Ok(Maze {
            tiles: area,
            start: (xs, ys).into(),
        })
    }

    /// Update the time needed to reach the different non-wall tiles of the track.
    fn score_path (&mut self) {

        let mut coo = self.start;
        let mut path_len = 0;

        'outer: loop {
            path_len += 1;

            // Try to move in the 4 direction
            for dir in Direction::iter() {
                let next_coo = coo.next(dir);

                // Look for the first non-wall tile and record the path length
                let next_tile = self.tiles.sample_mut(next_coo);
                match *next_tile {

                    MazeTile::Empty(0) => {
                        *next_tile = MazeTile::Empty(path_len);
                        coo = next_coo;
                        break;
                    }

                    // Stop when we reach the end
                    MazeTile::End(0) => {
                        *next_tile = MazeTile::End(path_len);
                        break 'outer;
                    }

                    _ => continue,
                }
            }
        }
    }

    /// Count the number of unique rat-run cheats across the track that enable  to save
    /// at least `min_time_saved` ps. Parameter `rule` gives the number of ps during which
    /// walls can be removed (e.g. 2 or 20).
    fn count_cheats(&self, min_time_saved: u32, rule: u32) -> usize {

        // Collect the number of cheats of different length in debug
        #[cfg(debug_assertions)]
        let mut info: HashMap<u32, usize> = HashMap::new();

        // Total number of cheats
        let mut count = 0;

        // Look at all the empty tiles
        let track = self.tiles.iter_cells()
            .filter(|(_x, _y, &cell)| cell.is_empty());

        for (x, y, tile) in track {

            // Current tile coordinate and reach time
            let coo: Coo = (x, y).into();
            let Some (start_time) = tile.track_time() else { unreachable!(); };

            // Check all the adjacent non-wall coordinates that are reachable under `rule` pico-sec.
            // For each of them, give their normal arrival time and their distance from `coo`
            let end_times = coo.iter_adjacent_manhattan(rule).filter_map(
                |next_coo| match self.tiles.try_sample(next_coo) {
                    Some(tile) => {
                        let distance = next_coo.manhattan_distance(&coo);
                        if let Some (end_time) = tile.track_time() { Some((end_time, distance))}
                        else { None }
                    },
                    _ => None,
                }
            );

            // Iterate on all the ending tile
            for (end_time, end_distance) in end_times {

                // Check the time we can save worth it
                if end_time <= start_time { continue; }
                let saved_time = end_time - start_time - end_distance;
                if saved_time >= min_time_saved {

                    #[cfg(debug_assertions)]
                    info.entry(saved_time).and_modify(|e| *e += 1).or_insert(1);

                    count += 1;
                }
            }
        }

        //#[cfg(debug_assertions)]
        //dbg!(&info);

        count
    }
}

/// Solve both parts of the puzzle
fn solve (content: &[&str], min_time_save: u32) -> Result<(usize, usize)> {

    let mut maze = Maze::new(content)?;
    maze.score_path();

    let count_2  = maze.count_cheats(min_time_save, 2);
    let count_20 = maze.count_cheats(min_time_save, 20);

    Ok((count_2, count_20))
}

pub fn day_20 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST), 1).unwrap_or_default().0 == 44);
    debug_assert!(solve (&split(TEST), 50).unwrap_or_default().1 == 285);

    let (ra, rb) = solve(content, 100)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}