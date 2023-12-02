use std::fs;

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
            let first = find_first(line).to_string();
            let last = find_last(line).to_string();

            let combination = first + &last;

            total += combination.parse::<i32>().unwrap();
        }
    );

    println!("Total: {}", total);
}
