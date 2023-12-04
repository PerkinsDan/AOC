use std::fs;

fn compare(lines: Vec<&str>) -> char {
    let mut dup = ' ';
    for c in lines[0].chars() {
        if lines[1].contains(c) && lines[2].contains(c) {
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
    let mut i = 1;

    let mut group: Vec<&str> = vec![];

    rucksack.for_each(|line| {
        group.push(line);

        if i % 3 == 0 {
            let shared = compare(group.clone());
            total += get_value(shared);
            group.clear();
        }

        i += 1;
    });

    println!("{total}");
}
