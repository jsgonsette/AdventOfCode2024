use std::cmp::Ordering;
use std::fmt::Display;
use anyhow::*;
use itertools::Itertools;
use crate::Solution;

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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up, Down, Left, Right, Stay
}

static DIRECTIONS: &[Direction] = &[
    Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::Stay
];

/// Maze content at some coordinate
#[derive(Default, Copy, Clone, Debug)]
struct Cell {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    wall: bool,
}

/// Encodes the static content of the maze at some time
#[derive(Default, Clone, Debug)]
struct Maze {

    /// Maze's cells
    cells: Vec<Cell>,

    /// Dimensions
    width: usize,
    height: usize,
}

/// Encodes the status of our exploration
struct ExplorationMap {
    
    /// Maze to explore
    maze: Maze,

    /// State of the maze after having found a solution
    maze_evolved: Maze,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct ExplorationStep {
    x: usize,
    y: usize,
    t: Time,
}


impl PartialOrd<Self> for ExplorationStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExplorationStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Lower time comes first
        other.t.cmp(&self.t)
    }
}

impl Cell {
    fn from_character (c: char) -> Option<Cell> {
        let mut cell = Cell::default();
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

    fn is_empty (&self) -> bool {
        !self.up && !self.down && !self.left && !self.right && !self.wall
    }
}

/// To help debugging
impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for y in 0..self.height {
            let row: String = (0..self.width).map(|x| {
               self.sample((x, y)).to_char()
            }).join("");

            f.write_str("\n")?;
            f.write_str(&row)?;
        }
        f.write_str("\n")
    }
}

impl Maze {

    /// New maze instance from puzzle file content
    fn new(content: &[&str]) -> Result<Maze> {

        let height = content.len();
        let width = content[0].len();
        let cells = Self::load_cell_from_content(content)?;

        Ok(Maze { cells, width, height, })
    }

    fn entry(&self) -> (usize, usize) { (1, 0) }

    fn exit(&self) -> (usize, usize) {
        (self.width -2, self.height -1)
    }

    /// Create a new maze by making this one evolve by one minute
    fn evolve (&self) -> Maze {

        // Empty maze
        let cells = vec! [Cell::default(); self.width * self.height];
        let mut maze = Maze { cells, width: self.width, height: self.height };

        for x in 0..self.width {
            for y in 0..self.height {

                let cell = self.sample((x, y));

                // Clone the wall
                if cell.wall { maze.sample_mut((x, y)).wall = true; }

                // Propagate the blizzard
                if cell.up    { maze.sample_mut((x, self.loop_up    (y))).up = true; }
                if cell.down  { maze.sample_mut((x, self.loop_down  (y))).down = true; }
                if cell.left  { maze.sample_mut((self.loop_left  (x), y)).left = true; }
                if cell.right { maze.sample_mut((self.loop_right (x), y)).right = true; }
            }
        }
        maze
    }

    /// Create the vector of cells used to encode the maze from the puzzle file `content`
    fn load_cell_from_content (content: &[&str]) -> Result<Vec<Cell>> {

        let width = content[0].len();

        // Make a single vector of cells to encode the maze
        let cells: Option<Vec<Cell>> = content.iter().flat_map (|row| {
            row.as_bytes().iter().map(|&b| {
                if row.len() == width {
                    Cell::from_character(b as char)
                }
                else { None }
            })
        }).collect();

        cells.ok_or(anyhow!("Invalid content"))
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

        if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
            None
        } else {
            match self.sample((nx as usize, ny as usize)).is_empty() {
                true => Some((nx as usize, ny as usize)),
                false => None,
            }
        }
    }

    /// Get the cell at some location `coo`
    fn sample (&self, coo: (usize, usize)) -> Cell {
        self.cells[coo.1 * self.width + coo.0]
    }

    fn sample_mut(&mut self, coo: (usize, usize)) -> &mut Cell {
        &mut self.cells[coo.1 * self.width + coo.0]
    }

    fn loop_left (&self, x: usize) -> usize {
        if x <= 1 { self.width - 2 }
        else { x-1 }
    }

    fn loop_right (&self, x: usize) -> usize {
        if x >= self.width - 2 { 1 }
        else { x + 1 }
    }

    fn loop_down (&self, y: usize) -> usize {
        if y >= self.height - 2 { 1 }
        else { y + 1 }
    }

    fn loop_up (&self, y: usize) -> usize {
        if y <= 1 { self.height - 2 }
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
        let unvisited = vec![vec![false; self.maze.height]; self.maze.width];
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