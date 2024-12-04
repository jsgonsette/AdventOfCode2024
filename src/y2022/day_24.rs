use std::cmp::Ordering;
use std::fmt::Display;
use std::collections::BinaryHeap;
use anyhow::*;
use itertools::Itertools;

const TEST: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

type Time = u32;
type PQ = BinaryHeap<ExplorationStep>;

enum Direction {
    Up, Down, Left, Right, Stay
}

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
    
    /// Best time to reach all the maze's cells
    reached: Vec<Option<Time>>,
}

#[derive(Eq, PartialEq, Debug)]
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

    fn can_move (&self, coo: (usize, usize), mov: Direction) -> bool {

        let coo = (coo.0 as isize, coo.1 as isize);
        let (nx, ny) = match (coo, mov) {
            ((x, y), Direction::Stay) => (x, y),
            ((x, y), Direction::Down) => (x, y + 1),
            ((x, y), Direction::Up) => (x, y - 1),
            ((x, y), Direction::Left) => (x - 1, y),
            ((x, y), Direction::Right) => (x + 1, y),
        };

        if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
            false
        } else {
            self.sample((nx as usize, ny as usize)).is_empty()
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
        let width = maze.width;
        let height = maze.height;

        ExplorationMap {
            maze,
            reached: vec![None; height * width],
        }
    }

    fn solve (&mut self, (x, y): (usize, usize)) -> Time {

        let mut pq = PQ::new();
        pq.push(ExplorationStep { x, y, t: 0, });
        self.set_time_if_better((x, y), 0);

        let mut dyn_maze = self.maze.evolve();
        let mut time: Time = 0;
        //println!("{}", self.maze);

        while !pq.is_empty() {
            let ExplorationStep {x, y, t} = pq.pop().unwrap();

            if t > time {
                //println!("{}", dyn_maze);
                dyn_maze = dyn_maze.evolve();
                time = t;
            }
            //println!("x: {}, y: {}, t: {}", x, y, t);

            if x == self.maze.width -2 && y == self.maze.height -1 { break; }

            if dyn_maze.can_move((x, y), Direction::Stay) {
                pq.push(ExplorationStep { x, y, t: t + 1, });
            }
            if dyn_maze.can_move((x, y), Direction::Up) {
                pq.push(ExplorationStep { x, y: y-1, t: t + 1, });
            }
            if dyn_maze.can_move((x, y), Direction::Down) {
                pq.push(ExplorationStep { x, y: y+1, t: t + 1, });
            }
            if dyn_maze.can_move((x, y), Direction::Left) {
                pq.push(ExplorationStep { x: x-1, y, t: t + 1, });
            }
            if dyn_maze.can_move((x, y), Direction::Right) {
                pq.push(ExplorationStep { x: x+1, y, t: t + 1, });
            }
        }

        time
    }

    fn set_time_if_better (&mut self, (x, y): (usize, usize), t: Time) -> bool {
        if x == 0 || x >= self.maze.width - 1 || y == 0 || y >= self.maze.height - 1 { return false }

        let reached = &mut self.reached[y * self.maze.width + x];
        match reached {
            None => {
                *reached = Some (t);
                true
            },
            Some(prev_t) if t < *prev_t => {
                *reached = Some (t);
                true
            }
            _ => { false }
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let maze = Maze::new(content)?;
    let mut exploration_map = ExplorationMap::from(maze);
    let num_steps = exploration_map.solve((1, 0));

    Ok(num_steps as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_24 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 18);
    //debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((ra, rb))
}