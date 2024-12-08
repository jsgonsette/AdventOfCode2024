use std::collections::HashMap;
use std::fmt::Display;
use anyhow::*;
use itertools;
use itertools::Itertools;
use crate::{Cell, CellArea, Solution};

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

type Frequency = char;

type Coo = (usize, usize);

/// An antenna on the map
#[derive(Default)]
struct CellAntenna {

    /// Antenna frequency
    frequency: Option<Frequency>,

    /// Number of antinodes laying on this cell
    antinode: u32,
}

/// Models the map with its antenna
struct Map {

    /// The area with the antenna frequencies
    area: CellArea<CellAntenna>,

    /// For each antenna frequency, we retain the corresponding antennas on the map
    antennas: HashMap<Frequency, Vec<Coo>>,
}

impl Cell for CellAntenna {

    fn from_character(c: char) -> Option<Self> {
        let frequency = match c {
            '.' => None,
            _   => Some (c)
        };
        Some(Self { frequency, antinode: 0 })
    }

    fn to_char(&self) -> char {
        match (self.frequency, self.antinode) {
            (_, count) if count > 0 => '#',
            (Some(frequency), _) => frequency,
            _ => '.',
        }
    }
}

/// To help debugging
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.area.fmt(f)
    }
}

impl Map {

    /// New map based on the puzzle file content
    fn new(content: &[&str]) -> Result<Self> {

        // Load the area
        let area: CellArea<CellAntenna> = CellArea::new(content)?;

        // Collect the antennas by frequencies.
        // For each frequency, collect the matching coordinates
        let mut antenna: HashMap<Frequency, Vec<Coo>> = HashMap::new();
        for (x, y, cell) in area.iter_cells() {
            if let Some(frequency) = cell.frequency {
                antenna.entry(frequency).or_default().push((x, y));
            }
        };

        Ok(Map {
            area,
            antennas: antenna,
        })
    }

    fn count_antinodes(&mut self, consider_harmonics: bool) -> usize {

        // Count the number of antinodes
        let mut count = 0;

        // For each frequency
        for (&_freq, coos) in self.antennas.iter() {

            // and for each possible pair of corresponding coordinates
            for coo_pair in coos.iter().combinations(2) {

                // Mark the map with an antinode and track their quantity. Return true
                // if the coordinate in on map.
                let mut mark_cell = |x, y| {
                    if let Some (cell) = self.area.try_sample_mut((x, y)) {
                        if cell.antinode == 0 { count += 1 }
                        cell.antinode += 1;
                        true
                    } else {
                        false
                    }
                };

                // Start antinodes on the antenna locations
                let (mut anti_0_x, mut anti_0_y) = (coo_pair[0].0 as isize, coo_pair[0].1 as isize);
                let (mut anti_1_x, mut anti_1_y) = (coo_pair[1].0 as isize, coo_pair[1].1 as isize);

                if consider_harmonics {
                    mark_cell(anti_0_x, anti_0_y);
                    mark_cell(anti_1_x, anti_1_y);
                }

                // Step between the pair of antennas
                let dx = anti_1_x - anti_0_x;
                let dy = anti_1_y - anti_0_y;

                loop {
                    // Move the antinode positions
                    anti_0_x -= dx; anti_0_y -= dy;
                    anti_1_x += dx; anti_1_y += dy;

                    // Add them to the map if possible
                    let in_map_0 = mark_cell(anti_0_x, anti_0_y);
                    let in_map_1 = mark_cell(anti_1_x, anti_1_y);

                    if !consider_harmonics { break }
                    if !in_map_0 && !in_map_1 { break }
                }

            }
        }

        count
    }

}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {
    let mut map = Map::new(content)?;
    let count_antinodes = map.count_antinodes(false);

    Ok (count_antinodes)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {
    let mut map = Map::new(content)?;
    let count_antinodes = map.count_antinodes(true);

    Ok (count_antinodes)
}

pub fn day_8 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 14);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 34);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}