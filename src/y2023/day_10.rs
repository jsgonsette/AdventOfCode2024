use std::collections::{HashMap};
use anyhow::*;
use crate::{Cell, CellArea, Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

const TEST_2: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

const TEST_3: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// All the possible pipe types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pipe {
    Empty, Start, Vertical, Horizontal, TopRight, TopLeft, BottomLeft, BottomRight
}

impl Pipe {

    /// Return true for the corners
    fn is_corner (&self) -> bool {
        match self {
            Pipe::TopRight | Pipe::TopLeft => true,
            Pipe::BottomRight | Pipe::BottomLeft => true,
            _ => false,
        }
    }

    /// Return true for a pair of connecting corners that does not reverse the
    /// vertical or horizontal flow.
    fn same_corner_parity (&self, other: &Self) -> bool {
        match self {
            Pipe::BottomRight => *other == Pipe::TopLeft,
            Pipe::TopLeft => *other == Pipe::BottomRight,
            Pipe::BottomLeft => *other == Pipe::TopRight,
            Pipe::TopRight => *other == Pipe::BottomLeft,
            _ => panic!("Not a corner"),
        }
    }
}

/// Encodes a trail as a set of coordinates and [Pipe] pairs.
/// We don't care about the sequence
type Trail = HashMap<Coo, Pipe>;

/// Models the maze of [pipes](Pipe)
struct PipeMaze {

    /// The area containing the pipes
    pipes: CellArea<Pipe>,

    /// The start coordinate
    start: Coo,
}

impl Default for Pipe {
    fn default() -> Self { Pipe::Empty }
}

impl Cell for Pipe {
    fn from_character(c: char) -> Option<Self> {
        match c {
            '.' => Some(Pipe::Empty),
            'S' => Some(Pipe::Start),
            '|' => Some(Pipe::Vertical),
            '-' => Some(Pipe::Horizontal),
            'L' => Some(Pipe::TopRight),
            'J' => Some(Pipe::TopLeft),
            'F' => Some(Pipe::BottomRight),
            '7' => Some(Pipe::BottomLeft),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Pipe::Empty => '.',
            Pipe::Start => 'S',
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::TopRight => 'L',
            Pipe::TopLeft => 'J',
            Pipe::BottomRight => 'F',
            Pipe::BottomLeft => '7',
        }
    }
}

impl PipeMaze {

    /// New pipe maze instance, based on the puzzle file content
    fn new(content: &[&str]) -> Result<PipeMaze> {

        // Build the area of pipes
        let pipes: CellArea<Pipe> = CellArea::new(content)?;

        // Look for the tile with the start coordinate
        let start_cell = pipes.iter_cells()
            .find(|(_x, _y, &cell)| cell == Pipe::Start)
            .ok_or (anyhow!("Start loc not found"))?;

        let start = (start_cell.0, start_cell.1).into();

        Ok(PipeMaze { pipes, start })
    }

