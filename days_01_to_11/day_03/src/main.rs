use std::{collections::HashSet, fs::read_to_string};

fn find_char_that_appears_in_both_halves(s: &&str) -> char {
    let half = s.len() / 2;
    let (left, right) = (&s[0..half], &s[half..]);
    let left_hash_set: HashSet<char> = HashSet::from_iter(left.chars());
    let right_hash_set: HashSet<char> = HashSet::from_iter(right.chars());
    let mut intersection = left_hash_set.intersection(&right_hash_set);
    let shared_item = intersection.next().expect("No shared items.");
    (*shared_item).to_owned()
}

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn char_to_score(c: char) -> i32 {
    let score = CHARS.find(|x| x == c).expect("Invalid scoring character");
    (score as i32) + 1
}

fn part_1() {
    let file_string = read_to_string("input.txt").expect("Error reading file");
    let lines: Vec<&str> = file_string.split_whitespace().collect();
    let total: i32 = lines
        .iter()
        .map(find_char_that_appears_in_both_halves)
        .map(char_to_score)
        .sum();
    println!("{:?}", total);
}

fn find_common_char(arr: &[&str]) -> char {
    let mut arr_iter = arr.iter().peekable();

    let mut common_chars: HashSet<char> = HashSet::from_iter(arr_iter.peek().unwrap().chars());

    arr_iter.for_each(|s| {
        let hash_set: HashSet<char> = HashSet::from_iter(s.chars());
        let intersection = common_chars.intersection(&hash_set);
        let intersection_chars = intersection.map(|x| *x);
        common_chars = HashSet::from_iter(intersection_chars);
    });

    *common_chars.iter().next().expect("No shared chars")
}

fn part_2() {
    let file_string = read_to_string("input.txt").expect("Error reading file");
    let lines: Vec<&str> = file_string.split_whitespace().collect();
    let total: i32 = lines
        .chunks(3)
        .map(find_common_char)
        .map(char_to_score)
        .sum();

    println!("{:?}", total);
}

fn main() {
    part_1();
    println!("---");
    part_2();
}
