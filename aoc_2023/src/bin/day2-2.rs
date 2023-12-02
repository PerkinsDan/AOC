use std::fs;

struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn parse_line(line: &str) -> Game {
        let parts: Vec<_> = line.split(":").collect();

        let reveals: Vec<_> = parts[1]
            .split(";")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim().split(", ").collect::<Vec<_>>())
            .collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        reveals.iter().for_each(|r| {
            r.iter().for_each(|s| {
                let parts: Vec<_> = s.split_whitespace().collect();
                let color = parts[1];
                let value = parts[0].parse::<i32>().unwrap();

                match color {
                    "red" => {
                        if red < value {
                            red = value
                        }
                    }
                    "green" => {
                        if green < value {
                            green = value
                        }
                    }
                    "blue" => {
                        if blue < value {
                            blue = value
                        }
                    }
                    _ => (),
                }
            });
        });

        Game { red, green, blue }
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/day2.input").expect("Error reading file");
    let lines = input.lines();

    let total: i32 = lines
        .map(|line| {
            let game = Game::parse_line(line);

            return game.power();
        })
        .sum();

    println!("Total: {}", total);
}
