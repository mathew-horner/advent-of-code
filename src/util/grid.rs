use std::hash::Hash;

/// Utility type to handle 2D arrays in AoC.
///
/// Each location in the grid is represented by a [`Cell`].
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: ParseCell> Grid<T> {
    /// Read the input and output a 2D array of `T` with each cell parsed as the type `T`.
    pub fn parse(input: crate::Input) -> Self {
        Self { data: input.read_lines().map(|line| line.chars().map(T::parse_cell).collect()).collect() }
    }
}

impl<T> Grid<T> {
    /// Return all [`Cell`]s that match the given condition.
    pub fn find_all<'a>(&'a self, f: impl Fn(&T) -> bool) -> impl Iterator<Item = Cell<'a, T>> {
        self.data
            .iter()
            .enumerate()
            .flat_map(move |(r, columns)| {
                columns.into_iter().enumerate().map(move |(c, value)| Cell {
                    grid: self,
                    data: CellData { position: GridCoords { row: r, col: c }, value },
                })
            })
            .filter(move |cell| f(cell.data.value))
    }

    /// Return the [`Cell`] at the given coordinates.
    pub fn get<'a>(&'a self, position: GridCoords) -> Option<Cell<'a, T>> {
        if position.row >= self.data.len() || position.col >= self.data[0].len() {
            return None;
        }
        let value = &self.data[position.row][position.col];
        Some(Cell { grid: self, data: CellData { position, value } })
    }

    fn get_with_signed_coords<'a>(&'a self, coords: GridCoords<i32>) -> Option<Cell<'a, T>> {
        if coords.row < 0 || coords.col < 0 {
            return None;
        }
        self.get(GridCoords { row: coords.row as usize, col: coords.col as usize })
    }
}

/// An element within a [`Grid`].
///
/// See [`CellData`] for more details.
#[derive(Clone, Copy)]
pub struct Cell<'a, T> {
    pub grid: &'a Grid<T>,
    pub data: CellData<'a, T>,
}

impl<'a, T> Cell<'a, T> {
    /// Return the 4 cells ⬆️ , ⬇️ , ⬅️ , and ➡️  of this cell.
    ///
    /// Only in-bounds cells are included.
    pub fn adjacent4(&'a self) -> impl Iterator<Item = Cell<'a, T>> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(row, col)| GridCoords::new(row, col))
            .map(|offset| self.data.position.with_offset(offset))
            .filter_map(|coords| self.grid.get_with_signed_coords(coords))
    }
}

/// The `position` and `value` of a [`Cell`].
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CellData<'a, T> {
    pub position: GridCoords,
    pub value: &'a T,
}

impl<T> CellData<'_, T>
where
    T: Clone,
{
    pub fn to_owned(self) -> OwnedCellData<T> {
        OwnedCellData { position: self.position, value: self.value.clone() }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct OwnedCellData<T> {
    pub position: GridCoords,
    pub value: T,
}

impl<T> Hash for CellData<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state)
    }
}

impl<T> Hash for OwnedCellData<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state)
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct GridCoords<T = usize> {
    pub row: T,
    pub col: T,
}

impl<T> GridCoords<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }
}

impl<T> GridCoords<T>
where
    T: TryInto<i32>,
{
    /// Return new `GridCoords` with the `offset` applied.
    ///
    /// This function assumes that the given coords can be converted to `i32`.
    fn with_offset(self, offset: GridCoords<i32>) -> GridCoords<i32> {
        GridCoords {
            row: self.row.try_into().ok().unwrap() + offset.row,
            col: self.col.try_into().ok().unwrap() + offset.col,
        }
    }
}

pub trait ParseCell {
    /// Parse a `char` into a type.
    ///
    /// Used in [`Grid::parse()`]
    fn parse_cell(cell: char) -> Self;
}

// Implementations of ParseCell for common types:

impl ParseCell for u32 {
    fn parse_cell(cell: char) -> Self {
        cell.to_digit(10).unwrap()
    }
}
