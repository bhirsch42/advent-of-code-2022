use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Index, IndexMut},
};

use anyhow::Result;

use crate::utils::{
    grid::{Grid, Position},
    read_input_lines,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Air,
    Rock,
    Sand,
}

const SOURCE: (usize, usize) = (0, 500);

#[derive(Default)]
struct SandWorld {
    pub tiles: HashMap<Position, Tile>,
    pub has_floor: bool,
    pub lowest_rock_row: usize,
}

impl SandWorld {
    fn min_col(&self) -> usize {
        let position = self.tiles.keys().min_by(|x, y| x.col.cmp(&y.col)).unwrap();
        position.col
    }

    fn max_col(&self) -> usize {
        let position = self.tiles.keys().max_by(|x, y| x.col.cmp(&y.col)).unwrap();
        position.col
    }

    fn min_row(&self) -> usize {
        let position = self.tiles.keys().min_by(|x, y| x.row.cmp(&y.row)).unwrap();
        position.row
    }

    fn max_row(&self) -> usize {
        let position = self.tiles.keys().max_by(|x, y| x.row.cmp(&y.row)).unwrap();
        position.row
    }

    fn width(&self) -> usize {
        self.max_col() - self.min_col()
    }

    fn height(&self) -> usize {
        self.max_row() - self.min_row()
    }
}

impl From<&SandWorld> for Grid<Tile> {
    fn from(value: &SandWorld) -> Self {
        let mut grid = Grid::create_and_fill(value.height() + 1, value.width() + 1, Tile::Air);
        let min_row = value.min_row();
        let min_col = value.min_col();

        value.tiles.iter().for_each(|(position, tile)| {
            grid[position.row - min_row][position.col - min_col] = *tile
        });

        grid
    }
}

fn load_input(file_name: &str) -> SandWorld {
    let lines = read_input_lines(file_name);
    let paths: Vec<Vec<(usize, usize)>> = lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|token| {
                    let num_tokens: Vec<&str> = token.split(',').collect();
                    let col: usize = num_tokens.first().unwrap().parse().unwrap();
                    let row: usize = num_tokens.last().unwrap().parse().unwrap();
                    (row, col)
                })
                .collect()
        })
        .collect();

    let rows = paths.iter().flatten().map(|coord| coord.0);
    let max_row = rows.max().unwrap();

    let mut world = SandWorld {
        tiles: HashMap::new(),
        lowest_rock_row: max_row,
        has_floor: false,
    };

    paths.iter().for_each(|path| {
        path.windows(2).for_each(|window| {
            let (start, end) = (window.first().unwrap(), window.last().unwrap());

            let mut row_range = [start.0, end.0];
            row_range.sort();
            let row_range = row_range[0]..(row_range[1] + 1);

            let mut col_range = [start.1, end.1];
            col_range.sort();
            let col_range = col_range[0]..(col_range[1] + 1);

            for row in row_range {
                for col in col_range.clone() {
                    let position = Position { row, col };
                    world.tiles.insert(position, Tile::Rock);
                }
            }
        });
    });

    world
}

impl Debug for Grid<Tile> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .rows()
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match tile {
                            Tile::Air => '.',
                            Tile::Rock => 'X',
                            Tile::Sand => 'o',
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

const POSSIBLE_MOVES: [(i32, i32); 3] = [(1, 0), (1, -1), (1, 1)];

impl SandWorld {
    fn drop_sand(&mut self) -> bool {
        let tiles = &mut self.tiles;
        let mut current_position: Position = Position::from(SOURCE);

        'a: while current_position.row < self.lowest_rock_row + 1 {
            for possible_move in POSSIBLE_MOVES {
                let possible_position = Position::from((
                    ((current_position.row as i32) + possible_move.0) as usize,
                    ((current_position.col as i32) + possible_move.1) as usize,
                ));

                if tiles.get(&possible_position).is_none() {
                    current_position = possible_position;
                    continue 'a;
                }
            }

            tiles.insert(current_position, Tile::Sand);
            return true;
        }

        if self.has_floor {
            tiles.insert(current_position, Tile::Sand);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn example() -> Result<()> {
        let mut sand_grid = load_input("day_14_example");

        let mut count: i32 = 0;

        while sand_grid.drop_sand() {
            count += 1;
        }

        assert_eq!(count, 24);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        let mut sand_world = load_input("day_14");

        let mut count: i32 = 0;

        while sand_world.drop_sand() {
            count += 1;
        }

        assert_eq!(count, 825);
        Ok(())
    }

    #[test]
    fn example_part_2() -> Result<()> {
        let mut sand_world = load_input("day_14_example");
        sand_world.has_floor = true;

        let mut count: i32 = 0;

        while sand_world.tiles.get(&Position::from(SOURCE)) != Some(&Tile::Sand) {
            sand_world.drop_sand();
            count += 1;
        }

        assert_eq!(count, 93);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let mut sand_world = load_input("day_14");
        sand_world.has_floor = true;

        let mut count: i32 = 0;

        while sand_world.tiles.get(&Position::from(SOURCE)) != Some(&Tile::Sand) {
            sand_world.drop_sand();
            count += 1;

            // if count % 100 == 0 {
            //     let grid: Grid<Tile> = (&sand_world).try_into().unwrap();
            //     println!("{:?}", grid);
            //     println!();
            // }
        }

        assert_eq!(count, 26729);
        Ok(())
    }
}
