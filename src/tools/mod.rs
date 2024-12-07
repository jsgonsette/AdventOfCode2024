use anyhow::anyhow;

/// Models a rectangular area made of generic [Cell]
#[derive(Clone)]
pub struct CellArea<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

/// Models a single location inside a [CellArea]
pub trait Cell: Sized + Default {

    /// Create a Cell from a text character
    fn from_character (c: char) -> Option<Self>;

    /// Turn the cell into a text character
    fn to_char (&self) -> char;
}

impl<T: Cell> CellArea<T> {

    pub fn new(content: &[&str]) -> anyhow::Result<CellArea<T>> {

        let height = content.len();
        let width = content[0].len();
        let cells = Self::load_cell_from_content(content)?;

        Ok(CellArea {
            width,
            height,
            cells,
        })
    }

    pub fn iter_cells (&self) -> impl Iterator<Item=(usize, usize, &T)> {
        self.cells.iter().enumerate().map(
            |(i, cell)| (i % self.width, i / self.width, cell)
        )
    }

    /// Create the vector of cells used to encode the maze from the puzzle file `content`
    fn load_cell_from_content (content: &[&str]) -> anyhow::Result<Vec<T>> {

        let width = content[0].len();

        // Make a single vector of cells to encode the maze
        let cells: Option<Vec<T>> = content.iter().flat_map (|row| {
            row.as_bytes().iter().map(|&b| {
                if row.len() == width {
                    Cell::from_character(b as char)
                }
                else { None }
            })
        }).collect();

        cells.ok_or(anyhow!("Invalid content"))
    }

    /// Get the cell at some location `coo`
    pub fn sample (&self, coo: (usize, usize)) -> &T {
        &self.cells[coo.1 * self.width + coo.0]
    }

    pub fn sample_mut (&mut self, coo: (usize, usize)) -> &mut T {
        &mut self.cells[coo.1 * self.width + coo.0]
    }

    pub fn width (&self) -> usize { self.width }

    pub fn height (&self) -> usize { self.height }

}