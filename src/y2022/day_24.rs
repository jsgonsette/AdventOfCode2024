use anyhow::*;
use crate::{Cell, GridCell, Solution};

const TEST: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

type Time = u32;
type Jobs = Vec<ExplorationStep>;

/// The four directions we can move around + stay in place
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up, Down, Left, Right, Stay
}

/// Enables to iterate on all the directions
static DIRECTIONS: &[Direction] = &[
    Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::Stay
];

/// Maze content at some coordinate
#[derive(Default, Copy, Clone, Debug)]
struct MazeCell {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    wall: bool,
}

/// Encodes the static content of the maze at some time
#[derive(Clone)]
struct Maze (GridCell<MazeCell>);

/// Encodes the status of our exploration
struct ExplorationMap {
    
    /// Maze to explore
    maze: Maze,

    /// State of the maze after having found a solution
    maze_evolved: Maze,
}

/// Encodes a state of exploration, with a location and time
#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct ExplorationStep {
    x: usize,
    y: usize,
    t: Time,
}

impl Cell for MazeCell {
    fn from_character (c: char) -> Option<MazeCell> {
        let mut cell = MazeCell::default();
        match c {
            '.' => Some(cell),
            '#' => {
                cell.wall = true;
                Some(cell)
            },
            '<' => {
                cell.left = true;
                Some(cell)
            },
            '>' => {
                cell.right = true;
                Some(cell)
            },
            'v' => {
                cell.down = true;
                Some(cell)
            },
            '^' => {
                cell.up = true;
                Some(cell)
            },
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match (self.wall, self.up, self.down, self.left, self.right) {
            (true, _, _, _, _) => '#',
            (_, false, false, false, false) => '.',
            (_, true, false, false, false) => '^',
            (_, false, true, false, false) => 'v',
            (_, false, false, true, false) => '<',
            (_, false, false, false, true) => '>',
            _ => 'O',
        }
    }

}

impl MazeCell {

    /// Return true if no blizzard at this location
    fn is_empty (&self) -> bool {
        !self.up && !self.down && !self.left && !self.right && !self.wall
    }
}

impl Maze {

    /// New maze instance from puzzle file content
    fn new(content: &[&str]) -> Result<Maze> {

        let area = GridCell::new(content)?;
        Ok(Maze(area))
    }

    /// Get the maze's entry coordinate
    fn entry(&self) -> (usize, usize) { (1, 0) }

    /// Get the maze's exit coordinate
    fn exit(&self) -> (usize, usize) {
        (self.0.width () -2, self.0.height() -1)
    }

    /// Create a new maze by making this one evolve by one minute
    fn evolve (&self) -> Maze {

        // Empty maze
        let mut new_area = GridCell::<MazeCell>::new_empty(self.0.width(), self.0.height());

        for x in 0..self.0.width () {
            for y in 0..self.0.height () {

                let cell = self.0.sample((x, y));

                // Clone the wall
                if cell.wall { new_area.sample_mut((x, y)).wall = true; }

                // Propagate the blizzard
                if cell.up    { new_area.sample_mut((x, self.loop_up    (y))).up = true; }
                if cell.down  { new_area.sample_mut((x, self.loop_down  (y))).down = true; }
                if cell.left  { new_area.sample_mut((self.loop_left  (x), y)).left = true; }
                if cell.right { new_area.sample_mut((self.loop_right (x), y)).right = true; }
            }
        }
        Maze(new_area)
    }

    /// Determine if the given `mov` from `coo` is acceptable given the maze state.
    /// It is acceptable is there is no blizzard nor wall on the landing coordinate.
    /// In this case, return the landing coordinate
    fn can_move (&self, coo: (usize, usize), mov: Direction) -> Option<(usize, usize)> {

        let coo = (coo.0 as isize, coo.1 as isize);
        let (nx, ny) = match (coo, mov) {
            ((x, y), Direction::Stay) => (x, y),
            ((x, y), Direction::Down) => (x, y + 1),
            ((x, y), Direction::Up) => (x, y - 1),
            ((x, y), Direction::Left) => (x - 1, y),
            ((x, y), Direction::Right) => (x + 1, y),
        };

        if nx < 0 || ny < 0 || nx >= self.0.width() as isize || ny >= self.0.height() as isize {
            None
        } else {
            match self.0.sample((nx as usize, ny as usize)).is_empty() {
                true => Some((nx as usize, ny as usize)),
                false => None,
            }
        }
    }

