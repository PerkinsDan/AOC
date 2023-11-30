use std::fs;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
struct Round {
    player1: Move,
    player2: Move
}

fn parse_move(input: &str) -> Move {
    match input {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
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

fn result(p1: &Move, p2: &Move) -> i32 {
    match (p1, p2) {
        (Move::Rock, Move::Paper) => 0,
        (Move::Rock, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 0,
        (Move::Scissors, Move::Paper) => 6,
        (Move::Paper, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 6,
        _ => 3
    }
}

fn play_value(x: &Move) -> i32 {
    match x {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/day2.txt").expect("Error reading file");
    
    let mut my_score: i32 = 0;
    let mut opp_score: i32 = 0;

    input.lines().for_each(
        |line| {
            let round = parse_round(line);

            opp_score += result(&round.player1, &round.player2) + play_value(&round.player1);
            my_score += result(&round.player2, &round.player1) + play_value(&round.player2);
        }
    );

    println!("My score: {}", my_score);
}