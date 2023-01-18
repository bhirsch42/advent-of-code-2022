use itertools::Itertools;
use std::{error::Error, fs::read_to_string};

#[derive(Debug)]
enum GameMove {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

type Round = (GameMove, GameMove);

const INPUT_FILEPATH: &str = "input.txt";

impl TryInto<GameMove> for &str {
    type Error = ();

    fn try_into(self) -> Result<GameMove, Self::Error> {
        match self {
            "A" => Ok(GameMove::Rock),
            "B" => Ok(GameMove::Paper),
            "C" => Ok(GameMove::Scissors),
            "X" => Ok(GameMove::Rock),
            "Y" => Ok(GameMove::Paper),
            "Z" => Ok(GameMove::Scissors),
            _ => Err(()),
        }
    }
}

fn parse_round(s: &str) -> Round {
    s.split(" ")
        .take(2)
        .map(|x| (*x).try_into().expect("Parse error"))
        .collect_tuple()
        .expect("Parse error")
}

fn load_input() -> Vec<Round> {
    let input = read_to_string(INPUT_FILEPATH).expect("Error reading file");
    let lines = input.split('\n');
    lines.map(parse_round).collect()
}

fn round_to_outcome(round: &Round) -> Outcome {
    match round {
        (GameMove::Rock, GameMove::Rock) => Outcome::Draw,
        (GameMove::Rock, GameMove::Paper) => Outcome::Win,
        (GameMove::Rock, GameMove::Scissors) => Outcome::Lose,
        (GameMove::Paper, GameMove::Rock) => Outcome::Lose,
        (GameMove::Paper, GameMove::Paper) => Outcome::Draw,
        (GameMove::Paper, GameMove::Scissors) => Outcome::Win,
        (GameMove::Scissors, GameMove::Rock) => Outcome::Win,
        (GameMove::Scissors, GameMove::Paper) => Outcome::Lose,
        (GameMove::Scissors, GameMove::Scissors) => Outcome::Draw,
    }
}

fn outcome_to_score(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3,
    }
}

fn game_move_to_score(game_move: &GameMove) -> i32 {
    match game_move {
        GameMove::Rock => 1,
        GameMove::Paper => 2,
        GameMove::Scissors => 3,
    }
}

fn round_to_score(round: &Round) -> i32 {
    game_move_to_score(&round.1) + outcome_to_score(&round_to_outcome(round))
}

fn main() {
    let total: i32 = load_input().iter().map(round_to_score).sum();
    println!("{:?}", total);
}
