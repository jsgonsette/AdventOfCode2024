use std::cmp::PartialEq;
use anyhow::*;
use crate::Solution;
use crate::tools::BitSet;

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


/// Models the playground with the elves
struct PlayGround {

    /// Rectangular field with the binary encoded elves (1bit for each location)
    field: Vec<BitSet>,

    /// Store the location proposition from elves trying to move up
    votes_up: Vec<BitSet>,

    /// Store the location proposition from elves trying to move down
    votes_down: Vec<BitSet>,

    /// Store the location proposition from elves trying to move left
    votes_left: Vec<BitSet>,

    /// Store the location proposition from elves trying to move right
    votes_right: Vec<BitSet>,

    /// Directions to test for the vote rule
    test_directions: [Direction; 4],
}

impl PlayGround {

    /// New playground instance based on the puzzle file `content`
    fn new (content: &[&str]) -> Result<PlayGround> {

        // Puzzle width and height
        let width = content[0].len();
        let height = content.len();

        // Extend the size to enough room for the elves to evolve
        let margin_h = 4*width / 5;
        let extended_width = width + margin_h * 2;

        let top = (0..4*height/5).map(|_| BitSet::zeros(extended_width));
        let bottom = (0..4*height/5).map(|_| BitSet::zeros(extended_width));

        // Extract the elves locations
        let field = content.iter().map (|row| {
            let mut bit_row = BitSet::zeros(extended_width);
            for (idx, b) in row.as_bytes().iter ().rev ().enumerate() {
                bit_row.set_bit(idx + margin_h, *b == b'#')
            }
            bit_row
        });

        let extended_field: Vec<BitSet> = top.chain(field.chain(bottom)).collect();

        let votes_up = vec! [BitSet::zeros(extended_width); extended_field.len()];
        let votes_down = vec! [BitSet::zeros(extended_width); extended_field.len()];
        let votes_left = vec! [BitSet::zeros(extended_width); extended_field.len()];
        let votes_right = vec! [BitSet::zeros(extended_width); extended_field.len()];

        Ok (PlayGround {
            field : extended_field,
            votes_up,
            votes_down,
            votes_left,
            votes_right,
            test_directions: [Direction::North, Direction::South, Direction::West, Direction::East],
        })
    }

    /// Total height of the height
    fn height (&self) -> usize {
        self.field.len()
    }

    /// Total width of the height
    fn width (&self) -> usize {
        self.field [0].width()
    }

    fn _print (&self) {
        for row in self.field.iter() {
            println!();
            for idx in (0..row.width()).rev () {
                print!("{}", if row [idx] { '#' } else { '.' });
            }
        }
    }

    /// Play one round of voting and moving
    fn round (&mut self) -> bool {

        self.make_votes();
        self.resolve_votes();
        let moved = self.apply_votes();
        self.rotate_vote_directions();

        moved
    }

    /// Compute the number of elves
    fn num_elves (&self) -> usize {
        self.field.iter().map (|row| row.count_ones() as usize).sum()
    }

    /// Compute the area of the elves bounding box
    fn compute_elves_area (&self) -> usize {

        let start = self.field.iter().position(|row| !row.all_zeros()).unwrap();
        let last = self.height () - self.field.iter().rev ().position(|row| !row.all_zeros()).unwrap();
        let w = self.width();

        let (left, right, h) = self.field [start..last].iter()
            .fold((w, w, 0), |(left, right, h), row| {

                (left.min (row.leading_zeros() as usize),
                 right.min (row.trailing_zeros() as usize),
                 h+1)
            });

        h * (w - left - right)
    }

