use std::io::{stdin, BufRead};

fn main() {
    let mut max_calories = 0;
    let mut current = Vec::new();

    for line in stdin().lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            let calories = current.iter().sum::<u64>();
            current.clear();
            max_calories = max_calories.max(calories);
        } else {
            current.push(line.parse::<u64>().unwrap());
        }
    }

    println!("{}", max_calories);
}
