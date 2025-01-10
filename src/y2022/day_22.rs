use anyhow::*;
use crate::{Cell, GridCell, Solution};
use crate::tools::{Coo, Direction};

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

    /// All the tiles of the puzzle
    area: GridCell<Tile>,

    /// Current moving direction
    direction: Direction,

    /// Current location
    coo: Coo,

    /// Flat or Cube mode ?
    cube: bool,
}

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

/// Rotations modeling the jumps between the faces of a cube
enum Transform {
    Rot90, Rot180, RotNeg90, Rot0
}


impl Board {

    /// Create the board from the puzzle file content. Parameter `cube_mode` can
    /// be set to true for the second part of the puzzle, where we deal with a cube.
    fn new(content: &[&str], cube_mode: bool) -> Result<Board> {

        // Load the board content
        let area = GridCell::new(content)?;

        // Start direction and location
        let direction = Direction::Right;
        let coo: Coo = (0..area.width()).find_map(|x| {
            match *area.sample((x, 0)) {
                Tile::Empty => Some ((x as isize, 0).into ()),
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
                Instruction::TurnRight => { self.direction = self.direction.to_right(); },
                Instruction::TurnLeft => { self.direction = self.direction.to_left(); },
            }
        }
    }

    /// Move straight by a given amount of `steps`.
    fn move_straight (&mut self, steps: u32) {

        let mut coo = self.coo;
        let mut dir = self.direction;

        for _ in 0..steps {

            // Make one step
            let (next_coo, next_dir) = match self.cube {
                false => self.next_coo_flat(coo, dir),
                true  => self.next_coo_cube(coo, dir),
            };

            // If the position has not changed, we hit a wall and stop
            if next_coo == coo { break }

            coo = next_coo;
            dir = next_dir;
        }

        self.coo = coo;
        self.direction = dir;
    }

    /// Make one step in the direction `dir` from the location `coo`, if possible.
    /// [Tile::Void] tiles are ignored and the location is left unchanged if a [Tile::Wall] is hit.
    ///
    /// **This function is for the first part where the map is FLAT**
    fn next_coo_flat(&self, coo: Coo, dir: Direction) -> (Coo, Direction) {

        let mut next_coo = coo;
        loop {
            next_coo = next_coo.next(dir).wrap_around_area(self.area.width(), self.area.height());
            match self.area.sample(next_coo) {
                Tile::Empty => break (next_coo, dir),
                Tile::Wall => break (coo, dir),
                Tile::Void => continue,
            }
        }
    }

    /// Make one step in the direction `dir` from the location `coo`, if possible
    /// [Tile::Void] tiles are ignored and the location is left unchanged if a [Tile::Wall] is hit.
    ///
    /// **This function is for the first part where the map is a CUBE**
    fn next_coo_cube(&self, coo: Coo, dir: Direction) -> (Coo, Direction) {

        // Compute the next location and direction by wrapping them around the cube
        let (next_coo, next_dir) = self.wrap_cube_coo (
            coo.next(dir),
            dir
        );

        match self.area.sample(next_coo) {
            Tile::Empty => (next_coo, next_dir),
            Tile::Wall  => (coo, dir),
            Tile::Void  => unreachable!(),
        }
    }

    /// Wrap the provided location `coo` and direction `dir` around the cube when needed.
    /// The location and direction are left unchanged if they correspond to any cube's face.
    fn wrap_cube_coo (&self, coo: Coo, dir: Direction) -> (Coo, Direction) {

        // See function 'get_face_coo' for this naming. They are all the valid face coordinates
        const A: Coo = Coo { x: 1, y: 0 };
        const B: Coo = Coo { x: 1, y: 1 };
        const C: Coo = Coo { x: 1, y: 2 };
        const D: Coo = Coo { x: 0, y: 3 };
        const E: Coo = Coo { x: 0, y: 2 };
        const F: Coo = Coo { x: 2, y: 0 };

        // Get the face coordinate
        let face_coo = self.get_face_coo(coo);

        // Handle the cases when this face is not a valid one (i.e. A -> F).
        // In this case, we get a new face and some rotation to compute to map the coordinate
        let (new_face, tr) = match (face_coo, dir) {
            (f, Direction::Left) if f == A.next(Direction::Left) => (E, Transform::Rot180),
            (f, Direction::Left) if f == E.next(Direction::Left) => (A, Transform::Rot180),

            (f, Direction::Left) if f == B.next(Direction::Left) => (E, Transform::Rot90),
            (f, Direction::Up)   if f == E.next(Direction::Up)   => (B, Transform::RotNeg90),

            (f, Direction::Left) if f == D.next(Direction::Left) => (A, Transform::Rot90),
            (f, Direction::Up)   if f == A.next(Direction::Up)   => (D, Transform::RotNeg90),

            (f, Direction::Down) if f == D.next(Direction::Down) => (F, Transform::Rot0),
            (f, Direction::Up)   if f == F.next(Direction::Up)   => (D, Transform::Rot0),

            (f, Direction::Right) if f == D.next(Direction::Right) => (C, Transform::Rot90),
            (f, Direction::Down)  if f == C.next(Direction::Down)  => (D, Transform::RotNeg90),

            (f, Direction::Right) if f == C.next(Direction::Right) => (F, Transform::Rot180),
            (f, Direction::Right) if f == F.next(Direction::Right) => (C, Transform::Rot180),

            (f, Direction::Right) if f == B.next(Direction::Right) => (F, Transform::Rot90),
            (f, Direction::Down)  if f == F.next(Direction::Down)  => (B, Transform::RotNeg90),

            _ => return (coo, dir) // We are on a valid face, not lost in the emptiness of the manifold
        };

        // Compute the offset inside the current (and invalid) face
        let cube_width = self.area.width() as isize / 3;
        let off_x = (coo.x % cube_width) + if coo.x < 0 { cube_width } else { 0 };
        let off_y = coo.y % cube_width + if coo.y < 0 { cube_width } else { 0 };

        // Compute the new offset in the valid landing face
        let (tr_off_x, tr_off_y) = match tr {
            Transform::Rot0     => (off_x, off_y),
            Transform::Rot90    => (off_y, cube_width-1-off_x),
            Transform::Rot180   => (cube_width-1-off_x, cube_width-1-off_y),
            Transform::RotNeg90 => (cube_width-1-off_y, off_x),
        };

        // Compute the new direction in the valid landing face
        let new_dir = match tr {
            Transform::Rot0     => dir,
            Transform::Rot90    => dir.to_left(),
            Transform::Rot180   => dir.flip(),
            Transform::RotNeg90 => dir.to_right(),
        };

        // New final coordinate
        let new_coo = (
            new_face.x * cube_width + tr_off_x,
            new_face.y * cube_width + tr_off_y
        ).into();

        (new_coo, new_dir)
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
        let x = coo.x / cube_width + if coo.x < 0 { -1 } else { 0 };
        let y = coo.y / cube_width + if coo.y < 0 { -1 } else { 0 };
        (x, y).into()
    }


    /// Compute the password from the current position
    fn password (&self) -> usize {
        let num_dir = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        (1 + self.coo.y as usize) * 1000 + (1 + self.coo.x as usize) * 4 + num_dir
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

    board.apply_instructions(&instructions);
    Ok(board.password())
}

pub fn day_22 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 6032);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}