use std::{cmp::Ordering, iter::zip};

use crate::utils::read_input_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedArray(serde_json::Value);

impl PartialOrd for NestedArray {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (left, right) = (&self.0, &other.0);

        if left.is_number() && right.is_number() {
            let (left, right) = (left.as_i64()?, right.as_i64()?);
            return left.partial_cmp(&right);
        }

        let left = if left.is_array() {
            left.as_array()?.clone()
        } else {
            vec![left.clone()]
        };

        let right = if right.is_array() {
            right.as_array()?.clone()
        } else {
            vec![right.clone()]
        };

        for (left, right) in zip(left.clone(), right.clone()) {
            let result = NestedArray(left.clone()).partial_cmp(&NestedArray(right.clone()))?;
            if result != Ordering::Equal {
                return Some(result);
            }
        }

        if left.len() < right.len() {
            return Some(Ordering::Less);
        }

        if left.len() > right.len() {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Equal)
    }
}

impl Ord for NestedArray {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

const DIVIDER_PACKET_1: &str = "[[2]]";
const DIVIDER_PACKET_2: &str = "[[6]]";

pub fn load_input() -> Vec<(NestedArray, NestedArray)> {
    let input_lines = read_input_lines("day_13");

    let input_lines: Vec<serde_json::Value> = input_lines
        .iter()
        .filter_map(|input_line| serde_json::from_str(input_line).ok())
        .collect();

    input_lines
        .chunks(2)
        .map(|chunk| (NestedArray(chunk[0].clone()), NestedArray(chunk[1].clone())))
        .collect()
}

pub fn load_input_part_2() -> Vec<NestedArray> {
    let input_lines = read_input_lines("day_13");

    let mut input_lines: Vec<serde_json::Value> = input_lines
        .iter()
        .filter_map(|input_line| serde_json::from_str(input_line).ok())
        .collect();

    input_lines.push(serde_json::from_str(DIVIDER_PACKET_1).unwrap());
    input_lines.push(serde_json::from_str(DIVIDER_PACKET_2).unwrap());

    input_lines
        .iter()
        .map(|value| NestedArray(value.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use rayon::prelude::*;

    #[test]
    fn part_1() -> Result<()> {
        let input_lines = load_input();

        let result: usize = input_lines
            .par_iter()
            .enumerate()
            .fold(
                || 0,
                |agg, (index, pair)| {
                    if pair.0 < pair.1 {
                        agg + index + 1
                    } else {
                        agg
                    }
                },
            )
            .sum();

        assert_eq!(result, 5882);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let mut lines = load_input_part_2();

        lines.par_sort_unstable();

        let divider_1: serde_json::Value = serde_json::from_str(DIVIDER_PACKET_1).unwrap();
        let divider_2: serde_json::Value = serde_json::from_str(DIVIDER_PACKET_2).unwrap();

        let index_1 = lines.iter().position(|item| item.0 == divider_1).unwrap() + 1;
        let index_2 = lines.iter().position(|item| item.0 == divider_2).unwrap() + 1;

        assert_eq!(index_1 * index_2, 24948);

        Ok(())
    }
}
