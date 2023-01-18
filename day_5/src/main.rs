use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

fn load_lines() -> Vec<String> {
    let file = read_to_string("input.txt").unwrap();
    file.split('\n').map(|x| x.to_string()).collect()
}

fn to_sections(lines: &[String]) -> (&[String], &[String]) {
    let mut sections = lines.split(|line| line.is_empty());
    (sections.next().unwrap(), sections.next().unwrap())
}

fn to_stacks(initial_position_strs: &[String]) -> Vec<Vec<char>> {
    let mut initial_positions_stripped = initial_position_strs
        .iter()
        .map(|initial_position_str| {
            initial_position_str
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|s| s[1])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    initial_positions_stripped.pop();
    initial_positions_stripped.reverse();

    let stacks_count = initial_positions_stripped[0].len();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_count];
    initial_positions_stripped.iter().for_each(|chars| {
        chars.iter().enumerate().for_each(|(i, &c)| {
            if c != ' ' {
                stacks[i].push(c);
            }
        });
    });

    stacks
}

#[derive(Debug)]
struct CharMove {
    from: usize,
    to: usize,
    count: usize,
}

fn to_char_moves(move_strs: &[String]) -> Vec<CharMove> {
    lazy_static! {
        static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9]+").unwrap();
    }

    move_strs
        .iter()
        .map(|x| {
            let nums = NUMBER_REGEX
                .find_iter(x)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            CharMove {
                count: nums[0],
                from: nums[1] - 1,
                to: nums[2] - 1,
            }
        })
        .collect()
}

fn apply_moves(stacks: &mut [Vec<char>], char_moves: &[CharMove]) {
    char_moves.iter().for_each(|char_move| {
        (0..char_move.count).for_each(|_| {
            let c = stacks[char_move.from].pop().unwrap();
            stacks[char_move.to].push(c);
        })
    });
}

fn apply_moves_bulk(stacks: &mut [Vec<char>], char_moves: &[CharMove]) {
    char_moves.iter().for_each(|char_move| {
        let moved_chars: Vec<char> = {
            let from_stack = { &mut stacks[char_move.from] };
            let new_len = from_stack.len() - char_move.count;
            from_stack.drain(new_len..).collect()
        };

        stacks[char_move.to].extend(moved_chars);
    });
}

fn to_char_code(stacks: &[Vec<char>]) -> String {
    let top_chars: Vec<char> = stacks.iter().map(|stack| *stack.last().unwrap()).collect();
    String::from_iter(top_chars)
}

fn part_1(mut stacks: Vec<Vec<char>>, char_moves: &[CharMove]) {
    apply_moves(&mut stacks, char_moves);
    println!("Part 1: {}", to_char_code(&stacks));
}

fn part_2(mut stacks: Vec<Vec<char>>, char_moves: &[CharMove]) {
    apply_moves_bulk(&mut stacks, char_moves);
    println!("Part 2: {}", to_char_code(&stacks));
}

fn main() {
    let lines = load_lines();
    let (initial_position_strs, move_strs) = to_sections(&lines);
    let char_moves = to_char_moves(move_strs);

    let stacks = to_stacks(initial_position_strs);

    part_1(stacks.clone(), &char_moves);
    part_2(stacks, &char_moves);
}
