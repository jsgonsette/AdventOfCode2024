mod coordinates;

use std::fmt::Display;
use anyhow::*;
use itertools::Itertools;

pub use coordinates::{Direction, Coo, Coo_};

/// Reads rows made of numbers
pub struct RowReader {
    built_number: Option<usize>,
}

impl RowReader {

    pub fn new() -> RowReader {
        RowReader { built_number: None }
    }

    pub fn _iter_numbers_fix<'a, const N: usize> (
        &'a mut self,
        content: &'a [&'a str],
    ) -> impl Iterator<Item=Result<[usize; N]>> + 'a {

        // Deliver a fixed size vector for each row
        content.iter().map(|row| {
            self.process_row_fix(row).ok_or(anyhow!("Invalid row: {row}"))
        })
    }

    /// Iterate on all the numbers contained in a row, ignoring non-digit characters.
    pub fn iter_row<'a> (&'a mut self, row:&'a str) -> impl Iterator<Item=usize> + 'a {

        let row_it = row.as_bytes().iter().chain(std::iter::once(&0));
        row_it.flat_map(|&b| {
            self.process_byte(b)
        })
    }

    /// Convert a row into a variable size vector of numbers.
    /// All non-digit characters are ignored.
    pub fn process_row (&mut self, row: &str) -> Vec<usize> {
        self.iter_row(row).collect()
    }

    /// Convert a row into a fixed-size vector of numbers. All non-digit characters are ignored.
    /// The function fails if the exact number of numbers is not found.
    pub fn process_row_fix<const N: usize> (&mut self, row: &str) -> Option<[usize; N]> {
        let mut v = [0; N];
        let mut idx = 0;

        let row_it = row.as_bytes().iter().chain(std::iter::once(&0));
        for &b in row_it {

            // Process the next byte and eventually collect a number
            match self.process_byte(b) {
                Some (number) => {
                    if idx < N {
                        v[idx] = number;
                        idx += 1;
                    }
                    else { return None; }
                },
                None => {},
            }
        }

        match idx {
            _ if idx == N => Some (v),
            _ => None
        }
    }

    fn process_byte (&mut self, byte: u8) -> Option<usize> {
        match byte as char  {
            '0' ..= '9' => {
                let current = self.built_number.unwrap_or_default();
                self.built_number = Some(current*10 + (byte - '0' as u8) as usize);
                None
            },
            _ => {
                let out = self.built_number;
                self.built_number = None;
                out
            },
        }
    }
}

/// Models a rectangular area made of generic [Cell]
#[derive(Clone)]
pub struct CellArea<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

/// Models a single location inside a [CellArea]
pub trait Cell: Sized + Default + Clone {

    /// Create a Cell from a text character
    fn from_character (c: char) -> Option<Self>;

    /// Turn the cell into a text character
    fn to_char (&self) -> char;
}


/// To help debugging
impl<T: Cell> Display for CellArea<T> {
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

impl<T: Cell> CellArea<T> {

    /// Instantiate the area on the basis of the puzzle file content.
    pub fn new(content: &[&str]) -> anyhow::Result<CellArea<T>> {

        let width = content.iter().map(|line| line.len()).max().unwrap_or(0);
        let cells = Self::load_cell_from_content(content, width)?;
        let height = cells.len () / width;

        Ok(CellArea {
            width,
            height,
            cells,
        })
    }

    pub fn new_empty (width: usize, height: usize) -> CellArea<T> {
        CellArea {
            width,
            height,
            cells: vec![Default::default(); width * height],
        }
    }

    /// Return a copy of this instance with additional margin cells along its 4 sides.
    /// Parameter `margin` indicates how many cells to add.
    pub fn inflated (&self, margin: usize) -> CellArea<T> {
        let n_width = self.width + margin * 2;
        let n_height = self.height + margin * 2;
        let n_cells = vec![Default::default(); n_width * n_height];

        let mut new_area = CellArea {
            width: n_width,
            height: n_height,
            cells: n_cells,
        };

        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            *new_area.sample_mut((x+margin, y+margin)) = self.sample((x, y)).clone();
        }

        new_area
    }

    /// Iterates on the cells. Yield tuples of `(x, y, &cell)` items
    pub fn iter_cells (&self) -> impl Iterator<Item=(usize, usize, &T)> {
        self.cells.iter().enumerate().map(
            |(i, cell)| (i % self.width, i / self.width, cell)
        )
    }

    /// Iterates on the cells coordinates. Yield tuples of `(x, y)` items
    pub fn iter_xy (&self) -> impl Iterator<Item=(usize, usize)> {
        (0..self.width).cartesian_product(0..self.height)
    }

    /// Create the vector of cells used to encode the maze from the puzzle file `content`
    fn load_cell_from_content (content: &[&str], width: usize) -> Result<Vec<T>> {

        // Make a single vector of cells to encode the maze
        let cells: Option<Vec<T>> = content.iter()
            .take_while(|row| !row.is_empty())
            .flat_map (|row| {

                // If the row length is unequal, expand it with white spaces
                let expand_len = width - row.len();
                let row_it = row.as_bytes().iter();
                let expand_it = std::iter::repeat(&(' ' as u8)).take(expand_len);

                row_it.chain (expand_it).map(|&b| { Cell::from_character(b as char) })
            }).collect();

        cells.ok_or(anyhow!("Invalid content"))
    }

    /// Get the cell at some location `coo`
    pub fn sample (&self, coo: (usize, usize)) -> &T {
        &self.cells[coo.1 * self.width + coo.0]
    }

    //pub fn sample_mut (&mut self, coo: (usize, usize)) -> &mut T {
    pub fn sample_mut (&mut self, coo:impl Into<Coo_>) -> &mut T {
        let coo = coo.into();
        &mut self.cells[coo.y as usize * self.width + coo.x as usize]
    }

    /// Try getting a reference on the cell at some `coo`
    pub fn try_sample (&self, coo:impl Into<Coo_>) -> Option<&T> {
        let coo = coo.into();
        if coo.x < 0 || coo.x >= self.width as isize { return None }
        if coo.y < 0 || coo.y >= self.height as isize { return None }
        Some (self.sample((coo.x as usize, coo.y as usize)))
    }

    pub fn try_sample_mut (&mut self, coo:impl Into<Coo_>) -> Option<&mut T> {
        let coo = coo.into();
        if coo.x < 0 || coo.x >= self.width as isize { return None }
        if coo.y < 0 || coo.y >= self.height as isize { return None }
        Some (self.sample_mut((coo.x as usize, coo.y as usize)))
    }

    /// Return the area width
    pub fn width (&self) -> usize { self.width }

    /// Return the area height
    pub fn height (&self) -> usize { self.height }

    /// Return the area
    pub fn area (&self) -> usize { self.width * self.height }

    pub fn wrap_coo (&self, coo: (isize, isize)) -> (isize, isize) {

        let w = self.width as isize;
        let h = self.height as isize;

        let x = match coo.0 {
            v if v < 0  => w + v,
            v if v >= w => v - w,
            v           => v,
        };

        let y = match coo.1 {
            v if v < 0  => h + v,
            v if v >= h => v - h,
            v           => v,
        };

        (x, y)
    }
}