use std::cmp::Ordering;
use std::collections::BinaryHeap;
use anyhow::*;
use crate::{Cell, CellArea, Solution};
use crate::tools::{Coo, Direction, RowReader};

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

/// A tile of the memory space. When corrupted, we record at which time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MemoryTile {
    Safe, Corrupted(u32),
}

/// Models the memory corrupted maze
struct MemorySpace {
    area: CellArea<MemoryTile>,
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

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl Default for MemoryTile {
    fn default () -> Self {
        MemoryTile::Safe
    }
}

impl Cell for MemoryTile {
    fn from_character(_c: char) -> Option<Self> {
        todo!()
    }

    fn to_char(&self) -> char {
        match self {
            MemoryTile::Safe => '.',
            MemoryTile::Corrupted(_) => '#',
        }
    }
}

impl MemorySpace {

    /// New empty instance of given `width` and `height`
    fn new (width: usize, height: usize) -> Self {
        let area = CellArea::new_empty(width, height);

        Self { area }
    }

    /// Fill the maze with `num_corruptions` corrupted tiles, according to the puzzle
    /// file `content`
    fn fill_space (&mut self, content: &[&str]) -> Result<()> {

        let mut reader = RowReader::new(false);

        for (idx, &row) in content.iter().enumerate() {
            let location: [usize; 2] = reader.process_row_fix(row)
                .ok_or(anyhow!("Invalid row: {}", row))?;

            let coo: Coo = (location[0], location[1]).into();
            *self.area.sample_mut(coo) = MemoryTile::Corrupted (1 + idx as u32);
        }

        Ok(())
    }

    fn get_corruption_it<'a> (content: &'a[&'a str]) -> impl Iterator<Item=Result<Coo>> +'a {

        let mut reader = RowReader::new(false);

        // Iterate on the rows
        content.iter().map (move |&row| {

            // Read the two values and convert them into a coordinate
            let location: [usize; 2] = reader.process_row_fix(row)
                .ok_or(anyhow!("Invalid row: {}", row))?;
            Ok((location[0], location[1]).into())
        })
    }

    /// Do a Dijkstra search to compute the number of steps required to reach the exit tile.
    /// The parameter `num_corruptions` activates this first equivalent amount of blocks, other
    /// are ignored.
    fn compute_num_steps_to_exit (&self, num_corruptions: u32) -> Option<usize> {

        let mut visited = vec![vec![false; self.area.height()]; self.area.width()];
        let exit: Coo = (self.area.width()-1, self.area.height()-1).into();

        let mut pq = PriorityQueue::new ();
        let start = Explore { coo: Coo { x: 0, y: 0 }, score: 0 };
        pq.push (start);

        while let Some (Explore { coo, score }) = pq.pop() {

            if coo == exit { return Some(score); }

            for dir in Direction::iter() {
                let next_coo = coo.next(dir);
                let nx = next_coo.x as usize;
                let ny = next_coo.y as usize;

                if let Some(tile) = self.area.try_sample(next_coo) {

                    if let MemoryTile::Corrupted(time) = *tile {
                        if time <= num_corruptions { continue; }
                    }
                    if visited[nx][ny] { continue; }

                    visited[nx][ny] = true;
                    pq.push(Explore { coo: next_coo, score: score +1 });
                }
                else { continue; }
            }
        }

        None
    }
}

/// Solve first part of the puzzle, with a memory space of size `width` x `height`.
/// Parameter `num_corruptions` activates this amount of corrupted blocks
fn part_a (content: &[&str], width: usize, height: usize, num_corruptions: usize) -> Result<usize> {

    let mut space = MemorySpace::new(width, height);
    space.fill_space(content)?;
    let num_steps = space.compute_num_steps_to_exit(num_corruptions as u32).ok_or(anyhow!("No path found"))?;

    Ok(num_steps)
}

/// Solve second part of the puzzle, with a memory space of size `width` x `height`.
/// Parameter `num_corruptions_start` set a low bound for which we know a path exists.
fn part_b (content: &[&str], width: usize, height: usize, num_corruptions_start: usize) -> Result<String> {

    let mut space = MemorySpace::new(width, height);
    space.fill_space(content)?;

    // Check if the maze with `num_corruptions` blocks has a solution
    let has_path = |num_corruptions: &usize| {
        space.compute_num_steps_to_exit(*num_corruptions as u32).is_some()
    };

    // Binary search over the possible range
    let search_slice: Vec<_> = (num_corruptions_start..content.len()).collect();
    let first_blocked_path = search_slice.partition_point(has_path);
    let num_corruptions = search_slice[first_blocked_path];

    // Retrieve the corresponding location
    let mut reader = RowReader::new(false);
    let loc: [usize; 2] = reader.process_row_fix(content[num_corruptions - 1]).unwrap();

    let loc_string = format!("{},{}", loc[0], loc[1]);
    Ok(loc_string)
}

fn part_b2 (content: &[&str], width: usize, height: usize) -> Result<String> {

    let mut space = MemorySpace::new(width, height);
    space.fill_space(content)?;

    Ok("".to_string())
}

pub fn day_18 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST), 7, 7, 12).unwrap_or_default() == 22);

    let ra = part_a(content, 71, 71, 1024)?;
    let rb = part_b(content, 71, 71, 1024)?;

    Ok((Solution::Unsigned(ra), Solution::Text(rb)))
}