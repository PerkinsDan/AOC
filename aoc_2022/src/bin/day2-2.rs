use std::fs;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
    Lose,
    Tie,
    Win
}

#[derive(Debug)]
struct Round {
    player1: Move,
    player2: Move
}

fn parse_move(input: &str) -> Move {
    match input {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        "X" => Move::Lose,
        "Y" => Move::Tie,
        "Z" => Move::Win,
        _ => panic!("Invalid move")
    }
}

fn parse_round(line: &str) -> Round {
    let mut split = line.split(" ");
    let player1 = split.next().unwrap();
    let player2 = split.next().unwrap();

    Round {
        player1: parse_move(player1),
        player2: parse_move(player2)
    }
}

fn decision(p1: &Move, p2: &Move) -> Move {

    match (p1, p2) {
        (Move::Rock, Move::Lose) => Move::Scissors,
        (Move::Rock, Move::Win) => Move::Paper,
        (Move::Rock, Move::Tie) => Move::Rock,
        (Move::Scissors, Move::Lose) => Move::Paper,
        (Move::Scissors, Move::Win) => Move::Rock,
        (Move::Scissors, Move::Tie) => Move::Scissors,
        (Move::Paper, Move::Lose) => Move::Rock,
        (Move::Paper, Move::Win) => Move::Scissors,
        (Move::Paper, Move::Tie) => Move::Paper,
        _ => panic!("Invalid move")
    }
}

fn play_value(x: &Move) -> i32 {
    match x {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
        Move::Lose => 0,
        Move::Tie => 3,
        Move::Win => 6,
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/day2.txt").expect("Error reading file");
    
    let mut my_score: i32 = 0;

    input.lines().for_each(
        |line| {
            let round = parse_round(line);
            let decision = decision(&round.player1, &round.player2);

            
            my_score += play_value(&round.player2) + play_value(&decision);
        }
    );

    println!("My score: {}", my_score);
}