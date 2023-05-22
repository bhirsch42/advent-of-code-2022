use itertools::Itertools;
use std::{fs::read_to_string, mem::swap, num::ParseIntError};

const INPUT_FILEPATH: &str = "input.txt";

fn load_input() -> Vec<Result<i32, ParseIntError>> {
    let input = read_to_string(INPUT_FILEPATH).expect("Error reading file");
    let lines = input.split('\n');
    lines.map(|line| line.parse::<i32>()).collect()
}

fn part_1() -> i32 {
    let values = load_input();

    let mut sum: i32 = 0;
    let mut max: i32 = 0;

    values.iter().for_each(|value| {
        match value {
            Ok(num) => sum += num,
            Err(_) => {
                sum = 0;
            }
        }

        if sum > max {
            max = sum
        };
    });

    max
}

fn update_max(max: &mut [i32; 3], value: &i32) {
    let mut current = *value;

    max.iter_mut().for_each(|max_value| {
        if *max_value < current {
            swap(max_value, &mut current);
        }
    });
}

fn part_2() -> i32 {
    let values = load_input();

    let mut sum: i32 = 0;
    let mut max: [i32; 3] = [0, 0, 0];

    values.iter().for_each(|value| match value {
        Ok(num) => sum += num,
        Err(_) => {
            update_max(&mut max, &sum);
            sum = 0;
        }
    });

    max.iter().sum::<i32>()
}

fn scratch() {
    let total: i32 = load_input()
        .split(|x| matches!(x, Err(_)))
        .map(|arr| arr.iter().map(|x| x.to_owned().unwrap()).collect())
        .map(|x: Vec<i32>| x.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("Total: {}", total);
}

fn main() {
    part_1();
    println!("---");
    part_2();
    println!("---");
    scratch();
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn day_01_part_1() {
        assert_eq!(part_1(), 72240);
    }

    #[test]
    fn day_01_part_2() {
        assert_eq!(part_2(), 210957);
    }
}
