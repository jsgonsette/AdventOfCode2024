use std::collections::HashSet;
use anyhow::*;
use crate::{Cell, GridCell, Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const TEST_2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// The different possible tiles in the [Warehouse] or [WarehouseWide]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum WarehouseTile {
    Empty, Box, Robot, Wall, BoxLeft, BoxRight
}

/// Models the warehouse in part 1
struct Warehouse {
    area: GridCell<WarehouseTile>,
    robot: Coo,
}

/// Models the wide warehouse in part 2
struct WarehouseWide {
    area: GridCell<WarehouseTile>,
    robot: Coo,
}

impl Default for WarehouseTile {
    fn default() -> Self {
        WarehouseTile::Empty
    }
}

impl Cell for WarehouseTile {
    fn from_character (c: char) -> Option<WarehouseTile> {
        match c {
            '.' => Some(WarehouseTile::Empty),
            'O' => Some(WarehouseTile::Box),
            '@' => Some(WarehouseTile::Robot),
            '#' => Some(WarehouseTile::Wall),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            WarehouseTile::Empty => '.',
            WarehouseTile::Box   => 'O',
            WarehouseTile::Robot => '@',
            WarehouseTile::Wall  => '#',
            WarehouseTile::BoxLeft  => '[',
            WarehouseTile::BoxRight  => ']',
        }
    }
}

impl Warehouse {

    /// New warehouse instance based on the puzzle file content
    fn new(content: &[&str]) -> Result<Warehouse> {

        // Built the area from the puzzle file content
        let area = GridCell::new(content)?;

        // Find the robot location
        let robot: Coo = area.iter_cells().find_map(| (x, y, tile) | {
            match tile {
                WarehouseTile::Robot => Some((x, y).into ()),
                _                    => None,
            }
        }).ok_or(anyhow!("Robot not found"))?;

        Ok(Warehouse {
            area,
            robot
        })
    }

    /// Compute an instance of *WIDE* warehouse from this warehouse
    fn twice_wide (self) -> WarehouseWide {

        let mut area = GridCell::new_empty(self.area.width()*2, self.area.height());
        for (x, y, &cell) in self.area.iter_cells() {

            // Each tile is doubled horizontally
            let (left, right) = match cell {
                WarehouseTile::Empty => (WarehouseTile::Empty,   WarehouseTile::Empty),
                WarehouseTile::Box   => (WarehouseTile::BoxLeft, WarehouseTile::BoxRight),
                WarehouseTile::Robot => (WarehouseTile::Robot,   WarehouseTile::Empty),
                WarehouseTile::Wall  => (WarehouseTile::Wall,    WarehouseTile::Wall),
                _ => unreachable!()
            };

            *area.sample_mut((x*2,   y)) = left;
            *area.sample_mut((x*2+1, y)) = right;
        }

        WarehouseWide {
            area,
            robot: (self.robot.x*2, self.robot.y).into(),
        }
    }

    /// Move the robot one step in the provided `direction`, moving all the boxes
    /// with him when possible
    fn move_robot (&mut self, direction: Direction) {

        // Look for an empty tile
        if let Some (empty_coo) = self.find_empty_tile(direction) {

            // The adjacent tile becomes the robot, the robot becomes an empty tile,
            let adjacent_coo = self.robot.next(direction);
            *self.area.sample_mut(adjacent_coo) = WarehouseTile::Robot;
            *self.area.sample_mut(self.robot) = WarehouseTile::Empty;

            // and the empty tile becomes a box
            if empty_coo != adjacent_coo {
                *self.area.sample_mut(empty_coo) = WarehouseTile::Box;
            }

            self.robot = adjacent_coo;
        }
    }

    /// Compute the sum of all the boxes locations, according to the GPS system
    fn location_sum (&self) -> usize {

        self.area
            .iter_cells()
            .filter(|(_x, _y, &cell) | cell == WarehouseTile::Box)
            .map(|(x, y, _)| y*100+x)
            .sum::<usize>()
    }

    /// Return the coordinate of the first empty tile in the provided `direction`,
    /// starting from the robot location.
    fn find_empty_tile (&self, direction: Direction) -> Option<Coo> {

        let mut scan_coo = self.robot;

        loop {
            let tile = self.area.sample(scan_coo);
            match tile {
                WarehouseTile::Empty => break Some (scan_coo),
                WarehouseTile::Wall => break None,
                _ => {
                    scan_coo = scan_coo.next(direction);
                },
            }
        }
    }
}

impl WarehouseWide {

    /// Move the robot one step in the provided `direction`, moving all the boxes
    /// with him when possible
    fn move_robot (&mut self, direction: Direction) {

        match direction {
            Direction::Left  => self.move_robot_x(-1),
            Direction::Right => self.move_robot_x(1),
            Direction::Up    => self.move_robot_y(-1),
            Direction::Down  => self.move_robot_y(1),
        }
    }

    /// Move the robot up or down by one step.
    /// Parameter `y_step` stands for up (-1) or down (+1)
    fn move_robot_y (&mut self, y_step: isize) {

        // Collect all the boxes that move vertically with the robot
        if let Some (boxes) = self.collect_boxes_y(y_step) {

            // Move them
            self.move_boxes(y_step, boxes);

            // Update the tile with the robot
            *self.area.sample_mut(self.robot) = WarehouseTile::Empty;
            self.robot.y += y_step;
            *self.area.sample_mut(self.robot) = WarehouseTile::Robot;
        }
    }

