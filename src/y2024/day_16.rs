use anyhow::*;
use crate::{Cell, CellArea, Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

type Location = (Coo, Direction);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MazeTile {
    Empty,
    Wall,
    Start,
    End,
}

struct Maze {
    tiles: CellArea<MazeTile>,
    loc: Location,
}

impl Default for MazeTile {
    fn default() -> Self {
        MazeTile::Empty
    }
}


impl Cell for MazeTile {
    fn from_character (c: char) -> Option<MazeTile> {
        match c {
            '.' => Some(MazeTile::Empty),
            '#' => Some(MazeTile::Wall),
            'E' => Some(MazeTile::End),
            'S' => Some(MazeTile::Start),
            _ => None,
        }
    }

    fn to_char (&self) -> char {
        match self {
            MazeTile::Empty => '.',
            MazeTile::Wall  => '#',
            MazeTile::Start  => 'S',
            MazeTile::End  => 'E',
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl Maze {
    fn new (content: &[&str]) -> Result<Self> {

        let area = CellArea::new(content)?;
        let (xs, ys, _) = area.iter_cells().find(|(x, y, &cell)| cell == MazeTile::Start).ok_or(anyhow!("No start"))?;
        let (xe, ye, _) = area.iter_cells().find(|(x, y, &cell)| cell == MazeTile::End).ok_or(anyhow!("No end"))?;

        Ok (Maze {
            tiles: area,
            loc: ((xs, ys).into(), Direction::Right),
        })
    }
}

/// Solve first part of the puzzle
fn part_a (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_16 (_content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}