/// The four possible displacements
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up, Down, Left, Right,
}

/// To help iterate on the directions
static DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Down, Direction::Left, Direction::Right];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coo {
    pub x: isize,
    pub y: isize,
}

impl From<Coo> for (isize, isize) {
    fn from(coo: Coo) -> Self { (coo.x, coo.y) }
}

impl From<(usize, usize)> for Coo {
    fn from((x, y): (usize, usize)) -> Self {
        assert!(x <= isize::MAX as usize && y <= isize::MAX as usize);
        Coo { x: x as isize, y: y as isize }
    }
}

impl From<(isize, isize)> for Coo {
    fn from((x, y): (isize, isize)) -> Self {
        Coo { x, y }
    }
}

impl From<Coo> for (usize, usize) {
    fn from(coo: Coo) -> Self {
        assert!(coo.x >= 0 && coo.y >= 0);
        (coo.x as usize, coo.y as usize)
    }
}

impl Coo {

    /// Returns the coordinates resulting from moving one step in the provided `direction`
    pub fn next (&self, direction: Direction) -> Self {
        let step = direction.step();
        Self {
            x: self.x + step.x,
            y: self.y + step.y,
        }
    }

    /// Returns the coordinates resulting from moving one step in the provided `direction`,
    /// except if the coordinates are not in the area [0; width[ x [0; height[.
    pub fn try_next(&self, direction: Direction, width: usize, height: usize) -> Option<Coo> {
        let next_coo = self.next(direction);
        if next_coo.x < 0 ||
            next_coo.y < 0 ||
            next_coo.x >= width as isize ||
            next_coo.y >= height as isize {
            None
        } else {
            Some(next_coo)
        }
    }
}

impl Direction {

    /// Return an iterator on the possible directions
    pub fn iter() -> impl Iterator<Item = Direction> { DIRECTIONS.iter().cloned() }

    /// Coordinate increment by stepping in the given direction
    pub fn step(&self) -> Coo {
        match self {
            Direction::Up => Coo { x: 0, y: -1 },
            Direction::Down => Coo { x: 0, y: 1 },
            Direction::Left => Coo { x: -1, y: 0 },
            Direction::Right => Coo { x: 1, y: 0 },
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

