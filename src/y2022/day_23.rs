use std::fmt::Display;
use anyhow::*;
use itertools::Itertools;
use crate::Solution;

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

#[derive(Copy, Clone, Debug)]
enum Cell {
    Empty,
    Elf,
}

struct PlayGround {

    /// Playground's cells
    cells: Vec<Cell>,

    /// Dimensions
    width: usize,
    height: usize,
}

/// To help debugging
impl Display for PlayGround {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for y in 0..self.height {
            let row: String = (0..self.width).map(|x| {
                self.sample((x, y)).to_char()
            }).join("");

            f.write_str("\n")?;
            f.write_str(&row)?;
        }
        f.write_str("\n")
    }
}

impl Cell {
    fn from_character (c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Empty),
            '#' => Some(Cell::Elf),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Elf => '#',
        }
    }
}

impl PlayGround {

    /// New playground instance from puzzle file content
    fn new(content: &[&str], margin: usize) -> Result<PlayGround> {

        let height = content.len() + margin*2;
        let width = content[0].len() + margin*2;
        let cells = Self::load_cell_from_content(content, margin)?;

        Ok(PlayGround { cells, width, height, })
    }

    /// Create the vector of cells used to encode the initial playground from the puzzle file `content`
    fn load_cell_from_content (content: &[&str], margin: usize) -> Result<Vec<Cell>> {

        let width = content[0].len();

        let margin_top = vec!["."; width].join("");
        let margin_left = vec!["."; margin].join("");

        let it_top = std::iter::repeat(margin_top.as_str()).take (margin);
        let it_content = content.iter ().copied();
        let it_bottom = std::iter::repeat(margin_top.as_str()).take (margin);
        let it = it_top.chain(it_content).chain(it_bottom);

        // Make a single vector of cells to encode the maze
        let cells: Option<Vec<Cell>> = it.flat_map (|row| {

            let it_left = margin_left.as_bytes().iter();
            let it_row = row.as_bytes().iter();
            let it_right = margin_left.as_bytes().iter();

            it_left.chain(it_row).chain(it_right).map(|&b| {
                if row.len() == width {
                    Cell::from_character(b as char)
                }
                else { None }
            })
        }).collect();

        cells.ok_or(anyhow!("Invalid content"))
    }

    /// Get the cell at some location `coo`
    fn sample (&self, coo: (usize, usize)) -> Cell {
        self.cells[coo.1 * self.width + coo.0]
    }
}


/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let playground = PlayGround::new(content, 10)?;
    println!("{}", playground);
    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_23 (_content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}