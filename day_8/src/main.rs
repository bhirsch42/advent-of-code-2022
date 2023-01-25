mod grid;
mod part_1;
mod part_2;

use grid::Grid;
use part_1::part_1;
use part_2::part_2;
use std::fs::read_to_string;

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

fn main() {
    let tree_height_grid = load_trees();
    part_1(&tree_height_grid);
    part_2(&tree_height_grid);
}
