use std::fs::read_to_string;

// O(n * m), could do O(n) with a hashmap
fn find_window_without_dupes(chars: Vec<char>, window_size: usize) -> i32 {
    for (i, cs) in chars.windows(window_size).enumerate() {
        let mut cs_vec = cs.to_vec();
        cs_vec.sort();
        cs_vec.dedup();

        if cs_vec.len() == window_size {
            return (i + window_size) as i32;
        }
    }

    -1
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let all_chars: Vec<char> = input.chars().collect();
    let part_1 = find_window_without_dupes(all_chars.clone(), 4);
    let part_2 = find_window_without_dupes(all_chars, 14);

    println!("Part 1: {part_1:?}");
    println!("Part 2: {part_2:?}");
}
