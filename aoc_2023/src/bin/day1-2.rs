use std::fs;

fn parse_line(line: &str) -> String {
    line.to_string()
    .replace("one", "one1one")
    .replace("two", "two2two")
    .replace("three", "three3three")
    .replace("four", "four4four")
    .replace("five", "five5five")
    .replace("six", "six6six")
    .replace("seven", "seven7seven")
    .replace("eight", "eight8eight")
    .replace("nine", "nine9nine")
}

fn find_first(line: &str) -> char {
    if let Some(c) = line.chars().find(|&c| c.is_digit(10)) {
        return c;
    }

    '0'
}

fn find_last(line: &str) -> char {
    if let Some(c) = line.chars().rev().find(|&c| c.is_digit(10)) {
        return c;
    }

    '0'
}

fn main() {
    let input = fs::read_to_string("src/bin/day1.input").expect("Error reading file");
    let lines = input.lines();

    let mut total: i32 = 0;

    lines.for_each(
        |line| {
            let line = parse_line(line);
            let first = find_first(&line).to_string();
            let last = find_last(&line).to_string();

            let combination = first.clone() + &last;

            total += combination.parse::<i32>().unwrap();
        }
    );

    println!("Total: {}", total);
}
