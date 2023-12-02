use std::fs;

struct Rucksack {
    left: String,
    right: String
}

fn parse_line(line: &str) -> Rucksack {
    let length = line.len();
    let first_half = line[0..length/2].to_string();
    let second_half = line[length/2..length].to_string();
    
    Rucksack {
        left: first_half,
        right: second_half
    }
}

fn compare(left: &String, right: &String) -> char {
    let mut dup = ' ';
    for c in left.chars() {
        if right.contains(c) {
            dup = c
        }
    }

    dup
}

fn get_value(c: char) -> i32 {
    let value = c as i32;

    if value >= 97 {
        return value - 96;
    } else if value >= 65 {
        return value - 38;
    }

    return value;
    
}


fn main() {
    let input = fs::read_to_string("src/bin/day3.txt").expect("Error reading file");
    let rucksack = input.split("\n");
    let mut total = 0;

    rucksack.for_each(
        |line| {
            let rucksack = parse_line(line);
            
            let shared = compare(&rucksack.left, &rucksack.right);

            total += get_value(shared);
        }
    );

    println!("{total}");
}