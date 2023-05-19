use std::collections::VecDeque;

use crate::types::{Monkey, Operation, OperationValue, Variable};

const ROUNDS: i64 = 10000;

pub fn part_2(monkeys: &[Monkey]) {
    let mut item_arrs: Vec<VecDeque<i64>> = monkeys
        .iter()
        .map(|monkey| monkey.starting_items.clone().into())
        .collect();

    let mut inspection_counts = vec![0; monkeys.len()];

    let universal_divisor = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test_divisor);

    (0..ROUNDS).for_each(|_round| {
        monkeys.iter().for_each(|monkey| 'a: loop {
            let item = {
                let items = item_arrs.get_mut(monkey.id).unwrap();

                let mut value = match items.pop_front() {
                    Some(item) => item,
                    None => break 'a,
                };

                let right_hand_value = match monkey.operation_value {
                    OperationValue::Variable(Variable::Old) => value,
                    OperationValue::Value(v) => v,
                };

                value = match monkey.operation {
                    Operation::Add => value + right_hand_value,
                    Operation::Multiply => value * right_hand_value,
                };

                value % universal_divisor
            };

            inspection_counts[monkey.id] += 1;

            let target = if item % monkey.test_divisor == 0 {
                monkey.true_target
            } else {
                monkey.false_target
            };

            item_arrs.get_mut(target).unwrap().push_back(item);
        })
    });

    inspection_counts.sort();
    inspection_counts.reverse();

    let result: i64 = inspection_counts.first().unwrap() * inspection_counts.get(1).unwrap();

    println!("Part 2: {result:?}");
}
