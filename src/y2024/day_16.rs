use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use anyhow::*;
use crate::{Cell, GridCell, Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

type Score = usize;

/// Location in maze: coordinate + direction
type Location = (Coo, Direction);

/// All the possible ancestor locations at some point on an optimal path
/// (for part 2 we can have multiple ones)
type Ancestors = Vec<Location>;

/// All the visited locations with their score and ancestors
type History = HashMap<Location, (Score, Ancestors)>;

/// A location to explore, with its score and its path ancestor
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Explore {
    loc: Location,
    score: Score,
    previous: Option<Location>,
}

/// Dijkstra priority queue, made of sorted [locations to explore](Explore)
type PriorityQueue = BinaryHeap<Explore>;

/// Models the different possible tiles in the [Maze]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MazeTile {
    Empty,
    Wall,
    Start,
    End,
}

/// Models the maze, as a set of tiles and start location
struct Maze {
    tiles: GridCell<MazeTile>,
    start: Location,
}

/// Implements an ordering for the [priority queue](PriorityQueue)
impl Ord for Explore {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

/// Implements an ordering for the [priority queue](PriorityQueue)
impl PartialOrd for Explore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for MazeTile {
    fn default() -> Self {
        MazeTile::Empty
    }
}


impl Cell for MazeTile {
    fn from_character (c: char) -> Option<MazeTile> {
        match c {
            '.' => Some(MazeTile::Empty),
            '#' => Some(MazeTile::Wall),
            'E' => Some(MazeTile::End),
            'S' => Some(MazeTile::Start),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            MazeTile::Empty => '.',
            MazeTile::Wall  => '#',
            MazeTile::Start  => 'S',
            MazeTile::End  => 'E',
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl Maze {

    /// Create a new maze instance based on the puzzle file `content`
    fn new (content: &[&str]) -> Result<Self> {

        // Load the tiles
        let area = GridCell::new(content)?;

        // Recover the start location
        let (xs, ys, _) = area.iter_cells().find(
            |(_x, _y, &cell)| cell == MazeTile::Start
        ).ok_or(anyhow!("No start loc found"))?;

        Ok (Maze {
            tiles: area,
            start: ((xs, ys).into(), Direction::Right),
        })
    }

    /// Given the `arrival` location and the history of `visited` locations, count
    /// the number of coordinates that are on one of the multiple best paths.
    fn count_best_paths_locations (arrival: Location, visited: &History) -> usize {

        let mut spots: HashSet<Coo> = HashSet::new ();
        let mut queue:Vec<Location> = vec![];

        // Init the queue with the arrival location and all its possible ancestors
        spots.insert(arrival.0);
        for dir in Direction::iter() {
            let loc = (arrival.0, dir);
            if let Some ((_score, ancestors)) = visited.get(&loc) {
                for ancestor in ancestors { queue.push(*ancestor); }
            }
        }

        // For each location in the queue
        while let Some(loc) = queue.pop() {

            // Record the coordinate
            spots.insert(loc.0);

            // Insert all the possible predecessors
            if let Some ((_score, ancestors)) = visited.get(&loc) {
                for ancestor in ancestors {
                    queue.push(*ancestor);
                }
            }
        }

        spots.len()
    }

    /// Update the `history` of visited locations with a new exploration element `explore`.
    /// Return true if we see this element for the first time.
    fn update_history (history: &mut History, explore: &Explore) -> bool {

        // If this location has already been visited...
        if let Some ((score, ancestors)) = history.get_mut(&explore.loc) {

            // ... keep track of the ancestor if the score is equal
            if explore.score == *score {
                ancestors.push(explore.previous.unwrap());
            }
            false
        }
        // Otherwise, add this element to the history
        else {
            let ancestors = match explore.previous {
                None => vec![],
                Some(loc) => vec![loc],
            };
            history.insert(explore.loc, (explore.score, ancestors));
            true
        }
    }

    /// Given the current element `explore`, schedule to visit the location in front (if it is not a wall).
    fn explore_ahead (&self, explore: &Explore, pq: &mut PriorityQueue) {

        let current_dir = explore.loc.1;
        let next_coo = explore.loc.0.next(current_dir);
        let next_loc = (next_coo, current_dir);
        let next_score = explore.score+1;

        // If not a wall, program its exploration
        if let Some (tile) = self.tiles.try_sample(next_coo) {
            if *tile != MazeTile::Wall {

                let to_explore = Explore {
                    loc: next_loc,
                    score: next_score,
                    previous: Some(explore.loc)
                };
                pq.push(to_explore);
            }
        }
    }

    /// Given the current element `explore`, schedule to visit the two 90° rotations
    fn explore_turns  (&self, explore: &Explore, pq: &mut PriorityQueue) {

        let current_dir = explore.loc.1;

        for dir in Direction::iter() {
            if dir == current_dir || dir == current_dir.flip() { continue }

            let next_loc = (explore.loc.0, dir);
            let next_score = explore.score+1000;

            let to_explore = Explore { loc: next_loc, score: next_score, previous: Some(explore.loc) };
            pq.push(to_explore);
        }
    }

    /// Solve the maze by searching for all the possible nearest paths that reach the end tile.
    /// The function returns
    /// * a [History] containing, for all the visited tiles, its score and its possible predecessors.
    /// * The coordinate/direction of the arrival tile
    fn solve (&self) -> (History, Location) {

        // To keep track of visited locations
        let mut visited = History::new();

        // Dijkstra PQ, starting at the start location
        let mut pq = PriorityQueue::new ();
        let start = Explore { loc: self.start, score: 0, previous:None };
        pq.push (start);

        // Search loop
        let mut arrival: Option<(Location, usize)> = None;
        while let Some(explore) = pq.pop() {

            // Update the history with the next element to explore. Skip it if we have
            // already seen it.
            let new_element = Self::update_history(&mut visited, &explore);
            if !new_element { continue}

            // Check for arrival. Record location and score but do not stop
            if *self.tiles.sample(explore.loc.0) == MazeTile::End {
                arrival = Some((explore.loc, explore.score));
            }

            // Stop when we have found the arrival and when the queue only contains
            // locations with worse scores
            if let Some ((_loc, score)) = arrival {
                if explore.score > score { break }
            }

            // Explore the location one step ahead
            self.explore_ahead (&explore, &mut pq);

            // Try the 90° rotations
            self.explore_turns(&explore, &mut pq);
        }

        let Some ((loc, _score)) = arrival else { panic!("No solution found")};
        (visited, loc)
    }
}

/// Solve both parts of the puzzle
fn solve (_content: &[&str]) -> Result<(usize, usize)> {

    // Build and solve the maze
    let maze = Maze::new(_content)?;
    let (history, arrival_loc) = maze.solve();

    // Retrieve the path len from the history
    let Some (arrival_entry) = history.get(&arrival_loc) else { bail!("No solution found") };
    let path_len = arrival_entry.0;

    // Count the number of coordinates that are on one of the best paths
    let best_loc_count = Maze::count_best_paths_locations (arrival_loc, &history);

    Ok((path_len, best_loc_count))
}


pub fn day_16 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST)).unwrap_or_default() == (7036, 45));

    let (ra, rb) = solve (content)?;
    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}