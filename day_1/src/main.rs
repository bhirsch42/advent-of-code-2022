use std::{fs::read_to_string, mem::swap, num::ParseIntError};

const INPUT_FILEPATH: &str = "input.txt";

fn load_input() -> Vec<Result<i32, ParseIntError>> {
    let input = read_to_string(INPUT_FILEPATH).expect("Error reading file");
    let lines = input.split('\n');
    lines.map(|line| line.parse::<i32>()).collect()
}

fn part_1() {
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

    println!("{:?}", max);
}

fn update_max(max: &mut [i32; 3], value: &i32) {
    let mut current = *value;

    max.iter_mut().for_each(|max_value| {
        if *max_value < current {
            swap(max_value, &mut current);
        }
    });
}

fn part_2() {
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

    println!(
        "{} = {}",
        max.map(|num| num.to_string()).join(" + "),
        max.iter().sum::<i32>()
    );
}
fn main() {
    part_1();
    println!("---");
    part_2();
}