    /// Given the blizzard horizontal location `x`, returns its next position when moving to the left
     fn loop_left (&self, x: usize) -> usize {
        if x <= 1 { self.0.width() - 2 }
        else { x-1 }
    }

    /// Given the blizzard horizontal location `x`, returns its next position when moving to the right
    fn loop_right (&self, x: usize) -> usize {
        if x >= self.0.width() - 2 { 1 }
        else { x + 1 }
    }

    /// Given the blizzard horizontal location `y`, returns its next position when moving to the bottom
    fn loop_down (&self, y: usize) -> usize {
        if y >= self.0.height() - 2 { 1 }
        else { y + 1 }
    }

    /// Given the blizzard horizontal location `y`, returns its next position when moving to the top
    fn loop_up (&self, y: usize) -> usize {
        if y <= 1 { self.0.height() - 2 }
        else { y - 1 }
    }

}

impl ExplorationMap {

    fn from(maze: Maze) -> ExplorationMap {
        ExplorationMap {
            maze_evolved: maze.clone (),
            maze,
        }
    }

    /// Return the number of steps required to join the coordinates `from` and `to`.
    /// If `continuation` is true, the maze initial state is the one reached
    /// during the last call to this function.
    fn solve (&mut self, from: (usize, usize), to: (usize, usize), continuation: bool) -> Time {

        // Jobs for the current time step and for the next one
        let mut jobs = Jobs::new();
        let mut next_jobs = Jobs::new();
        jobs.push(ExplorationStep { x: from.0, y: from.1, t: 0, });

        // Keep track of the visited places for the current time step
        let unvisited = vec![vec![false; self.maze.0.height ()]; self.maze.0.width ()];
        let mut visited = unvisited.clone();

        // Our dynamic maze
        let mut time: Time = 0;
        let mut dyn_maze = match continuation {
            false => self.maze.evolve(),
            true => self.maze_evolved.clone (),
        };

        while !jobs.is_empty() {

            // Extract one item from the exploration steps
            let step = jobs.pop().unwrap();
            let ExplorationStep {x, y, t} = step;

            // Exit found ?
            if x == to.0 && y == to.1 { break; }

            // Test all the directions around
            for direction in DIRECTIONS {

                if let Some ((nx, ny)) = dyn_maze.can_move((x, y), *direction) {
                    if !visited[nx][ny] {

                        next_jobs.push(
                            ExplorationStep { x: nx, y: ny, t: t + 1 }
                        );
                        visited[nx][ny] = true;
                    }
                }
            }

            // When no more items, prepare for the next time step
            if jobs.is_empty() {
                time = t;
                dyn_maze = dyn_maze.evolve();
                std::mem::swap(&mut jobs, &mut next_jobs);
                visited = unvisited.clone();
            }
        }

        self.maze_evolved = dyn_maze;
        time+1
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let maze = Maze::new(content)?;
    let entry = maze.entry();
    let exit = maze.exit();
    let mut exploration_map = ExplorationMap::from(maze);

    let num_steps = exploration_map.solve(entry, exit, false);
    Ok(num_steps as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let maze = Maze::new(content)?;
    let entry = maze.entry();
    let exit = maze.exit();
    let mut exploration_map = ExplorationMap::from(maze);

    let go = exploration_map.solve(entry, exit, false) as usize;
    let back = exploration_map.solve(exit, entry, true) as usize;
    let go_again = exploration_map.solve(entry, exit, true) as usize;

    Ok(go + back + go_again)
}

pub fn day_24 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 18);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 54);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}