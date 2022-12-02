use std::collections::HashMap;
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
    let combs = vec![
        (("A", "X"), 1 + 3), // draw
        (("A", "Y"), 2 + 6), // win
        (("A", "Z"), 3 + 0), // loss
        (("B", "X"), 1 + 0), // loss
        (("B", "Y"), 2 + 3), // draw
        (("B", "Z"), 3 + 6), // win
        (("C", "X"), 1 + 6), // win
        (("C", "Y"), 2 + 0), // loss
        (("C", "Z"), 3 + 3), // draw
    ]
    .iter()
    .map(|x| (x.0, x.1 as u64))
    .collect::<HashMap<(&str, &str), u64>>();

    let score = stdin()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let x = x.split_once(' ').unwrap();
            combs.get(&x).unwrap().clone()
        })
        .sum::<u64>();

    println!("score = {}", score);
}

fn second_part() {
    let combs = vec![
        (("A", "X"), 3 + 0), // loss
        (("A", "Y"), 1 + 3), // draw
        (("A", "Z"), 2 + 6), // win
        (("B", "X"), 1 + 0), // loss
        (("B", "Y"), 2 + 3), // draw
        (("B", "Z"), 3 + 6), // win
        (("C", "X"), 2 + 0), // loss
        (("C", "Y"), 3 + 3), // draw
        (("C", "Z"), 1 + 6), // win
    ]
        .iter()
        .map(|x| (x.0, x.1 as u64))
        .collect::<HashMap<(&str, &str), u64>>();

    let score = stdin()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let x = x.split_once(' ').unwrap();
            combs.get(&x).unwrap().clone()
        })
        .sum::<u64>();

    println!("score = {}", score);
}
