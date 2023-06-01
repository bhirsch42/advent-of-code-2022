use crate::utils::grid::{Grid, Position};
use anyhow::{anyhow, Result};
use std::fmt::Debug;

use crate::utils::read_input_lines;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Elevation(u16);

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';

impl TryFrom<&char> for Elevation {
    type Error = anyhow::Error;

    fn try_from(value: &char) -> std::result::Result<Self, Self::Error> {
        match value {
            'a' => Ok(Elevation(1)),
            'b' => Ok(Elevation(2)),
            'c' => Ok(Elevation(3)),
            'd' => Ok(Elevation(4)),
            'e' => Ok(Elevation(5)),
            'f' => Ok(Elevation(6)),
            'g' => Ok(Elevation(7)),
            'h' => Ok(Elevation(8)),
            'i' => Ok(Elevation(9)),
            'j' => Ok(Elevation(10)),
            'k' => Ok(Elevation(11)),
            'l' => Ok(Elevation(12)),
            'm' => Ok(Elevation(13)),
            'n' => Ok(Elevation(14)),
            'o' => Ok(Elevation(15)),
            'p' => Ok(Elevation(16)),
            'q' => Ok(Elevation(17)),
            'r' => Ok(Elevation(18)),
            's' => Ok(Elevation(19)),
            't' => Ok(Elevation(20)),
            'u' => Ok(Elevation(21)),
            'v' => Ok(Elevation(22)),
            'w' => Ok(Elevation(23)),
            'x' => Ok(Elevation(24)),
            'y' => Ok(Elevation(25)),
            'z' => Ok(Elevation(26)),
            &START_CHAR => Ok(Elevation(1)),
            &END_CHAR => Ok(Elevation(26)),
            _ => Err(anyhow!("Invalid elevation")),
        }
    }
}

impl Debug for Grid<Elevation> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows: Vec<String> = self
            .rows()
            .iter()
            .map(|row| {
                let nums: Vec<String> = row.iter().map(|&item| format!("{:0>2}", item.0)).collect();
                nums.join(" ")
            })
            .collect();

        let rows = rows.join("\n");
        writeln!(f, "Width: {:?}", self.width)?;
        writeln!(f, "Height: {:?}", self.height)?;
        writeln!(f, "{rows}")
    }
}

#[derive(Debug)]
struct Heightmap {
    elevations: Grid<Elevation>,
    start: Position,
    end: Position,
}

fn load_heightmap() -> Result<Heightmap> {
    let lines = read_input_lines("day_12");
    let width = lines.first().unwrap().len();
    let chars: Vec<char> = lines.concat().chars().collect();

    let elevations: Vec<Elevation> = chars
        .iter()
        .map(|&char| Elevation::try_from(&char))
        .collect::<Result<Vec<Elevation>>>()?;

    let char_grid = Grid::new(chars, width);

    let elevation_grid = Grid::new(elevations, width);

    let start_position = char_grid
        .find(&START_CHAR)
        .ok_or_else(|| anyhow!("Couldn't find start position"))?;

    let end_position = char_grid
        .find(&END_CHAR)
        .ok_or_else(|| anyhow!("Couldn't find end position"))?;

    Ok(Heightmap {
        elevations: elevation_grid,
        start: start_position,
        end: end_position,
    })
}

impl Heightmap {
    fn dist_bfs(&self) -> i32 {
        0
    }
}

pub fn day_12() -> Result<()> {
    let heightmap = load_heightmap()?;
    let dist = heightmap.dist_bfs();
    println!("{dist:?}");
    Ok(())
}
