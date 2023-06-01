use std::fs::read_to_string;
mod part_1;
mod part_2;
mod types;

use regex::Regex;
use types::Monkey;

fn load_monkeys() -> Vec<Monkey> {
    let file = read_to_string("input.txt").unwrap();
    let regex_pattern = read_to_string("regex.txt").unwrap();

    let re = Regex::new(&regex_pattern).unwrap();

    let monkeys: Vec<Monkey> = re
        .captures_iter(&file)
        .map(|capture| {
            let monkey: Monkey = capture.try_into().unwrap();
            monkey
        })
        .collect();

    monkeys
}
fn main() {
    let monkeys = load_monkeys();

    part_1::part_1(&monkeys);
    part_2::part_2(&monkeys);
}
