use itertools::Itertools;

/// The four possible displacements (up, down, left and right)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up, Down, Left, Right,
}

/// To help iterate on the directions
static DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Down, Direction::Left, Direction::Right];

/// A 2-D coordinate
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coo {
    pub x: isize,
    pub y: isize,
}

/// [Coo] to signed pair
impl From<Coo> for (isize, isize) {
    fn from(coo: Coo) -> Self { (coo.x, coo.y) }
}

/// Unsigned pair to [Coo]
impl From<(usize, usize)> for Coo {
    fn from((x, y): (usize, usize)) -> Self {
        assert!(x <= isize::MAX as usize && y <= isize::MAX as usize);
        Coo { x: x as isize, y: y as isize }
    }
}

/// Signed pair to [Coo]
impl From<(isize, isize)> for Coo {
    fn from((x, y): (isize, isize)) -> Self {
        Coo { x, y }
    }
}

/// [Coo] to unsigned pair
impl From<Coo> for (usize, usize) {
    fn from(coo: Coo) -> Self {
        assert!(coo.x >= 0 && coo.y >= 0);
        (coo.x as usize, coo.y as usize)
    }
}

impl Default for Coo {
    fn default() -> Self { Coo { x: 0, y: 0 } }
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

    /// Returns the coordinates resulting from moving some distance in the provided `direction`
    pub fn step (&self, direction: Direction, distance: isize) -> Self {
        let step = direction.step();
        Self {
            x: self.x + step.x * distance,
            y: self.y + step.y * distance,
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

    /// Return the Manhattan distance with `other` coordinate
    pub fn manhattan_distance (&self, other: &Coo) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }

    /// Iterate on the 8 adjacent coordinates (up, down, left, right + 4 diagonals)
    pub fn iter_adjacent_8 (&self) -> impl Iterator<Item = Coo> + '_ {

        let x_it = self.x - 1..=self.x + 1;
        let y_it = self.y - 1..=self.y + 1;

        x_it.cartesian_product(y_it)
            .filter(|(x, y)| *x != self.x || *y != self.y)
            .map(|(x, y)| Coo { x, y })
    }

    /// Iterate on the 4 adjacent coordinates (up, down, left, right)
    pub fn iter_adjacent_4 (&self) -> impl Iterator<Item = Coo> + '_ {
        Direction::iter().map(|dir| self.next(dir))
    }

    /// Iterate on all the adjacent tiles that are up to a `limit` Manhattan distance.
    /// Calling this function with a limit of 1 is equivalent
    /// to the function [iter_adjacent_4](Self::iter_adjacent_4)
    pub fn iter_adjacent_manhattan (&self, limit: u32) -> impl Iterator<Item = Coo> + '_ {

        let x_it = self.x - limit as isize..=self.x + limit as isize;
        let y_it = self.y - limit as isize..=self.y + limit as isize;

        x_it.cartesian_product(y_it)
            .filter(move |(x, y)| {
                let distance = (x - self.x).abs () + (y - self.y).abs ();
                distance > 0 && distance <= limit as isize
            })
            .map(|(x, y)| Coo { x, y })
    }

    /// Adjusts the coordinates to wrap around a defined 2D area
    pub fn wrap_around_area (&self, width: usize, height: usize) -> Coo {
        let x = self.x.rem_euclid(width as isize);
        let y = self.y.rem_euclid(height as isize);
        Coo { x, y }
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

    /// Get the direction resulting from doing a U-turn
    pub fn flip(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

