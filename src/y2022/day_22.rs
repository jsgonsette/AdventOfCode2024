use anyhow::*;
use crate::{Cell, CellArea, Solution};

const TEST: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

/// A single move instruction
#[derive(Debug)]
enum Instruction {
    Move(u32),
    TurnRight,
    TurnLeft,
}

/// The content of the [Board] at some coordinate
#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Void,
    Empty,
    Wall,
}

/// Models the board and its tiles
struct Board {
    area: CellArea<Tile>,
    direction: Direction,
    coo: Coo,
    cube: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    fn to_step (&self) -> Coo {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

type Coo = (isize, isize);

impl Default for Tile {
    fn default() -> Self {
        Tile::Void
    }
}

impl Cell for Tile {
    fn from_character (c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Empty),
            '#' => Some(Tile::Wall),
            ' ' => Some(Tile::Void),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Void => ' ',
        }
    }
}

enum Transform {
    Rot90, Rot180, RotNeg90
}


impl Board {

    /// Create the board from the puzzle file content
    fn new(content: &[&str], cube_mode: bool) -> Result<Board> {

        // Load the board content
        let mut area = CellArea::new(content)?;

        // Start direction and location
        let direction = Direction::Right;
        let coo: Coo = (0..area.width()).find_map(|x| {
            match *area.sample((x, 0)) {
                Tile::Empty => Some ((x as isize, 0)),
                _           => None
            }
        }).ok_or(anyhow!("Could not find entry point"))?;

        Ok(Self {
            area,
            direction,
            coo,
            cube: cube_mode,
        })
    }

    /// Apply the provided list of `instructions` one by one
    fn apply_instructions(&mut self, instructions: &[Instruction]) {

        for ins in instructions {
            match ins {
                Instruction::Move(x) => { self.move_straight(*x) },
                Instruction::TurnRight => { self.direction = self.direction.turn_right()},
                Instruction::TurnLeft => { self.direction = self.direction.turn_left()},
            }
        }
    }

    /// Move straight by a given amount of `steps`.
    fn move_straight (&mut self, steps: u32) {

        let mut coo = self.coo;
        for _ in 0..steps {

            // Make one step
            let next_coo = match self.cube {
                false => self.next_coo_flat(coo, self.direction),
                true  => self.next_coo_cube(coo, self.direction),
            };

            // If the position has not changed, we hit a wall and stop
            if next_coo == coo { break }

            coo = next_coo;
        }

        self.coo = coo;
    }

    /// Make one step in the direction `dir` from the location `coo`.
    /// [Tile::Void] tiles are ignored and the location is left unchanged if a [Tile::Wall] is hit.
    ///
    /// **This function is for the first part where the map is FLAT**
    fn next_coo_flat(&self, coo: Coo, dir: Direction) -> Coo {
        let step = dir.to_step();

        let mut next_coo = coo;
        loop {
            next_coo = self.area.wrap_coo ((next_coo.0 + step.0, next_coo.1 + step.1));
            match self.area.sample((next_coo.0 as usize, next_coo.1 as usize)) {
                Tile::Empty => break next_coo,
                Tile::Wall => break coo,
                Tile::Void => continue,
            }
        }
    }

    /// Make one step in the direction `dir` from the location `coo`.
    /// [Tile::Void] tiles are ignored and the location is left unchanged if a [Tile::Wall] is hit.
    ///
    /// **This function is for the first part where the map is a CUBE**
    fn next_coo_cube(&self, coo: Coo, dir: Direction) -> Coo {

        let step = dir.to_step();
        let (next_coo, next_dir) = self.wrap_cube_coo (
            (coo.0 + step.0, coo.1 + step.1),
            self.direction
        );

        match self.area.sample((next_coo.0 as usize, next_coo.1 as usize)) {
            Tile::Empty => next_coo,
            Tile::Wall  => coo,
            Tile::Void  => unreachable!(),
        }
    }


