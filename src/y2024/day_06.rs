use std::fmt::Display;
use anyhow::*;
use itertools::Itertools;
use crate::{Cell, CellArea, Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// The state of a lab location
#[derive(Copy, Clone, Debug, PartialEq)]
enum LabCell {
    Empty,
    Obstruction,
    Guard,
}

/// Keep track of cells exploration, by direction
#[derive(Debug, Default, Copy, Clone)]
struct History {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

#[derive(Clone)]
struct Lab {

    /// Visited cells
    visited: Vec<History>,

    /// Guard location
    guard: Option<Coo>,

    /// Guard moving direction
    guard_dir: Direction,

    /// Models the area locations
    area: CellArea<LabCell>,
}

impl Default for LabCell {
    fn default() -> Self { LabCell::Empty }
}

impl Cell for LabCell {
    fn from_character (c: char) -> Option<LabCell> {
        match c {
            '.' => Some(LabCell::Empty),
            '#' => Some(LabCell::Obstruction),
            '^' => Some(LabCell::Guard),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            LabCell::Empty => '.',
            LabCell::Obstruction => '#',
            LabCell::Guard => '^',
        }
    }
}

impl History {
    fn is_visited (&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}

/// To help debugging
impl Display for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for y in 0..self.area.height (){
            let row: String = (0..self.area.width ()).map(|x| {
                let cell = self.area.sample((x, y)).to_char();

                let h = &self.sample_history((x, y));
                let vertical =  h.up || h.down;
                let horizontal = h.left || h.right;
                match (vertical, horizontal) {
                    (true, true) => '+',
                    (false, false) => cell,
                    (true, false) => '|',
                    (false, true) => '-',
                }
            }).join("");

            f.write_str("\n")?;
            f.write_str(&row)?;
        }
        f.write_str("\n")
    }
}

impl Lab {

    /// New playground instance from puzzle file content
    fn new(content: &[&str]) -> Result<Lab> {

        let area = CellArea::<LabCell>::new (content)?;

        let guard_pos = area.iter_cells().find_map(
            |(x, y, &cell)| if cell == LabCell::Guard {
                Some ((x, y))
            } else { None }
        );

        let (guard_x, guard_y) = guard_pos.ok_or(anyhow!("No guard found"))?;
        let guard_dir = Direction::Up;
        let guard_flat_pos = guard_y  * area.width() + guard_x;

        let mut visited: Vec<History> = vec![Default::default(); area.width () * area.height ()];
        visited [guard_flat_pos].up = true;

        Ok(
            Lab {
                area,
                visited,
                guard_dir,
                guard: Some((guard_x, guard_y).into ())
            }
        )
    }

    /// Move the guard one step ahead
    fn move_guard(&mut self) -> bool {

        if let Some (pos) = self.guard {
            *self.area.sample_mut(pos) = LabCell::Empty;

            let next = pos.try_next(self.guard_dir, self.area.width (), self.area.height ());
            if let Some(pos_next) = next {
                if !self.is_empty(pos_next) {
                    self.guard_dir = self.guard_dir.to_right();
                }
                else {
                    self.guard = next;
                }
            }
            else {
                self.guard = None;
            }
        }

        if let Some (pos) = self.guard {
            *self.area.sample_mut(pos) = LabCell::Guard;
            true
        } else {
            false
        }
    }

    /// Put a block at the specified `pos`
    fn put_block(&mut self, pos: Coo) {
        *self.area.sample_mut(pos) = LabCell::Obstruction
    }

    /// Mark the given position `coo` and direction `dir` as visited
    fn mark_visited (&mut self, coo: Coo, dir: Direction) {
        let history = &mut self.visited [coo.y as usize * self.area.width () + coo.x as usize];
        match dir {
            Direction::Left => history.left = true,
            Direction::Right => history.right = true,
            Direction::Up => history.up = true,
            Direction::Down => history.down = true,
        }
    }

    /// Return the number of visited cells
    fn count_visited (&self) -> usize {
        self.visited.iter().filter(|&x| x.is_visited()).count()
    }

    /// Check if the given position `coo` and `direction` has already been visited
    fn is_already_visited(&self, coo: Coo, direction: Direction) -> bool {
        let history = &self.visited [coo.y as usize * self.area.width () + coo.x as usize];
        match direction {
            Direction::Left => history.left,
            Direction::Right => history.right,
            Direction::Up => history.up,
            Direction::Down => history.down,
        }
    }

    /// Return an iterator on all the visited coordinates
    fn get_visited_history (&self) -> impl Iterator<Item =Coo> + '_ {

        (0..self.area.width()).flat_map(move |x| {
            (0..self.area.height()).filter_map(move |y| {
                let index = y * self.area.width() + x;
                match self.visited [index].is_visited() {
                    true => Some ((x, y).into()),
                    false => None,
                }
            })
        })
    }

    fn sample_history (&self, coo: (usize, usize)) -> History {
        self.visited[coo.1 * self.area.width () + coo.0]
    }

    fn is_empty (&self, coo: Coo) -> bool {
        *self.area.sample(coo) == LabCell::Empty
    }

    fn get_guard_position (&self) -> Option<(Coo, Direction)> {
        self.guard.map(|pos| Some ((pos, self.guard_dir))).unwrap_or_default()
    }
}

/// Check if the guard patrol will loop, by detecting when its position matches one of its
/// previous ones.
fn will_loop (lab: &mut Lab) -> bool {

    loop {
        // If the guard move out of the area, we obviously didn't loop
        if !lab.move_guard() { break false }

        // Otherwise, check the new position ...
        if let Some ((pos, dir)) = lab.get_guard_position() {

            // If already visited, we are entering a loop. Otherwise, mark the position.
            if lab.is_already_visited(pos, dir) { break true }
            else { lab.mark_visited(pos, dir) }
        }
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut lab = Lab::new(content)?;
    while lab.move_guard() {
        if let Some ((coo, dir)) = lab.get_guard_position() {
            lab.mark_visited(coo, dir);
        }
    };

    Ok(lab.count_visited())
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let initial_lab = Lab::new(content)?;

    // Initial solving to get the guard history
    let mut lab = initial_lab.clone();
    while lab.move_guard() {
        if let Some ((coo, dir)) = lab.get_guard_position() {
            lab.mark_visited(coo, dir);
        }
    };

    // Test each position on the guard history
    let mut counter = 0;
    for coo in lab.get_visited_history() {
        let mut test_lab = initial_lab.clone();
        test_lab.put_block(coo);

        if will_loop(&mut test_lab) { counter += 1; }
    }

    Ok (counter)
}

pub fn day_6 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 41);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 6);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}