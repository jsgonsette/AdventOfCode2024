use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use anyhow::*;
use crate::{Cell, GridCell, Solution};
use crate::tools::{Coo, Direction, IntReader};

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

/// Diffusion set for the alternate method of part 2 (same size of the memory space area)
type DiffuseSet = Vec<Vec<bool>>;

/// A tile of the memory space. When corrupted, we record at which time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MemoryTile {
    Safe, Corrupted(u32),
}

/// Models the memory corrupted maze
struct MemorySpace {
    area: GridCell<MemoryTile>,
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
        let area = GridCell::new_empty(width, height);

        Self { area }
    }

    /// Fill the maze with `num_corruptions` corrupted tiles, according to the puzzle
    /// file `content`
    fn fill_space (&mut self, content: &[&str]) -> Result<()> {

        let mut reader = IntReader::new(false);

        for (idx, &row) in content.iter().enumerate() {
            let location: [usize; 2] = reader.process_row_fix(row)
                .ok_or(anyhow!("Invalid row: {}", row))?;

            let coo: Coo = (location[0], location[1]).into();
            *self.area.sample_mut(coo) = MemoryTile::Corrupted (1 + idx as u32);
        }

        Ok(())
    }

    /// Return an iterator on the corruption coordinates, in the order they appear.
    fn get_corruption_it<'a> (content: &'a[&'a str]) -> impl DoubleEndedIterator<Item=Result<Coo>> +'a {

        let mut reader = IntReader::new(false);

        // Iterate on the rows
        content.iter().map (move |&row| {

            // Read the two values and convert them into a coordinate
            let location: [usize; 2] = reader.process_row_fix(row)
                .ok_or(anyhow!("Invalid row: {}", row))?;
            Ok((location[0], location[1]).into())
        })
    }

    /// Memory space entry
    fn entry (&self) -> Coo {
        (0usize, 0usize).into()
    }

    /// Memory space exit
    fn exit (&self) -> Coo {
        (self.area.width()-1, self.area.height()-1).into()
    }

    /// Do a Dijkstra search to compute the number of steps required to reach the exit tile.
    /// The parameter `num_corruptions` activates this first equivalent amount of blocks, other
    /// are ignored.
    fn compute_num_steps_to_exit (&self, num_corruptions: u32) -> Option<usize> {

        let mut visited = vec![vec![false; self.area.height()]; self.area.width()];
        let exit = self.exit();

        let mut pq = PriorityQueue::new ();
        let start = Explore { coo: self.entry(), score: 0 };
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

    /// Extend a `set` of empty cells from the provided `coo`
    fn diffuse_from (&self, coo: Coo, set: &mut DiffuseSet) {

        // Init the queue with the initial coordinate
        let mut queue = Vec::<Coo>::new ();
        queue.push (coo);
        set [coo.x as usize][coo.y as usize] = true;

        while let Some (coo) = queue.pop() {

            // For each coordinate, check the 4 neighbors
            for dir in Direction::iter() {
                let next_coo = coo.next(dir);
                let x = next_coo.x as usize;
                let y = next_coo.y as usize;

                // If safe and if not in the set, add it and pursue the diffusion
                match self.area.try_sample(next_coo) {
                    Some(MemoryTile::Safe) if !set [x][y] => {
                        set [x][y] = true;
                        queue.push (next_coo);
                    }
                    _ => {}
                }
            }
        }
    }

}

/// Find the cutting block of part 2 with an alternate method. Here we expand two areas of
/// empty cells from the start and the end. Then we remove the block one by one and check
/// when both areas meet.
fn find_cutting_block (content: &[&str], space: &mut MemorySpace) -> Result<Coo> {

    // Build the diffusion sets from the start and the exit
    let mut start_set = vec![vec![false; space.area.height()]; space.area.width()];
    let mut end_set = vec![vec![false; space.area.height()]; space.area.width()];
    space.diffuse_from(space.entry(), &mut start_set);
    space.diffuse_from(space.exit(), &mut end_set);

    // Iterate on the blocks in reverse
    for coo in MemorySpace::get_corruption_it(content).rev() {

        // Remove the block
        let coo: Coo = coo?;
        *space.area.sample_mut(coo) = MemoryTile::Safe;

        // Check the tiles adjacent to the removed block to determine if they touch the diffusion areas
        let touching_it = coo.iter_adjacent_4().map (|next_coo| {
            if space.area.is_inside(next_coo) {
                let (nx, ny) = (next_coo.x as usize, next_coo.y as usize);

                let touch_start = start_set[nx][ny];
                let touch_end = end_set[nx][ny];
                (touch_start, touch_end)
            }
            else { (false, false) }
        });

        let (touch_start, touch_end) = touching_it.fold (
            (false, false),
            |(any_start, any_end), (start, end)| {
                (any_start || start, any_end || end)
            }
        );

        // Check the result
        match (touch_start, touch_end) {

            // If the block touches both sets, we are done.
            (true, true) => { return Ok(coo); }

            // Otherwise, we extend the diffusion areas.
            (true, _) => { space.diffuse_from(coo, &mut start_set); },
            (_, true) => { space.diffuse_from(coo, &mut end_set); },
            _         => {},
        }
    }

    Err(anyhow!("Cutting block Not found"))
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
    let mut reader = IntReader::new(false);
    let loc: [usize; 2] = reader.process_row_fix(content[num_corruptions - 1]).unwrap();

    let loc_string = format!("{},{}", loc[0], loc[1]);
    Ok(loc_string)
}

/// Solve second part of the puzzle, with a memory space of size `width` x `height`.
/// Use an alternative method based on diffusion areas extended from both the entry and the exit
fn part_b_alt(content: &[&str], width: usize, height: usize) -> Result<String> {

    // Instantiate the memory space with all the blocks
    let mut space = MemorySpace::new(width, height);
    space.fill_space(content)?;

    // Find the cutting block
    let cutting_block = find_cutting_block(content, &mut space)?;

    let loc_string = format!("{},{}", cutting_block.x, cutting_block.y);
    Ok(loc_string)
}

pub fn day_18 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST), 7, 7, 12).unwrap_or_default() == 22);
    debug_assert!(part_b (&split(TEST), 7, 7, 12).unwrap_or_default() == "6,1");
    debug_assert!(part_b_alt(&split(TEST), 7, 7).unwrap_or_default() == "6,1");

    let ra = part_a(content, 71, 71, 1024)?;
    let rb = part_b(content, 71, 71, 1024)?;

    Ok((Solution::Unsigned(ra), Solution::Text(rb)))
}