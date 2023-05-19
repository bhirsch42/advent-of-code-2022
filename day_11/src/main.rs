use std::fs::read_to_string;

use regex::{Captures, Regex};

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation character"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Variable {
    Old,
}

#[derive(Copy, Clone, Debug)]
enum OperationValue {
    Variable(Variable),
    Value(i32),
}

#[derive(Debug)]
struct Monkey {
    pub id: usize,
    pub items: Vec<i32>,
    pub operation: Operation,
    pub operation_value: OperationValue,
    pub test_divisor: i32,
    pub true_target: usize,
    pub false_target: usize,
}

impl From<Captures<'_>> for Monkey {
    fn from(capture: Captures) -> Self {
        let id: usize = capture.get(1).unwrap().as_str().parse().unwrap();

        let items: Vec<i32> = capture
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let operation = capture.get(5).unwrap().as_str();
        let operation: Operation = operation.try_into().unwrap();

        let operation_value = capture.get(6).unwrap().as_str();
        let operation_value = match operation_value {
            "old" => OperationValue::Variable(Variable::Old),
            value => OperationValue::Value(value.parse().unwrap()),
        };

        let test_divisor: i32 = capture.get(7).unwrap().as_str().parse().unwrap();
        let true_target: usize = capture.get(8).unwrap().as_str().parse().unwrap();
        let false_target: usize = capture.get(9).unwrap().as_str().parse().unwrap();

        Monkey {
            id,
            items,
            operation,
            operation_value,
            test_divisor,
            true_target,
            false_target,
        }
    }
}

fn load_monkeys() {
    let file = read_to_string("input.txt").unwrap();
    let regex_pattern = read_to_string("regex.txt").unwrap();

    let re = Regex::new(&regex_pattern).unwrap();

    let result: Vec<Monkey> = re
        .captures_iter(&file)
        .map(|capture| {
            println!("{capture:#?}");
            let monkey: Monkey = capture.try_into().unwrap();
            monkey
        })
        .collect();

    println!("{result:#?}");
}

fn part_1(monkeys: &[Monkey]) {
    println!("Part 1:");
}

fn main() {
    let monkeys = load_monkeys();
    // part_1(&monkeys);
}