    /// Apply the results of the voting scheme, by moving the elves that are able to do it.
    /// Return true if at least one elf could move
    fn apply_votes (&mut self) -> bool {

        let mut moving = false;
        for y in 1..self.height()-1 {

            // All the elves that cannot move on this row, because there is no successful vote decision for them
            let elves_not_moving = &self.field [y] & !(
                &self.votes_up [y-1] | &self.votes_down [y+1] | (&self.votes_left [y] >> 1) | (&self.votes_right [y] << 1)
            );

            // All the elves that are landing on this row because of successful vote decisions
            let elves_moving = &self.votes_up[y] | &self.votes_down[y] | &self.votes_left[y] | &self.votes_right[y];
            moving |= !elves_moving.all_zeros();
            self.field [y] = elves_not_moving | elves_moving;
        }

        moving
    }

    /// Update the voting directions ordering at the end of a round
    fn rotate_vote_directions (&mut self) {
        self.test_directions.rotate_left(1);
    }

    /// Resolve votes by cancelling those that bump into each others.
    /// Because of the voting scheme, only up - down and left - right pairs can be in conflict.
    fn resolve_votes (&mut self) {

        // Scan each field row and cancel votes when 2 elves reach the same location, one from
        // the top, one from the bottom. Same for left and right directions.
        for y in 0..self.height() {
            let not_going_down = !(&self.votes_down[y]);
            let not_going_up = !(&self.votes_up[y]);
            let not_going_left = !(&self.votes_left[y]);
            let not_going_right = !(&self.votes_right[y]);

            self.votes_up[y] &= &not_going_down;
            self.votes_down[y] &= &not_going_up;
            self.votes_right[y] &= &not_going_left;
            self.votes_left[y] &= &not_going_right;
        }
    }

    /// Perform the voting scheme for each elf
    fn make_votes(&mut self) {

        // Work with triplet of rows to get all the information we need around the middle row.
        let mut a;
        let mut b = &self.field [0];
        let mut c = &self.field [1];

        // Scan the field
        for y in 2..self.height() {

            // Previous (up), current and next (down) rows
            a = b;
            b = c;
            c = &self.field[y];

            // The 'up' bits capture the occupancy of NW, N and NE positions above each bit in 'b'
            // Similarly, we have the 'down', 'left' and 'right' to capture the occupancy below
            // and on the sides
            let mut up = (a << 1) | a | (a >> 1);
            let mut down = (c << 1) | c | (c >> 1);
            let vertical = a | b | c;
            let mut left = &vertical >> 1;
            let mut right = &vertical << 1;

            // Elves on row 'b' that are voting (some spots around them must be occupied)
            let mut moving_elves = b & (&up | &down | &left | &right);

            // Move propositions for going up, down, left or right (for elves on row 'b').
            // They are tested in order and each time we remove voters from remaining 'moving_elves'
            for direction in self.test_directions.iter() {
                match direction {
                    Direction::North => {
                        up = &moving_elves & !&up;
                        moving_elves ^= &up;
                    }
                    Direction::South => {
                        down = &moving_elves & !&down;
                        moving_elves ^= &down;
                    }
                    Direction::East => {
                        right = &moving_elves & !&right;
                        moving_elves ^= &right;
                    }
                    Direction::West => {
                        left = &moving_elves & !&left;
                        moving_elves ^= &left;
                    }
                }
            }

            // New location propositions, split in 4 arrays depending on the chosen direction.
            self.votes_up [y-2] = up;
            self.votes_down [y] = down;
            self.votes_left [y-1] = left << 1;
            self.votes_right [y-1] = right >> 1;
        }
    }
}

/// Solve both parts of the puzzle
fn solve (content: &[&str]) -> Result<(usize, usize)> {

    let mut playground = PlayGround::new(content)?;

    let mut round = 0;
    let mut empty_area = 0;

    let round_stop = loop {
        round += 1;
        let moved = playground.round();

        if round == 10 {
            empty_area = playground.compute_elves_area() - playground.num_elves();
        }
        if !moved { break round }
    };

    Ok((empty_area, round_stop))
}

pub fn day_23 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST)).unwrap_or_default() == (110, 20));

    let (ra, rb) = solve(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}