    fn wrap_cube_coo (&self, coo: Coo, dir: Direction) -> (Coo, Direction) {

        const A: Coo = (1, 0);
        const B: Coo = (1, 1);
        const C: Coo = (1, 2);
        const D: Coo = (0, 3);
        const E: Coo = (0, 2);
        const F: Coo = (2, 0);

        let left  = |(x, y): Coo| (x-1, y);
        let right = |(x, y): Coo| (x+1, y);
        let up    = |(x, y): Coo| (x, y-1);
        let down  = |(x, y): Coo| (x, y+1);

        let face = self.get_face_coo(coo);
        let (new_face, tr) = match (face, dir) {
            (f, Direction::Left) if f == left(A) => (E, Transform::Rot180),
            (f, Direction::Left) if f == left(E) => (A, Transform::Rot180),

            (f, Direction::Left) if f == left(B) => (E, Transform::Rot90),
            (f, Direction::Up)   if f == up  (E) => (B, Transform::RotNeg90),

            (f, Direction::Left) if f == left(D) => (A, Transform::Rot90),
            (f, Direction::Up)   if f == up  (A) => (D, Transform::RotNeg90),

            (f, Direction::Down) if f == down(D) => (F, Transform::Rot180),
            (f, Direction::Up)   if f == up  (F) => (D, Transform::Rot180),

            (f, Direction::Right) if f == right(D) => (C, Transform::Rot90),
            (f, Direction::Down)  if f == down (C) => (D, Transform::RotNeg90),

            (f, Direction::Right) if f == right(C) => (F, Transform::Rot180),
            (f, Direction::Right) if f == right(F) => (C, Transform::Rot180),

            (f, Direction::Right) if f == right(B) => (F, Transform::Rot90),
            (f, Direction::Down)  if f == down (F) => (B, Transform::RotNeg90),

            _ => return (coo, dir) // We are on a valid face, not lost in the emptiness of the manifold
        };

        let cube_width = self.area.width() as isize / 3;
        let off_x = coo.0 % cube_width;
        let off_y = coo.1 % cube_width;

        let (tr_off_x, tr_off_y) = match tr {
            Transform::Rot90 => (off_y, cube_width-1-off_x),
            Transform::Rot180 => (cube_width-1-off_x, cube_width-1-off_y),
            Transform::RotNeg90 => (cube_width-1-off_y, off_x),
        };

        (coo, dir)
    }

    /// Compute a coordinate reflecting in which face of the cube we are.
    /// e.g.: A -> (1, 0)
    ///
    /// ```
    ///     0   1   2
    ///       +---+---+
    /// 0     | A | F |
    ///       +---+---+
    /// 1     | B |
    ///   +---+---+
    /// 2 | E | C |
    ///   +---+---+
    /// 3 | D |
    ///   +---+
    /// ```
    fn get_face_coo(&self, coo: Coo) -> Coo {

        let cube_width = self.area.width() as isize / 3;
        let x = coo.0 / cube_width + if coo.0 < 0 { -1 } else { 0 };
        let y = coo.1 / cube_width + if coo.1 < 0 { -1 } else { 0 };
        (x, y)
    }


    /// Compute the password from the current position
    fn password (&self) -> usize {
        let num_dir = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        (1 + self.coo.1 as usize) * 1000 + (1 + self.coo.0 as usize) * 4 + num_dir
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Load the vector of instructions for a single line `row`
fn load_instructions(row: &str) -> Result<Vec<Instruction>> {

    let mut instructions = Vec::new();

    let mut number: Option<u32> = None;
    for c in row.chars() {
        match c {
            'R' => {
                if let Some (n) = number { instructions.push(Instruction::Move(n)); }
                number = None;
                instructions.push(Instruction::TurnRight)
            },
            'L' => {
                if let Some (n) = number { instructions.push(Instruction::Move(n)); }
                number = None;
                instructions.push(Instruction::TurnLeft)
            },
            _ => {
                if let Some (digit) = c.to_digit(10) {
                    number = Some (number.unwrap_or_default() * 10 + digit);
                }
                else { bail!("Invalid character in instructions"); }
            },
        }
    }
    if let Some (n) = number { instructions.push(Instruction::Move(n)); }

    Ok (instructions)
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut board = Board::new(content, false)?;
    let instructions = load_instructions(content [board.area.height()+1])?;

    board.apply_instructions(&instructions);
    Ok(board.password())
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let mut board = Board::new(content, true)?;
    let instructions = load_instructions(content [board.area.height()+1])?;

    //println!("Board: {}", board.area);
    //println!("instruction: {:?}", instructions);

    board.apply_instructions(&instructions);
    Ok(board.password())
}

pub fn day_22 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 6032);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 5031);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}