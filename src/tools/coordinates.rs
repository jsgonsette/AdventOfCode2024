/// The four possible displacements
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up, Down, Left, Right,
}

/// To help iterate on the directions
static DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Down, Direction::Left, Direction::Right];

pub type Coo = (isize, isize);

impl Direction {

    /// Return an iterator on the possible directions
    pub fn iter() -> impl Iterator<Item = Direction> { DIRECTIONS.iter().cloned() }

    /// Coordinate increment by stepping in the given direction
    pub fn step(&self) -> Coo {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    /// Get the direction resulting from turning right
    pub fn to_right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    /// Get the direction resulting from turning left
    pub fn to_left(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

