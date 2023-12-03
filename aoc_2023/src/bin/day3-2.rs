use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Match {
    value: String,
    start: usize,
    end: usize,
    line: usize,
}

fn main() {
    let input = fs::read_to_string("src/bin/day3.input").expect("Error reading file");
    let numbers = find_all(&input, r"\d+");
    let parts = find_all(&input, r"\*");

    let gears = gear_ratio(numbers, parts);

    let total = gears.iter().sum::<i32>();

    println!("{:?}", total);
}

fn gear_ratio(numbers: Vec<Match>, parts: Vec<Match>) -> Vec<i32> {
    parts
        .iter()
        .map(|n| {
            let gear: Vec<&Match> = numbers.iter().filter(|g| is_adjacent(g, n)).collect();

            if gear.len() == 2 {
                return gear
                    .iter()
                    .map(|g| g.value.parse::<i32>().unwrap())
                    .product();
            }

            0
        })
        .collect::<Vec<i32>>()
}

fn is_adjacent(a: &&Match, b: &Match) -> bool {
    return a.line.saturating_sub(1) <= b.line
        && a.line + 1 >= b.line
        && (a.start <= b.end)
        && (a.end >= b.start);
}

fn find_all(input: &str, pattern: &str) -> Vec<Match> {
    let re = Regex::new(pattern).unwrap();
    let mut matches: Vec<Match> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            matches.push(Match {
                value: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                line: i,
            });
        }
    }

    return matches;
}
