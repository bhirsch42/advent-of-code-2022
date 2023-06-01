use core::panic;
use std::fs::read_to_string;

#[derive(Debug)]
enum Operation {
    Add,
    Noop,
}

impl Operation {
    fn cycles(&self) -> i32 {
        match self {
            Operation::Add => 2,
            Operation::Noop => 1,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    pub operation: Operation,
    pub value: Option<i32>,
}

impl From<Vec<&str>> for Instruction {
    fn from(value: Vec<&str>) -> Self {
        match value[..] {
            ["addx", value] => {
                let value: i32 = value.parse().unwrap();

                Instruction {
                    operation: Operation::Add,
                    value: Some(value),
                }
            }
            ["noop"] => Instruction {
                operation: Operation::Noop,
                value: None,
            },
            _ => panic!("Error parsing instruction"),
        }
    }
}

fn load_instructions() -> Vec<Instruction> {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.split('\n');
    let instructions: Vec<Instruction> = lines
        .map(|line| {
            let words: Vec<&str> = line.split(' ').collect();
            let instruction: Instruction = words.into();
            instruction
        })
        .collect();

    instructions
}

fn part_1(instructions: &[Instruction]) {
    let mut cycle = 0;
    let mut register = 1;
    let mut signal_strengths: Vec<i32> = vec![];

    instructions.iter().for_each(|instruction| {
        (0..instruction.operation.cycles()).for_each(|_| {
            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                signal_strengths.push(register * cycle);
            }
        });

        if let Instruction {
            operation: Operation::Add,
            value: Some(value),
        } = instruction
        {
            register += value;
        }
    });

    let result: i32 = signal_strengths.iter().sum();
    println!("Part 1: {result:?}");
}

fn part_2(instructions: &[Instruction]) {
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut pixels = [[false; 40]; 6];

    instructions.iter().for_each(|instruction| {
        (0..instruction.operation.cycles()).for_each(|_| {
            let col = cycle % 40;
            let row = cycle / 40;

            let row_i: usize = row.try_into().unwrap();
            let col_i: usize = col.try_into().unwrap();

            pixels[row_i][col_i] = col - 1 <= register && register <= col + 1;
            cycle += 1;
        });

        if let Instruction {
            operation: Operation::Add,
            value: Some(value),
        } = instruction
        {
            register += value;
        }
    });

    let rows: Vec<String> = pixels
        .iter()
        .map(|row| {
            let chars: Vec<&str> = row
                .iter()
                .map(|col| match col {
                    true => "#",
                    false => " ",
                })
                .collect();

            chars.join(" ")
        })
        .collect();

    let screen = rows.join("\n");
    println!("Part 2:");
    println!("{screen}");
}

fn main() {
    let instructions = load_instructions();
    part_1(&instructions);
    part_2(&instructions);
}