    /// Return the kind of type matching a pair of directions
    fn get_start_pipe_type (first_dir: Direction, last_dir: Direction) -> Pipe{

        let pipe = match (first_dir, last_dir) {
            (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => Pipe::Vertical,
            (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => Pipe::Horizontal,
            (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => Pipe::TopRight,
            (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => Pipe::TopLeft,
            (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => Pipe::BottomRight,
            (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => Pipe::BottomLeft,
            _ => unreachable!()
        };

        pipe
    }

    /// Search for a loop, given an initial direction `first_dir`.
    /// If found, the function returns
    /// * the set of all the coordinates that are part of the loop trail.
    /// * The last direction (enables to recover what is hidden by the `S`)
    ///
    /// If no loop could be found, returns `None`
    fn find_loop (&self, first_dir: Direction) -> Option<(Trail, Direction)> {

        let mut loop_trail = Trail::new();
        let mut loc = self.start;
        let mut direction = first_dir;

        loop {

            // Move on in the direction we are facing
            loc = loc.next(direction);
            let Some(pipe) = self.pipes.try_sample(loc) else { break None };
            loop_trail.insert(loc, *pipe);

            // When we reach the start pipe, return the loop we have found
            if *pipe == Pipe::Start {
                break Some ((loop_trail, direction))
            }

            // Look at the pipe at this new location, and check we have a connection.
            // In this case, the new direction is returned
            let new_direction = match (direction, pipe) {
                (Direction::Down, Pipe::TopRight)   => Some (Direction::Right),
                (Direction::Up,   Pipe::BottomLeft) => Some (Direction::Left),
                (Direction::Left, Pipe::TopRight)   => Some (Direction::Up),
                (Direction::Right,Pipe::TopLeft)    => Some (Direction::Up),

                (Direction::Down, Pipe::TopLeft)    => Some (Direction::Left),
                (Direction::Up, Pipe::BottomRight)  => Some (Direction::Right),
                (Direction::Left, Pipe::BottomRight)=> Some (Direction::Down),
                (Direction::Right, Pipe::BottomLeft)=> Some (Direction::Down),

                (Direction::Down, Pipe::Vertical)   => Some (Direction::Down),
                (Direction::Up, Pipe::Vertical)     => Some (Direction::Up),
                (Direction::Left, Pipe::Horizontal) => Some (Direction::Left),
                (Direction::Right, Pipe::Horizontal)=> Some (Direction::Right),

                _ => None,
            };

            // Stop if we don't have a connection with the next pipe
            let Some (new_dir) = new_direction else { break None };
            direction = new_dir;
        }
    }

    /// Compute the number empty tiles that are enclosed by the giant loop
    fn compute_enclosed_area (&self, loop_trail: Trail) -> usize {

        let mut enclosed = 0;

        // Scan the area row by row
        for y in 0..self.pipes.height() {

            // We start outside, and we have not seen any corner yet
            let mut inside = false;
            let mut prev_corner: Option<Pipe> = None;

            // For each row, look at each coordinate from left to right
            for x in 0..self.pipes.width() {

                // Do we cross a vertical part of the loop ?
                if let Some(&pipe) = loop_trail.get(&(x, y).into()) {

                    // Obvious vertical part, we flip the inside flag
                    if pipe == Pipe::Vertical { inside = !inside }

                    // A pair of corner makes us cross the boundary one or two times
                    if pipe.is_corner() {
                        // Second corner
                        if let Some (corner) = prev_corner {

                            // Simple or double-crossing ?
                            if corner.same_corner_parity(&pipe) { inside = !inside; }
                            prev_corner = None;
                        }
                        // The first corner is just recorded and resolved when the second is encountered
                        else {
                            prev_corner = Some(pipe);
                        }
                    }
                }

                // Not on the trail, we use the flag to know if the tile is inside or not
                else if inside { enclosed += 1 }
            }

            assert_eq!(inside, false);
        }

        enclosed
    }
}

/// Solve both parts of the puzzle
fn solve (content: &[&str]) -> Result<(usize, usize)> {

    let maze = PipeMaze::new(content)?;

    // Test the possible directions until we find a loop
    for dir in Direction::iter() {
        if let Some ((mut loop_trail, last_dir)) = maze.find_loop (dir) {

            // Replace the start position 'S' by the correct pipe
            let start_pipe = PipeMaze::get_start_pipe_type(dir, last_dir);
            loop_trail.entry (maze.start).and_modify(|pipe| {*pipe = start_pipe; });

            let distance = loop_trail.len() / 2;
            let enclosed = maze.compute_enclosed_area(loop_trail);

            return Ok((distance, enclosed))
        }
    }

    bail!("No loop found");
}

pub fn day_10 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST)).unwrap_or_default().0 == 8);
    debug_assert!(solve (&split(TEST_2)).unwrap_or_default().1 == 4);
    debug_assert!(solve (&split(TEST_3)).unwrap_or_default().1 == 8);

    let (ra, rb) = solve(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}