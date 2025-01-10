use std::cmp::PartialEq;
use anyhow::*;
use itertools::Itertools;
use crate::{Cell, GridCell, Solution};

const TEST: &str = "\
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// The four possible displacements
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North, South, East, West,
}

/// The content of the [PlayGround] at some coordinate
#[derive(Copy, Clone, Debug, PartialEq)]
enum FieldCell {
    Empty,
    Elf,
}

/// A coordinate on the [PlayGround]
type Coo = (isize, isize);

/// Encodes a vote at some location
#[derive(Copy, Clone, Debug, Default)]
struct Vote {

    /// Coordinate the elf at this location would like to go to
    proposition: Option<Coo>,

    /// Number of propositions for this location
    target_count: usize,
}

/// Models the playground with the elves
struct PlayGround {

    /// Field content
    field: GridCell<FieldCell>,

    /// All the votes (one be location)
    votes: Vec<Vote>,

    /// Current direction for the vote rule
    current_dir: Direction,
}

impl Direction {

    /// Get the direction to consider next
    fn next(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }

    /// Coordinate increment by stepping in the given direction
    fn step(&self) -> Coo {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

impl Default for FieldCell {
    fn default() -> Self {
        FieldCell::Empty
    }
}

impl Cell for FieldCell {
    fn from_character (c: char) -> Option<FieldCell> {
        match c {
            '.' => Some(FieldCell::Empty),
            '#' => Some(FieldCell::Elf),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            FieldCell::Empty => '.',
            FieldCell::Elf => '#',
        }
    }
}


impl PlayGround {

    fn new (content: &[&str]) -> Result<PlayGround> {
        let field = GridCell::new(content)?.inflated(100);
        let votes = vec![Vote::default(); field.width() * field.height()];

        Ok(PlayGround { field, votes, current_dir: Direction::North })
    }

    /// Apply the results of all the votes by updating the elf locations accordingly.
    /// The function returns `(size, updated)`, where `size` is the empty area covered by
    /// the elves (part a) and `updated` is true if some elf has moved (part b)
    fn resolve_votes (&mut self) -> (usize, bool) {

        let mut new_field = GridCell::<FieldCell>::new_empty(
            self.field.width(),
            self.field.height()
        );

        // For tracking the covered area
        let mut top = self.field.height();
        let mut bottom = 0;
        let mut left = self.field.width();
        let mut right = 0;
        let mut elf_count = 0;
        let mut updated = false;

        // Check all the cells for elves
        for (x, y, cell) in self.field.iter_cells() {
            let x = x as isize;
            let y = y as isize;

            if *cell == FieldCell::Elf {
                elf_count += 1;

                // Next Elf location, based on the result of its vote
                let new_pos = if let Some (target_coo) = self.vote_result((x, y)) {
                    updated = true;
                    target_coo
                } else {
                    (x, y)
                };
                *new_field.sample_mut((new_pos.0 as usize, new_pos.1 as usize)) = FieldCell::Elf;

                // Track the field size
                top = top.min(new_pos.1 as usize);
                bottom = bottom.max(new_pos.1 as usize);
                left = left.min(new_pos.0 as usize);
                right = right.max(new_pos.0 as usize);
            }
        }

        // Update the new elves disposition and update the rule for the next turn
        self.field = new_field;
        self.current_dir = self.current_dir.next();

        // Return the size of the empty area covered by the elves
        let size = (bottom-top+1)*(right-left+1) - elf_count;
        (size, updated)
    }

    /// Return the result of a vote made by the elf at location `coo`.
    /// The function returns the new elf location, if the vote is successful, or `None` otherwise
    fn vote_result (&self, coo: Coo) -> Option<Coo> {

        // Get the target proposition
        let vote = self.sample_vote(coo);
        if let Some (coo_target) = vote.proposition {

            // Check there is not collision there
            let vote_target = self.sample_vote(coo_target);
            if vote_target.target_count == 1 {
                Some(coo_target)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the vote at location `coo`
    fn sample_vote (&self, coo: Coo) -> &Vote {
        let idx = coo.1 as usize * self.field.width() + coo.0 as usize;
        &self.votes [idx]
    }

    /// Do the vote mechanism for all the elves on the playground
    fn make_votes (&mut self) {
        let mut votes = vec![Vote::default(); self.field.width() * self.field.height()];

        for (x, y, cell) in self.field.iter_cells() {
            let x = x as isize;
            let y = y as isize;

            // An Elf with a neighbor must make a proposition
            if *cell == FieldCell::Elf && self.has_neighbors(x, y, None) {
                self.make_vote(&mut votes, x, y);
            }
        }

        self.votes = votes;
    }

    /// Create a vote for the elf at location `x, y`, by updating the collection of `votes`
    /// accordingly
    fn make_vote (&self, votes: &mut Vec<Vote>, x: isize, y: isize) {

        // Test the 4 directions in sequence
        let mut dir = self.current_dir;
        for _ in 0..4 {

            // Stop when the tested direction is empty and make a vote
            if !self.has_neighbors(x, y, Some(dir)) {

                // - the proposed coordinate
                let step = dir.step();
                let proposition = (x + step.0, y + step.1);

                // - vote
                let idx = y as usize * self.field.width() + x as usize;
                votes [idx].proposition = Some (proposition);

                let idx = proposition.1 as usize * self.field.width() + proposition.0 as usize;
                votes [idx].target_count += 1;

                break;
            }
            dir = dir.next();
        }
    }

    /// Check if some elf at location `x, y` has some neighbor in the provided `direction`.
    /// If no direction is given, the 8 tiles around are checked.
    fn has_neighbors (&self, x: isize, y: isize, direction: Option<Direction>) -> bool {

        let range_x = match direction {
            None | Some(Direction::North) | Some(Direction::South) => x-1..=x+1,
            Some(Direction::West) => x-1..=x-1,
            Some(Direction::East) => x+1..=x+1,
        };

        let range_y = match direction {
            None | Some(Direction::West) | Some(Direction::East) => y-1..=y+1,
            Some(Direction::North) => y-1..=y-1,
            Some(Direction::South) => y+1..=y+1,
        };

        range_x.cartesian_product(range_y).any (|(xi, yi)| {
            if xi == x && yi == y { return false}
            self.field.try_sample((xi, yi)).copied() == Some (FieldCell::Elf)
        })
    }
}

/// Solve both parts of the puzzle
fn solve (content: &[&str]) -> Result<(usize, usize)> {

    let mut playground = PlayGround::new(content)?;

    let mut round = 0;
    let mut empty_area = 0;

    let round_stop = loop {
        round += 1;
        playground.make_votes();
        let (area, updated) = playground.resolve_votes();

        if round == 10 { empty_area = area; }
        if !updated { break round }
    };

    Ok((empty_area, round_stop))
}

pub fn day_23 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST)).unwrap_or_default() == (110, 20));

    let (ra, rb) = solve(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}