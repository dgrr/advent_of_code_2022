use std::env;
use std::io::{stdin, BufRead};

fn main() {
    if let Some(arg) = env::args().nth(1) {
        if arg == "-1" {
            first_part();
        } else {
            second_part();
        }
    } else {
        println!("Specify the part of the test, -1 or -2");
        println!("{} (-1 | -2)", env::args().nth(0).unwrap());
    }
}

fn first_part() {
    let mut max_calories = 0;
    let mut current = 0;

    for line in stdin().lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            current = 0;
        } else {
            current += line.parse::<u64>().unwrap();
            max_calories = max_calories.max(current);
        }
    }

    println!("{}", max_calories);
}

fn second_part() {
    let mut results = Vec::new();
    let mut current = 0;

    for line in stdin().lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            results.push(current);
            current = 0;
        } else {
            current += line.parse::<u64>().unwrap();
        }
    }

    results.push(current);

    results.sort_by(|a, b| b.cmp(a));

    let calories = results.iter().take(3).sum::<u64>();
    println!("{}", calories);
}
