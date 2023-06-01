use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Could not parse direction: {s:?}"),
        }
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn default() -> Position {
        Position { x: 0, y: 0 }
    }
}

fn load_motions() -> Vec<Motion> {
    let file = read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.split('\n').collect();

    lines
        .iter()
        .map(|&line| {
            let tokens: Vec<&str> = line.split(' ').collect();

            Motion {
                direction: Direction::from(tokens[0]),
                distance: tokens[1].parse().unwrap(),
            }
        })
        .collect()
}

fn apply_step_to_head(direction: &Direction, head: &mut Position) {
    match direction {
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
        Direction::Left => head.x -= 1,
        Direction::Right => head.x += 1,
    }
}

fn apply_step_to_follower(leader: &Position, follower: &mut Position) {
    let x_dist = leader.x - follower.x;
    let y_dist = leader.y - follower.y;
    let distance = x_dist.abs().max(y_dist.abs());

    if distance <= 1 {
        return;
    }

    if x_dist < 0 {
        follower.x -= 1;
    }

    if x_dist > 0 {
        follower.x += 1;
    }

    if y_dist < 0 {
        follower.y -= 1;
    }

    if y_dist > 0 {
        follower.y += 1;
    }
}

fn part_1(motions: &[Motion]) {
    let mut rope_head = Position::default();
    let mut rope_tail = Position::default();
    let mut visited: HashMap<Position, bool> = HashMap::new();

    motions.iter().for_each(|motion| {
        (0..motion.distance).for_each(|_| {
            apply_step_to_head(&motion.direction, &mut rope_head);
            apply_step_to_follower(&rope_head, &mut rope_tail);
            visited.insert(rope_tail, true);
        });
    });

    println!("Part 1: {:?}", visited.len());
}

fn part_2(motions: &[Motion]) {
    let mut knots: [Position; 10] = [Position::default(); 10];
    let mut visited: HashMap<Position, bool> = HashMap::new();

    motions.iter().for_each(|motion| {
        (0..motion.distance).for_each(|_| {
            apply_step_to_head(&motion.direction, &mut knots[0]);

            (0..(knots.len() - 1)).for_each(|i| {
                let leader = &knots[i].clone();
                let follower = &mut knots[i + 1];
                apply_step_to_follower(leader, follower);
            });

            visited.insert(*knots.last().unwrap(), true);
        });
    });

    println!("Part 2: {:?}", visited.len());
}

fn main() {
    let motions: Vec<Motion> = load_motions();
    part_1(&motions);
    part_2(&motions);
}
