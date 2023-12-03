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
    let symbols = find_all(&input, r"[^.\d]");

    let gears: Vec<&Match> = numbers
        .iter()
        .filter(|n| symbols.iter().any(|s| is_adjacent(n, s)))
        .collect();

    let sum = gears
        .iter()
        .map(|n| n.value.parse::<i32>().unwrap())
        .sum::<i32>();

    println!("{:?}", sum);
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
