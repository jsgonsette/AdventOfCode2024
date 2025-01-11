use anyhow::*;
use itertools::Itertools;
use crate::{Cell, GridCell, Solution};
use crate::tools::{find_coo_extents, Coo, IntReader};

const TEST: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Rock, Air, Sand
}

struct Cave {

    /// The tiles of this cave
    tiles: GridCell<Tile>,

    /// For each tile, cache the coordinate of the previous location of the sand trajectory
    cache_previous: Vec<Coo>,

    /// Location from where the sand is poured
    source: Coo,

    /// Coordinate where the last bunch of sand stopped
    last_stop: Coo,

    /// Sand counter
    sand_counter: usize,

    /// Abyssal void or infinite ground ?
    infinite_ground: bool,

    mound_left: Mound,

    mound_right: Mound,
}

struct Mound {
    height: u32,
    forming: u32
}

const POUR_COO: Coo = Coo {x: 500, y: 0};

impl Default for Tile {
    fn default() -> Self { Tile::Air }
}

impl Cell for Tile {
    fn to_char (&self) -> char {
        match self {
            Tile::Rock => '#',
            Tile::Air => '.',
            Tile::Sand => 'o',
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl Mound {

    fn new () -> Mound {
        Mound {
            height: 0,
            forming: 0,
        }
    }

    fn add_sand (&mut self) {
        self.forming += 1;
        if self.forming > self.height {
            self.height += 1;
            self.forming = 0;
        }
    }
}

impl Cave {

    fn new (content: &[&str], infinite_ground: bool) -> Result<Self> {

        let (tiles, source) = Self::load_cave(content);
        let cache_previous = vec! [Coo {x: isize::MAX, y: isize::MAX}; tiles.area()];
        let last_stop = source;

        Ok (Cave {
            tiles,
            source,
            cache_previous,
            last_stop,
            sand_counter: 0,
            infinite_ground,
            mound_left: Mound::new(),
            mound_right: Mound::new()
        })
    }

    fn pour_sand (&mut self) {

        let mut coo_from = self.source;
        while let Some(coo_stop) = self.trace_trajectory(coo_from) {

            *self.tiles.sample_mut(coo_stop) = Tile::Sand;
            self.sand_counter += 1;

            if coo_stop == self.source { return }

            let idx = self.index(coo_stop.x, coo_stop.y);
            coo_from = self.cache_previous[idx];
        }
    }

    fn trace_trajectory (&mut self, coo: Coo) -> Option<Coo> {

        let next = [(0, 1), (-1, 1), (1, 1)];
        let mut p = coo;
        loop {

            let mut stopped = true;
            for n in next.iter() {

                let new_p = Coo {x: p.x + n.0, y: p.y + n.1};

                if self.infinite_ground && new_p.y as usize == self.tiles.height()-1 {
                    return Some(p)
                }
                if !self.tiles.is_inside(new_p) { return None }

                if *self.tiles.sample(new_p) == Tile::Air {

                    let idx = self.index(new_p.x, new_p.y);
                    self.cache_previous [idx] = p;

                    p = new_p;
                    stopped = false;
                    break;
                }
            }

            if stopped { return Some (p) }
        }
    }

    fn index (&self, x: isize, y: isize) -> usize {
        self.tiles.width() * y as usize + x as usize
    }

    fn load_cave (content: &[&str]) -> (GridCell::<Tile>, Coo) {

        // Extract the ground coordinates and compute the size of the area
        let lines = Self::load_lines(content);
        let (min, max) = find_coo_extents(lines.iter().flatten().copied());

        let min = Coo { x: min.x.min (POUR_COO.x), y: min.y.min (POUR_COO.y) };
        let max = Coo { x: max.x.max (POUR_COO.x), y: max.y.max (POUR_COO.y) };
        let width = (max.x - min.x) as usize +1;
        let height = (max.y - min.y) as usize +1;

        // Update height to add an infinite ground (question 2)
        let height = height +2;
        let min = Coo { x: min.x - height as isize, y: min.y};
        let width = width + 2*height;

        // Create an empty cave
        let mut grid = GridCell::<Tile>::new_empty(width, height);

        // And put the ground
        for line in lines {
            for (a, b) in line.iter().tuple_windows::<(&Coo, &Coo)>() {
                for x in a.x.min (b.x) ..= a.x.max (b.x) {
                    for y in a.y.min (b.y) ..= a.y.max (b.y) {
                        *grid.sample_mut((x-min.x, y-min.y)) = Tile::Rock;
                    }
                }
            }
        }

        (grid, Coo { x: 500-min.x, y: 0-min.y })
    }

    fn load_lines (content: &[&str]) -> Vec<Vec<Coo>> {

        let mut reader = IntReader::new(false);
        content.iter().map (|&row| {

            let coos: Vec<Coo> = reader.iter_row::<u32> (row).chunks(2).into_iter().map(|mut pair_it| {
                let x = pair_it.next().unwrap() as isize;
                let y = pair_it.next().unwrap() as isize;
                Coo { x, y }
            }).collect();

            coos
        }).collect()
    }
}



/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut cave = Cave::new(content, false)?;
    cave.pour_sand();

    Ok(cave.sand_counter)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let mut cave = Cave::new(content, true)?;
    cave.pour_sand();

    Ok(cave.sand_counter)
}

pub fn day_14 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 24);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() != 0);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}