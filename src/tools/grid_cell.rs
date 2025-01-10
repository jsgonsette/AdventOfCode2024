use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Display;
use anyhow::{anyhow, bail};
use itertools::Itertools;
use crate::tools::Coo;

/// Models a rectangular area made of generic [Cell]
#[derive(Clone)]
pub struct GridCell<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

/// Models a single location inside a [GridCell]
pub trait Cell: Sized + Default + Clone {

    /// Create a Cell from a text character
    fn from_character (_c: char) -> Option<Self> { None }

    /// Turn the cell into a text character
    fn to_char (&self) -> char { '?' }
}


/// Next element to explore with Dijkstra
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct DijkstraItem {
    coo: Coo,
    score: usize,
}

/// Dijkstra priority queue
type PriorityQueue = BinaryHeap<DijkstraItem>;

/// Ordering for [DijkstraItem] elements in the [PriorityQueue]
impl Ord for DijkstraItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

/// Ordering for [DijkstraItem] elements in the [PriorityQueue]
impl PartialOrd for DijkstraItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// To help debugging
impl<T: Cell> Display for GridCell<T> {
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

impl<T: Cell> GridCell<T> {

    /// Instantiate the area on the basis of the puzzle file content.
    pub fn new(content: &[&str]) -> anyhow::Result<GridCell<T>> {

        let width = content
            .iter()
            .take_while(|row| !row.is_empty())
            .map(|line| line.len()).max().unwrap_or(0);

        if width == 0 { bail!("Cell area is empty"); }

        let cells = Self::load_cell_from_content(content, width)?;
        let height = cells.len () / width;

        anyhow::Ok(GridCell {
            width,
            height,
            cells,
        })
    }

    /// New empty area (cell default) of given dimensions `width` and `height`
    pub fn new_empty (width: usize, height: usize) -> GridCell<T> {
        GridCell {
            width,
            height,
            cells: vec![Default::default(); width * height],
        }
    }

    /// Find the first cell for which the predicate function `f` returns `true`
    pub fn find_cell<F> (&self, f: F) -> Option<Coo>
    where F: Fn (&T) -> bool {
        self.iter_cells().find_map(
            |(x, y, tile)| match f(tile) {
                false => None,
                true  => Some(Coo::from((x, y))),
            }
        )
    }

    /// Return a copy of this instance with additional margin cells along its 4 sides.
    /// Parameter `margin` indicates how many cells to add.
    pub fn inflated (&self, margin: usize) -> GridCell<T> {
        let n_width = self.width + margin * 2;
        let n_height = self.height + margin * 2;
        let n_cells = vec![Default::default(); n_width * n_height];

        let mut new_area = GridCell {
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
    fn load_cell_from_content (content: &[&str], width: usize) -> anyhow::Result<Vec<T>> {

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
    pub fn sample (&self, coo:impl Into<Coo>) -> &T {
        let coo = coo.into();
        &self.cells[coo.y as usize * self.width + coo.x as usize]
    }

    /// Get the mutable cell at some location `coo`
    pub fn sample_mut (&mut self, coo:impl Into<Coo>) -> &mut T {
        let coo = coo.into();
        &mut self.cells[coo.y as usize * self.width + coo.x as usize]
    }

    /// Try getting a reference on the cell at some `coo`
    pub fn try_sample (&self, coo:impl Into<Coo>) -> Option<&T> {
        let coo = coo.into();
        if coo.x < 0 || coo.x >= self.width as isize { return None }
        if coo.y < 0 || coo.y >= self.height as isize { return None }
        Some (self.sample((coo.x as usize, coo.y as usize)))
    }

    /// Try getting a mutable reference on the cell at some `coo`
    pub fn try_sample_mut (&mut self, coo:impl Into<Coo>) -> Option<&mut T> {
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

    /// Return true if the coordinate is inside the area
    pub fn is_inside (&self, coo:impl Into<Coo>) -> bool {
        let coo = coo.into();
        coo.x >= 0 && coo.x < self.width as isize && coo.y >= 0 && coo.y < self.height as isize
    }

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

    /// Return an iterator that yields triplets `(coo, &cell, score)` by increasing score.
    ///
    /// This function implements a Dijkstra algorithm that begins its exploration at coordinate `from`.
    /// Then, it uses the provided `fn_adjacency` function to progress across its unexplored neighborhood.
    /// Each discovered cell get a score resulting from the parent cell + the adjacency weight.
    /// The iteration stops when there is no cell left to explore.
    pub fn iter_dijkstra<F, I> (&self, from: Coo, fn_adjacency: F) -> impl Iterator<Item = (Coo, &T, usize)>
    where
        F: Fn(Coo) -> I,
        I: Iterator<Item = Coo> {

        let mut visited = vec! [false; self.cells.len()];
        let mut pq = PriorityQueue::new ();

        let start = DijkstraItem { coo: from, score: 0 };
        pq.push (start);

        std::iter::from_fn(move || {

            if let Some (item) = pq.pop() {

                let notify = (item.coo, self.sample(item.coo), item.score);
                let adjacency = fn_adjacency (item.coo);

                for next_coo in adjacency {
                    let index = self.index(&next_coo);
                    if visited[index] { continue }
                    visited[index] = true;
                    pq.push(DijkstraItem { coo: next_coo, score: item.score+1 })
                }

                Some (notify)
            }

            else {
                None
            }
        })
    }

    fn index (&self, coo: &Coo) -> usize {
        coo.y as usize * self.width + coo.x as usize
    }

}