use std::{fmt::Debug, fs::read_to_string};

struct Grid<T>
where
    T: Clone,
{
    elements: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn new(elements: Vec<T>, width: usize) -> Self {
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

    fn rows(&self) -> Vec<Vec<T>> {
        self.elements.chunks(self.width).map(Vec::from).collect()
    }

    fn columns(&self) -> Vec<Vec<T>> {
        (0..self.width)
            .map(|row| {
                (0..self.height)
                    .map(|column| self.elements[row + column * self.width].clone())
                    .collect()
            })
            .collect()
    }
}

impl Grid<bool> {
    fn union_row(&mut self, row: usize, arr: &[bool]) {
        if self.width != arr.len() {
            panic!("Invalid array")
        }

        let row_str: String = arr.iter().map(|&b| if b { 'X' } else { '•' }).collect();

        let offset = row * self.width;
        arr.iter().enumerate().for_each(|(column, &b)| {
            let i = offset + column;
            self.elements[i] = self.elements[i] || b;
        });
    }

    fn union_column(&mut self, col: usize, arr: &[bool]) {
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
        writeln!(f, "Width: {:?}", self.width).unwrap();
        writeln!(f, "Height: {:?}", self.height).unwrap();
        write!(f, "{rows}")
    }
}

fn load_trees() -> Grid<i32> {
    let file = read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.split('\n').collect();
    let width = lines.first().unwrap().len();

    let elements = lines
        .iter()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    Grid::new(elements, width)
}

fn tree_heights_to_visibilities(tree_heights: &[i32]) -> Vec<bool> {
    let mut max = -1;

    tree_heights
        .iter()
        .map(|&height| {
            if height > max {
                max = height;
                true
            } else {
                false
            }
        })
        .collect()
}

fn main() {
    let tree_height_grid = load_trees();

    let mut visibility_grid = Grid::new(
        vec![false; tree_height_grid.elements.len()],
        tree_height_grid.width,
    );

    tree_height_grid
        .rows()
        .iter()
        .enumerate()
        .for_each(|(i, tree_height_row)| {
            let visibilities = tree_heights_to_visibilities(tree_height_row);
            visibility_grid.union_row(i, &visibilities);
            let mut tree_height_row = tree_height_row.clone();
            tree_height_row.reverse();
            let mut visibilities = tree_heights_to_visibilities(&tree_height_row);
            visibilities.reverse();
            visibility_grid.union_row(i, &visibilities);
        });

    tree_height_grid
        .columns()
        .iter()
        .enumerate()
        .for_each(|(i, tree_height_row)| {
            let visibilities = tree_heights_to_visibilities(tree_height_row);
            visibility_grid.union_column(i, &visibilities);
            let mut tree_height_row = tree_height_row.clone();
            tree_height_row.reverse();
            let mut visibilities = tree_heights_to_visibilities(&tree_height_row);
            visibilities.reverse();
            visibility_grid.union_column(i, &visibilities);
        });

    let visible_count = visibility_grid
        .elements
        .iter()
        .fold(0, |agg, &b| if b { agg + 1 } else { agg });

    println!("{visibility_grid:?}");
    println!();
    println!("Part 1: {visible_count:?}");
}
