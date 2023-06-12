use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

const NEIGHBOR_OFFSETS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

pub struct Grid<T>
where
    T: PartialEq,
{
    items: Vec<T>,
    width: usize,
}

#[derive(Debug)]
pub struct GridItem<T>
where
    T: PartialEq,
{
    pub item: T,
    pub position: Position,
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn new(elements: Vec<T>, width: usize) -> Self {
        if elements.len() % width != 0 {
            panic!("Invalid grid dimensions");
        }
        let height = elements.len() / width;

        Self {
            items: elements,
            width,
        }
    }

    pub fn height(&self) -> usize {
        self.items.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn create_and_fill(height: usize, width: usize, fill: T) -> Self
    where
        T: Clone,
    {
        Self::new(vec![fill; width * height], width)
    }

    pub fn rows(&self) -> Vec<&[T]> {
        self.items.chunks(self.width).collect()
    }

    pub fn rows_mut(&mut self) -> Vec<&mut [T]> {
        self.items.chunks_mut(self.width).collect()
    }

    pub fn columns(&self) -> Vec<Vec<&T>> {
        (0..self.width)
            .map(|row| {
                (0..self.height())
                    .map(|column| &self.items[row + column * self.width])
                    .collect()
            })
            .collect()
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        if position.row >= self.height() || position.col >= self.width {
            return None;
        }

        Some(&self.items[position.row * self.width + position.col])
    }

    pub fn find(&self, target: &T) -> Option<Position> {
        for (i, row) in self.rows().into_iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if target == item {
                    return Some(Position { row: i, col: j });
                }
            }
        }

        None
    }

    pub fn find_all(&self, target: &T) -> Vec<Position> {
        let mut results: Vec<Position> = vec![];

        for (i, row) in self.rows().into_iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if target == item {
                    results.push(Position { row: i, col: j });
                }
            }
        }

        results
    }

    pub fn neighbors(&self, target: &Position) -> Vec<GridItem<&T>> {
        NEIGHBOR_OFFSETS
            .iter()
            .filter_map(|offset| {
                let row: Option<usize> = ((target.row as i32) + offset[0]).try_into().ok();
                let col: Option<usize> = ((target.col as i32) + offset[1]).try_into().ok();

                if let (Some(row), Some(col)) = (row, col) {
                    let position = Position { row, col };
                    if let Some(item) = self.get(&position) {
                        return Some(GridItem { item, position });
                    }
                }

                None
            })
            .collect()
    }

    pub fn add_row(&mut self, fill: T)
    where
        T: Copy,
    {
        self.items.append(&mut vec![fill; self.width]);
    }
}

impl Debug for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows: Vec<String> = self
            .rows()
            .iter()
            .map(|row| row.iter().map(|&b| if b { 'X' } else { '•' }).collect())
            .collect();

        let rows = rows.join("\n");
        writeln!(f, "Width: {:?}", self.width)?;
        writeln!(f, "Height: {:?}", self.height())?;
        write!(f, "{rows}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position {
            row: value.0,
            col: value.1,
        }
    }
}

impl<T> Index<usize> for Grid<T>
where
    T: PartialEq,
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.rows()[index]
    }
}

impl<T> IndexMut<usize> for Grid<T>
where
    T: PartialEq,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width;
        let end = start + self.width;
        &mut self.items[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn index() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        let left = &grid[1];
        let right = vec![4, 5, 6];
        assert_eq!(left, right);
    }
}
