use std::fmt::Debug;

pub struct Grid<T>
where
    T: Clone,
{
    pub elements: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
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

    pub fn items_looking_north(&self, row: usize, col: usize) -> Vec<T> {
        (0..row)
            .map(|i| self.elements[i * self.width + col].clone())
            .rev()
            .collect()
    }

    pub fn items_looking_south(&self, row: usize, col: usize) -> Vec<T> {
        ((row + 1)..self.height)
            .map(|i| self.elements[i * self.width + col].clone())
            .collect()
    }

    pub fn items_looking_east(&self, row: usize, col: usize) -> Vec<T> {
        ((col + 1)..self.width)
            .map(|i| self.elements[row * self.width + i].clone())
            .collect()
    }

    pub fn items_looking_west(&self, row: usize, col: usize) -> Vec<T> {
        (0..col)
            .map(|i| self.elements[row * self.width + i].clone())
            .rev()
            .collect()
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.elements[row * self.width + col].clone()
    }
}

impl Grid<bool> {
    pub fn union_row(&mut self, row: usize, arr: &[bool]) {
        if self.width != arr.len() {
            panic!("Invalid array")
        }

        let _row_str: String = arr.iter().map(|&b| if b { 'X' } else { '•' }).collect();

        let offset = row * self.width;
        arr.iter().enumerate().for_each(|(column, &b)| {
            let i = offset + column;
            self.elements[i] = self.elements[i] || b;
        });
    }

    pub fn union_column(&mut self, col: usize, arr: &[bool]) {
        if self.height != arr.len() {
            panic!("Invalid array")
        }

        arr.iter().enumerate().for_each(|(row, &b)| {
            let i = col + row * self.width;
            self.elements[i] = self.elements[i] || b;
        });
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
