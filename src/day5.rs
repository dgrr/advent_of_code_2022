use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{stdin, Read};

use pest::Parser;
use pest_derive::Parser;

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

#[derive(Parser)]
#[grammar_inline = r#"
alpha = { ASCII_ALPHA }
number = { ASCII_DIGIT+ }

whitespace = _{ " " }
valid_whitespace = { " "{3} }
letter = _{ "[" ~ alpha ~ "]" }
rule = _{ whitespace{,1} ~ letter | valid_whitespace ~ whitespace{,1} }
line = { rule+ ~ NEWLINE }
lines = _{ line+ }

index = _{ whitespace* ~ number+ ~ whitespace* }
indexes = { index+ }

body_line = { "move" ~ whitespace ~ number+ ~ whitespace ~ "from" ~ whitespace ~ number+ ~ whitespace ~ "to" ~ whitespace ~ number+ ~ NEWLINE }
body = _{ body_line+ }
file = _{ lines ~ indexes ~ NEWLINE ~ NEWLINE ~ body }
"#]
struct Day5Parser;

fn parse_file() -> (HashMap<u32, VecDeque<String>>, Vec<(u32, u32, u32)>) {
    let mut data = String::new();
    stdin().read_to_string(&mut data).expect("lines");

    let mut moves = Vec::new();
    let mut crates = HashMap::new();
    let pairs = Day5Parser::parse(Rule::file, data.as_str()).expect("parse");

    for record in pairs.clone().into_iter() {
        if let Rule::indexes = record.as_rule() {
            for field in record.into_inner() {
                let number = field.as_str().parse::<u32>().unwrap();
                crates.insert(number, VecDeque::new());
            }

            break;
        }
    }

    for record in pairs.clone().into_iter() {
        match record.as_rule() {
            Rule::line => {
                let mut i = 0;
                for field in record.into_inner() {
                    i += 1;
                    if let Rule::alpha = field.as_rule() {
                        let cell = crates.get_mut(&i).unwrap();
                        cell.push_front(field.as_str().to_string());
                    }
                }
            }
            Rule::body_line => {
                let numbers: Vec<u32> = record
                    .into_inner()
                    .map(|x| x.as_str().parse::<u32>().unwrap())
                    .collect();
                moves.push((numbers[0], numbers[1], numbers[2]));
            }
            _ => {}
        }
    }

    (crates, moves)
}

fn first_part() {
    let (mut crates, moves) = parse_file();

    for (n, from, to) in moves {
        let mut to_move = Vec::new();
        let src = crates.get_mut(&from).unwrap();
        for _ in 0..n {
            to_move.push(src.pop_back().unwrap());
        }

        let dst = crates.get_mut(&to).unwrap();
        for e in to_move.into_iter() {
            dst.push_back(e);
        }
    }

    let mut top = Vec::new();
    for i in 0..crates.len() {
        let src = crates.get(&(i as u32 + 1)).unwrap();
        top.push(src.back().unwrap().clone())
    }

    println!("{}", top.join(""));
}

fn second_part() {
    let (mut crates, moves) = parse_file();

    for (n, from, to) in moves {
        let mut to_move = VecDeque::new();
        let src = crates.get_mut(&from).unwrap();
        for _ in 0..n {
            to_move.push_front(src.pop_back().unwrap());
        }

        let dst = crates.get_mut(&to).unwrap();
        for e in to_move.into_iter() {
            dst.push_back(e);
        }
    }

    let mut top = Vec::new();
    for i in 0..crates.len() {
        let src = crates.get(&(i as u32 + 1)).unwrap();
        top.push(src.back().unwrap().clone())
    }

    println!("{}", top.join(""));
}
