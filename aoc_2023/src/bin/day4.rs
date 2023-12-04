use std::fs;

fn parse_line(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(&[':', '|']).collect();
    let winners: Vec<&str> = parts[1].trim().split_whitespace().collect();
    let numbers: Vec<&str> = parts[2].trim().split_whitespace().collect();

    numbers
        .iter()
        .filter(|num| winners.iter().any(|winner| &winner == num))
        .collect::<Vec<_>>()
        .len() as u32
}

fn main() {
    let input = fs::read_to_string("src/bin/day4.input").expect("Error reading file");

    let parsed: Vec<&str> = input.split("\n").collect();

    let mut instances = vec![1; parsed.len()];
    let mut i = 0;

    let points: i32 = parsed
        .iter()
        .map(|line| {
            let matches = parse_line(line);

            // Part 1
            let mut points = 0;
            if matches > 0 {
                points = 2_i32.pow(matches - 1);
            }

            // Part 2
            for j in 1..matches + 1 {
                let card = i + j as usize;
                instances[card] += instances[i];
            }

            i += 1;

            points
        })
        .sum();

    let total: i32 = instances.iter().sum();

    println!("{} {}", points, total);
}
