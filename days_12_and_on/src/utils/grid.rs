use std::{fmt::Debug, vec};

pub struct Grid<T>
where
    T: Clone + PartialEq,
{
    pub elements: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: Clone + PartialEq,
{
    pub fn new(elements: Vec<T>, width: usize) -> Self {
        if elements.len() % width != 0 {
            panic!("Invalid grid dimensions");
        }
        let height = elements.len() / width;

        Self {
            elements,
            width,
            height,
        }
    }

    pub fn rows(&self) -> Vec<Vec<T>> {
        self.elements.chunks(self.width).map(Vec::from).collect()
    }

    pub fn columns(&self) -> Vec<Vec<T>> {
        (0..self.width)
            .map(|row| {
                (0..self.height)
                    .map(|column| self.elements[row + column * self.width].clone())
                    .collect()
            })
            .collect()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.height || col >= self.width {
            return None;
        }

        Some(&self.elements[row * self.width + col])
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

    pub fn neighbors(&self, target: &Position) -> Vec<&T> {
        let mut arr: Vec<&T> = vec![];

        if let Some(item) = self.get(target.row + 1, target.col) {
            arr.push(item);
        }

        if let Some(item) = self.get(target.row, target.col + 1) {
            arr.push(item);
        }

        if let Some(item) = self.get(target.row - 1, target.col) {
            arr.push(item);
        }

        if let Some(item) = self.get(target.row, target.col - 1) {
            arr.push(item);
        }

        arr
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
        writeln!(f, "Height: {:?}", self.height)?;
        write!(f, "{rows}")
    }
}

#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
