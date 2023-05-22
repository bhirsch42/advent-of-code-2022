use anyhow::{anyhow, Result};
use std::collections::BinaryHeap;
use std::fs::read_to_string;

#[derive(Debug)]
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

fn get_position_of_char(file: &str, target_char: &char) -> Option<Position> {
    let lines = file.split('\n');

    for (row, line) in lines.enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == *target_char {
                return Some(Position { row, col });
            }
        }
    }

    None
}

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Heightmap {
    elevations: Vec<Vec<Elevation>>,
    start: Position,
    end: Position,
}

fn load_heightmap() -> Result<Heightmap> {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.split('\n');

    let rows: Result<Vec<Vec<Elevation>>> = lines
        .map(|line| {
            line.chars()
                .map(|char| Elevation::try_from(&char))
                .collect()
        })
        .collect();

    let start_position = get_position_of_char(&file, &START_CHAR)
        .ok_or_else(|| anyhow!("Couldn't find start position"))?;

    let end_position = get_position_of_char(&file, &END_CHAR)
        .ok_or_else(|| anyhow!("Couldn't find end position"))?;

    Ok(Heightmap {
        elevations: rows?,
        start: start_position,
        end: end_position,
    })
}

fn main() -> Result<()> {
    let heightmap = load_heightmap()?;
    println!("{heightmap:?}");

    Ok(())
}
