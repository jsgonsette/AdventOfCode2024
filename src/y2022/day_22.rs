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

impl Board {

    /// Create the board from the puzzle file content
    fn new(content: &[&str]) -> Result<Board> {

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
        })
    }

    fn apply_instructions(&mut self, instructions: &[Instruction]) {

        for ins in instructions {
            match ins {
                Instruction::Move(x) => { self.move_straight(*x) },
                Instruction::TurnRight => { self.direction = self.direction.turn_right()},
                Instruction::TurnLeft => { self.direction = self.direction.turn_left()},
            }
            //println!("Position: {:?}, {:?}", self.coo, self.direction);
        }
    }

    fn move_straight (&mut self, steps: u32) {

        let mut coo = self.coo;
        for _ in 0..steps {
            let next_coo = self.next_coo(coo, self.direction);
            if next_coo == coo { break }
            coo = next_coo;
        }

        self.coo = coo;
    }

    /// Make one step in the direction `dir` from the location `coo`.
    /// [Tile::Void] tiles are ignored and the location is left unchanged if a [Tile::Wall] is hit.
    fn next_coo (&self, coo: Coo, dir: Direction) -> Coo {
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

    let mut board = Board::new(content)?;
    let instructions = load_instructions(content [board.area.height()+1])?;

    //println!("Board: {}", board.area);
    //println!("instruction: {:?}", instructions);

    board.apply_instructions(&instructions);
    Ok(board.password())
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_22 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 6032);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}