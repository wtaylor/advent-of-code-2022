use std::str::FromStr;
use crate::day02::Move::{Paper, Rock, Scissors};
use crate::day02::RoundResult::{Draw, Loss, Win};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn defeats_move(&self) -> Move {
        match &self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }
    }

    fn is_defeated_by(&self) -> Move {
        match &self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }

    fn beats(&self, other: &Move) -> bool {
        &self.defeats_move() == other
    }

    fn score_value(&self) -> u32 {
        match &self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            &_ => Err(format!("Fail to map value '{}' to a move", s))
        }
    }
}

#[derive(PartialEq)]
enum RoundResult {
    Win,
    Draw,
    Loss
}

impl RoundResult {
    fn score_value(&self) -> u32 {
        match &self {
            Win => 6,
            Draw => 3,
            Loss => 0
        }
    }
}

impl FromStr for RoundResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            &_ => Err(format!("Fail to map value '{}' to a round result", s))
        }
    }
}

fn parse_input(input: &str) -> Vec<(Move, Move, RoundResult)> {
    input.lines()
        .map(|l| l.split_whitespace())
        .map(|mut l|
            (
                l.next().unwrap().parse::<Move>().unwrap(),
                l.next().unwrap()
            )
        )
        .map(|l|
            (
                l.0,
                l.1.parse::<Move>().unwrap(),
                l.1.parse::<RoundResult>().unwrap()
            ))
        .collect::<Vec<(Move, Move, RoundResult)>>()
}

fn play_round(opponent_move: &Move, player_move: &Move) -> RoundResult {
    if opponent_move == player_move { return Draw }
    if player_move.beats(opponent_move) { return Win }

    Loss
}

fn get_players_required_move(opponent_move: &Move, required_result: &RoundResult) -> Move {
    match required_result {
        Draw => opponent_move.clone(),
        Loss => opponent_move.defeats_move(),
        Win => opponent_move.is_defeated_by()
    }
}

pub fn part_1(input: String) {
    let score: u32 = parse_input(&input).iter()
        .map(|r| r.1.score_value() + play_round(&r.0, &r.1).score_value())
        .sum();

    println!("Total score: {}", score)
}

pub fn part_2(input: String) {
    let score: u32 = parse_input(&input).iter()
        .map(|r| get_players_required_move(&r.0, &r.2).score_value() + r.2.score_value())
        .sum();

    println!("Total score: {}", score)
}
