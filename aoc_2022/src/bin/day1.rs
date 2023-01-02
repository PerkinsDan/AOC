fn main() {
    let contents = include_str!("day1.txt").split("\n\n");
    
    let reindeers: Vec<&str> = contents.collect();
    
    let first_ans = max_value(&reindeers);
    let second_ans = top_three(&reindeers);

    println!("{first_ans}");
    println!("{second_ans}");
}

fn max_value(reindeers: &Vec<&str>) -> i32 {
    let mut max_value = 0;

    for reindeer in reindeers {
        let calories = reindeer.split("\n");

        let mut sum = 0;

        for value in calories {
            if !value.is_empty() {
               sum += value.parse::<i32>().unwrap();
            }
        }

        if sum > max_value {
            max_value = sum;
        }
    }
    
    max_value
}

fn top_three(reindeers: &Vec<&str>) -> i32 {
    let mut sum_calories: Vec<i32> = Vec::new();
    
    for reindeer in reindeers {
        let calories = reindeer.split("\n");
       
        let mut sum = 0;

        for value in calories {
            if !value.is_empty() {
                sum += value.parse::<i32>().unwrap();
            }
        }

        sum_calories.push(sum);
    }

    sum_calories.sort();

    let mut total = 0;

    for _ in 0..3 {    
        total += sum_calories.pop().unwrap();
    }

    total
}
 
