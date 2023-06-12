use crate::utils::grid::{Grid, Position};
use anyhow::{anyhow, Result};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

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
        writeln!(f, "Width: {:?}", self.width())?;
        writeln!(f, "Height: {:?}", self.height())?;
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

#[derive(Debug)]
struct HeightmapPath {
    pub position: Position,
    pub dist: i32,
}

impl Ord for HeightmapPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for HeightmapPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl PartialEq for HeightmapPath {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for HeightmapPath {}

impl Heightmap {
    fn dist_bfs(&self, start: &Position, end: &Position) -> Option<i32> {
        let mut visited: HashMap<Position, bool> = HashMap::new();
        let mut paths: BinaryHeap<HeightmapPath> = BinaryHeap::new();

        paths.push(HeightmapPath {
            position: *start,
            dist: 0,
        });

        while let Some(heightmap_path) = paths.pop() {
            let current_position = heightmap_path.position;

            let current_elevation = self.elevations.get(&current_position).unwrap();

            for neighbor in self.elevations.neighbors(&current_position) {
                let has_visited = visited.get(&neighbor.position).map_or(false, |_| true);
                let is_accessible = current_elevation.0 + 1 >= neighbor.item.0;

                if !has_visited && is_accessible {
                    let dist = heightmap_path.dist + 1;

                    if neighbor.position == *end {
                        return Some(dist);
                    }

                    visited.insert(neighbor.position, true);

                    paths.push(HeightmapPath {
                        position: neighbor.position,
                        dist,
                    });
                }
            }
        }

        None
    }
}

pub fn day_12() -> Result<()> {
    let heightmap = load_heightmap()?;
    let dist = heightmap.dist_bfs(&heightmap.start, &heightmap.end);
    println!("{dist:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Ok;
    use rayon::prelude::*;

    #[test]
    fn part_1() -> Result<()> {
        let heightmap = load_heightmap()?;
        let dist = heightmap.dist_bfs(&heightmap.start, &heightmap.end);
        assert_eq!(dist, Some(420));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let heightmap = load_heightmap()?;
        let possible_starts = heightmap.elevations.find_all(&Elevation(1));
        let dist = possible_starts
            .par_iter()
            .filter_map(|start| heightmap.dist_bfs(start, &heightmap.end))
            .min();
        assert_eq!(dist, Some(414));
        Ok(())
    }
}
