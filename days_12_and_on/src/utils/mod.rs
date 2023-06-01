pub mod grid;

use std::fs::read_to_string;

pub fn read_input(file_name: &str) -> String {
    read_to_string(format!("inputs/{file_name}.txt")).unwrap()
}

pub fn read_input_lines(file_name: &str) -> Vec<String> {
    read_input(file_name)
        .split('\n')
        .map(|s| s.to_owned())
        .collect()
}
