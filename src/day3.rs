use std::env;
use std::io::stdin;

use itertools::Itertools;

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
    let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let prios: Vec<u32> = (1..=52).collect();

    let res = stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            first
                .chars()
                .cartesian_product(second.chars())
                .filter(|(a, b)| *a == *b)
                .map(|(a, _)| a)
                .nth(0)
        })
        .map(|char| {
            if let Some(char) = char {
                let index = letters.iter().position(|letter| *letter == char).unwrap();
                return prios[index];
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("{}", res);
}

fn second_part() {
    let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let prios: Vec<u32> = (1..=52).collect();

    let lines: Vec<String> = stdin().lines().map(|line| line.unwrap()).collect();

    let res = lines
        .chunks(3)
        .map(|lines| {
            lines[0]
                .chars()
                .cartesian_product(lines[1].chars())
                .cartesian_product(lines[2].chars())
                .filter(|((a, b), c)| *a == *b && *b == *c)
                .map(|((a, _), _)| a)
                .nth(0)
        })
        .map(|char| {
            if let Some(char) = char {
                let index = letters.iter().position(|letter| *letter == char).unwrap();
                return prios[index];
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("{}", res);
}
