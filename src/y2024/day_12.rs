use std::collections::HashSet;
use anyhow::*;
use crate::{Cell, CellArea, Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const TEST_2: &str = "\
AAAA
BBCD
BBCC
EEEC";

const TEST_3: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const TEST_4: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Models a single [Garden] location.
#[derive(Debug, Copy, Clone, Default)]
struct GardenTile {
    plant: char,
    visited: bool,
}

impl Cell for GardenTile {
    fn from_character(c: char) -> Option<Self> {
        if c.is_ascii_alphabetic() {
            Some(Self { plant: c.to_ascii_uppercase(), visited: false })
        }
        else {
            None
        }
    }

    fn to_char(&self) -> char {
        self.plant
    }
}

/// The characteristics of some region (same adjacent garden plots)
struct Region {
    area: u32,
    perimeter: u32,
    sides: u32,
}

/// Models the whole garden
struct Garden {

    /// The garden [tiles](GardenTile)
    tiles: CellArea<GardenTile>,
}

impl Garden {

    /// New garden instance, built from the puzzle file content.
    fn new (content: &[&str]) -> Result<Garden> {

        let tiles = CellArea::new(content)?;
        Ok(Self { tiles, })
    }

    /// Compute the *NORMAL* and *DISCOUNTED* fence price
    /// * Normal: (area x perimeter of each region)
    /// * Discounted: (area x #sides of each region)
    fn compute_fence_price (&mut self) -> (u32, u32) {

        let mut tot_price = 0;
        let mut tot_price_discount = 0;

        for (x, y) in self.tiles.iter_xy() {

            // If the location has not been visited yet, compute the corresponding region ...
            if self.tiles.sample((x, y)).visited == false {
                let region = self.calculate_region((x as isize, y as isize));

                // ... and its price
                tot_price += region.perimeter * region.area;
                tot_price_discount += region.sides * region.area;
            }
        }

        (tot_price, tot_price_discount)
    }

    /// Calculate the characteristics of a region, provided a representative location `coo`
    fn calculate_region (&mut self, coo: Coo) -> Region {

        let mut area = 0;

        // Keep track of all the fence positions (loc. and direction pointing to the fence)
        let mut fences = HashSet::<(Coo, Direction)>::new();

        // DFS queue, starting with the initial coordinate
        let mut queue: Vec<Coo> = Vec::with_capacity(self.tiles.area());
        queue.push(coo);

        // Visit the first tile and record the plant type
        let first_tile = self.tiles.sample_mut((coo.0 as usize, coo.1 as usize));
        first_tile.visited = true;
        let plant_type = first_tile.plant;

        // Keep going if we have unvisited tiles
        while let Some (coo) = queue.pop() {
            area += 1;

            // Test the 4 directions for expansion
            for dir in Direction::iter() {

                // Get the adjacent location
                let step = dir.step();
                let next_coo = (coo.0 + step.0, coo.1 + step.1);

                // Get the tile there
                if let Some (next_tile) = self.tiles.try_sample_mut(next_coo) {

                    // Not the same specie ? record the fence
                    if next_tile.plant != plant_type { fences.insert((coo, dir)); }

                    // Not visited ? schedule a visit
                    else if next_tile.visited == false {
                        queue.push(next_coo);
                        next_tile.visited = true;
                    }
                }

                // Out of bound ? record the fence
                else { fences.insert((coo, dir)); }
            }
        }

        Region {
            area,
            perimeter: fences.len() as u32,
            sides: Self::count_fence_sides(&fences),
        }
    }

    /// Count the number of sides given a collection of `fences`.
    ///
    /// *Each straight section of fence counts as a side*
    fn count_fence_sides (fences: &HashSet::<(Coo, Direction)>) -> u32 {

        let mut sides = 0;

        // For each fence
        for (coo, dir) in fences.iter() {
            match dir {

                // On top or on bottom ? count for +1 only if no fence on the left
                Direction::Up | Direction::Down => {
                    let step = Direction::Left.step();
                    let on_left = (coo.0 + step.0, coo.1 + step.1);
                    if !fences.contains(&(on_left, *dir)) { sides += 1 }
                },

                // On left or on right ? count for +1 only if no fence on the top
                Direction::Left | Direction::Right => {
                    let step = Direction::Up.step();
                    let on_top = (coo.0 + step.0, coo.1 + step.1);
                    if !fences.contains(&(on_top, *dir)) { sides += 1 }
                },
            }
        }

        sides
    }

}

/// Solve both parts of the puzzle
fn solve (content: &[&str]) -> Result<(usize, usize)> {

    let mut garden = Garden::new(content)?;
    let (price, discount_price) = garden.compute_fence_price();

    Ok ((price as usize, discount_price as usize))
}


pub fn day_12 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST)).unwrap_or_default() == (1930, 1206));
    debug_assert!(solve (&split(TEST_2)).unwrap_or_default().1 == 80);
    debug_assert!(solve (&split(TEST_3)).unwrap_or_default().1 == 236);
    debug_assert!(solve (&split(TEST_4)).unwrap_or_default().1 == 368);

    let (ra, rb) = solve(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}