    /// Move a collection of boxes vertically by one step.
    /// Parameter `y_step` stands for up (-1) or down (+1).
    /// The `boxes` are designated by a collection of coordinates: So, basically, each
    /// box appears two time in the set (left and right part)
    fn move_boxes (&mut self, y_step: isize, mut boxes: HashSet<Coo>) {

        let move_dir = match y_step {
            1  => Direction::Down,
            -1 => Direction::Up,
            _  => unreachable!()
        };

        // Sort the box coordinates from the farthest to the closest to the robot.
        let mut boxes: Vec<Coo> = boxes.drain().collect();
        boxes.sort_by_key(|coo| coo.y * y_step * -1);

        // Update the tiles to reflect the move
        for coo in boxes {
            *self.area.sample_mut(coo.next(move_dir)) = *self.area.sample(coo);
            *self.area.sample_mut(coo) = WarehouseTile::Empty;
        }
    }

    /// Collect all the boxes that would move vertically with the robot.
    /// Parameter `y_step` stands for up (-1) or down (+1).
    /// The function returns a set of coordinates, 2 by box.
    fn collect_boxes_y (&self, y_step: isize) -> Option<HashSet<Coo>> {

        let mut boxes = HashSet::new();

        let move_dir = match y_step {
            1  => Direction::Down,
            -1 => Direction::Up,
            _  => unreachable!()
        };

        // Process queue, we start with the tile above or below the robot
        let mut queue : Vec<Coo> = Vec::new();
        queue.push((self.robot.x, self.robot.y + y_step).into());

        while let Some (coo) = queue.pop() {

            // Look at the tile to process next
            let tile = self.area.sample(coo);
            match tile {

                WarehouseTile::Empty => {}

                // If a wall is encountered, we cannot move anything
                WarehouseTile::Wall => { return None }

                // If we have a box, we collect it and process the two tiles above or below it
                WarehouseTile::BoxLeft => {
                    boxes.insert(coo);
                    boxes.insert(coo.next(Direction::Right));

                    queue.push(coo.next (move_dir));
                    queue.push(coo.next (move_dir).next(Direction::Right));
                }
                WarehouseTile::BoxRight => {
                    boxes.insert(coo);
                    boxes.insert(coo.next(Direction::Left));

                    queue.push(coo.next (move_dir));
                    queue.push(coo.next (move_dir).next(Direction::Left));
                }

                _ => unreachable!()
            }
        }

        Some (boxes)
    }

    /// Move the robot left or right by one step.
    /// Parameter `x_step` stands for right (+1) or left (-1)
    fn move_robot_x (&mut self, x_step: isize) {

        // If there is some empty location on the left or the right
        if let Some (distance) = self.find_empty_tile_x(x_step) {

            let y = self.robot.y;
            let x = self.robot.x;

            // Starting from the robot, until the empty space, shift the tiles one by one.
            let init = WarehouseTile::Empty;
            (0 ..= distance as isize).fold(init, |incoming_tile, dist| {
                let tile = self.area.sample_mut((x + dist * x_step, y));
                let to_move = *tile;
                *tile = incoming_tile;
                to_move
            });

            // Update the robot coordinate
            self.robot.x += x_step;
        }
    }

    /// Return the distance of the first empty tile at the left or right of the robot location.
    /// Parameter `x_step` stands for right (+1) or left (-1)
    fn find_empty_tile_x (&self, x_step: isize) -> Option<usize> {

        let mut scan_coo = self.robot;
        let mut distance = 0;

        loop {
            let tile = self.area.sample(scan_coo);
            match tile {
                WarehouseTile::Empty => break Some (distance),
                WarehouseTile::Wall => break None,
                _ => {
                    scan_coo = (scan_coo.x + x_step, scan_coo.y).into ();
                    distance += 1;
                },
            }
        }
    }

    /// Compute the sum of all the boxes locations, according to the GPS system
    fn location_sum (&self) -> usize {

        self.area
            .iter_cells()
            .filter(|(_x, _y, &cell) | cell == WarehouseTile::BoxLeft)
            .map(|(x, y, _)| y*100+x)
            .sum::<usize>()
    }
}


/// Load the vector of instructions from the file `content`.
fn load_instructions (content: &[&str]) -> Result<Vec<Direction>> {

    let instructions: Option<Vec<Direction>> = content.iter().flat_map(|row| {
       row.as_bytes().iter().map(|&b| {
           match b as char {
               '<' => Some(Direction::Left),
               '>' => Some(Direction::Right),
               '^' => Some(Direction::Up),
               'v' => Some(Direction::Down),
               _   => None,
           }
       })
    }).collect();

    instructions.ok_or(anyhow!("Could not parse instructions"))
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    // Load the warehouse and the instructions from the file content
    let mut warehouse = Warehouse::new(content)?;
    let first_instruction_line = warehouse.area.height()+1;
    let instructions = load_instructions(&content [first_instruction_line..])?;

    // Execute the instructions
    for ins in instructions.iter() {
        warehouse.move_robot(*ins);
    }

    Ok(warehouse.location_sum())
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Load the wide warehouse
    let mut warehouse = Warehouse::new(content)?.twice_wide();

    // Load the instructions
    let first_instruction_line = warehouse.area.height()+1;
    let instructions = load_instructions(&content [first_instruction_line..])?;

    // and execute them
    for ins in instructions.iter() {
        warehouse.move_robot(*ins);
    }

    Ok(warehouse.location_sum())
}

pub fn day_15 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 2028);
    debug_assert!(part_a (&split(TEST_2)).unwrap_or_default() == 10092);
    debug_assert!(part_b (&split(TEST_2)).unwrap_or_default() == 9021);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}