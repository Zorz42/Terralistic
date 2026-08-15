use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

/// A dense, bounds checked 2D grid of `T`.
///
/// Every access goes through `translate_coords`, so a coordinate outside the grid is an `Err`
/// rather than a panic or the wrong cell. The grid owns its size for that reason: a `Vec` and a
/// size in separate fields can disagree.
///
/// Cells are **column major** - `index = x * height + y`, so stepping in `y` is a step of one.
/// `Chunks` is row major, and the disagreement is deliberate; see it before changing either.
/// The layout is part of the serialized form, so it is also the bytes on disk.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Grid<T> {
    size: (u32, u32),
    cells: Vec<T>,
}

impl<T> Grid<T> {
    /// A grid with no cells. Every coordinate is out of bounds.
    #[must_use]
    pub const fn new_empty() -> Self {
        Self { size: (0, 0), cells: Vec::new() }
    }

    #[must_use]
    pub const fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// Turns a coordinate into an index into `cells`, or an error if it is outside the grid.
    pub fn translate_coords(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || y < 0 || x >= self.size.0 as i32 || y >= self.size.1 as i32 {
            bail!("Coordinates are out of bounds! x: {x}, y: {y}");
        }

        Ok((x * self.size.1 as i32 + y) as usize)
    }

    /// Whether a coordinate is inside the grid, for callers that skip rather than report.
    #[must_use]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.translate_coords(x, y).is_ok()
    }

    pub fn get(&self, x: i32, y: i32) -> Result<&T> {
        let index = self.translate_coords(x, y)?;
        self.cells.get(index).ok_or_else(|| anyhow!("Cell is accessed out of the bounds! ({x}, {y})"))
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Result<&mut T> {
        let index = self.translate_coords(x, y)?;
        self.cells.get_mut(index).ok_or_else(|| anyhow!("Cell is accessed out of the bounds! ({x}, {y})"))
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) -> Result<()> {
        *self.get_mut(x, y)? = value;
        Ok(())
    }

    /// The backing storage, in the layout above, for sweeping the grid without a bounds check
    /// per cell.
    #[must_use]
    pub fn cells(&self) -> &[T] {
        &self.cells
    }
}

impl<T: Clone> Grid<T> {
    /// A grid of `size` with every cell holding `value`. The fill is an argument so that a
    /// caller wanting cells meaning "nothing here yet" says so rather than implying it.
    #[must_use]
    pub fn filled(size: (u32, u32), value: T) -> Self {
        Self {
            size,
            cells: vec![value; (size.0 * size.1) as usize],
        }
    }

    /// Builds a grid from columns: the outer slice is `x`, each inner `Vec` is that
    /// column's cells from `y = 0` downwards. Every column must be the same length.
    pub fn from_columns(columns: &[Vec<T>]) -> Result<Self> {
        let width = columns.len() as u32;
        let Some(first) = columns.first() else {
            bail!("A grid must not be empty");
        };
        let height = first.len() as u32;

        for column in columns {
            if column.len() as u32 != height {
                bail!("All columns must have the same length");
            }
        }

        let mut cells = Vec::with_capacity((width * height) as usize);
        for column in columns {
            cells.extend_from_slice(column);
        }

        Ok(Self { size: (width, height), cells })
    }
}
