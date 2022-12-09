use std::env;
use std::io::Read;
use std::io::stdin;

use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
ident = { (ASCII_ALPHANUMERIC | "." | "/")+ }
number = { ASCII_DIGIT+ }

WHITESPACE = _{ " " | "\t" }

file = { number ~ ident }
dir = { "dir" ~ ident }
file_or_dir = _{ file | dir }
cd = { "cd" ~ ident ~ NEWLINE }
ls = { "ls" ~ (NEWLINE ~ file_or_dir)* ~ NEWLINE? }
command = { "$" ~ (cd | ls) }
commands = _{ command+ }
"#]
struct Day7Parser;

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
    let mut data = String::new();
    stdin().read_to_string(&mut data).expect("lines");

    let commands = Day7Parser::parse(Rule::commands, data.as_str()).expect("parse");

    let mut max_depth = 0;
    let mut files = Vec::new();
    let mut path = Vec::new();
    for command in commands.into_iter() {
        match command.as_rule() {
            Rule::command => {
                let inner = command.into_inner().nth(0).unwrap();
                match inner.as_rule() {
                    Rule::cd => {
                        let dir = inner.into_inner().nth(0).unwrap().as_str().to_string();
                        if dir == ".." {
                            path.pop();
                        } else {
                            path.push(dir);
                        }
                    }
                    Rule::ls => {
                        for inner in inner.into_inner() {
                            match inner.as_rule() {
                                Rule::dir => {}
                                Rule::file => {
                                    let mut inner = inner.into_inner();
                                    let number =
                                        inner.next().unwrap().as_str().parse::<u64>().unwrap();
                                    // let name = inner.next().unwrap().as_str().to_string();

                                    max_depth = max_depth.max(path.len());

                                    let mut new_path = path.clone();

                                    while !new_path.is_empty() {
                                        files.push((new_path.clone(), number));
                                        new_path.pop();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let mut new_files = Vec::new();
    // this is quite fucked up because we need to sum all the directories in some weird way
    for (key, _) in files.iter() {
        let files = files
            .iter()
            .filter(|x| x.0.eq(key))
            .group_by(|x| key.clone())
            .into_iter()
            .map(|(key, values)| (key, values.map(|x| x.1).sum::<u64>()))
            .collect::<Vec<_>>();

        for file in files {
            new_files.push(file);
        }
    }

    let res = new_files
        .iter()
        .unique_by(|x| x.0.clone())
        .filter(|x| x.1 < 100_000)
        .map(|x| x.1)
        .sum::<u64>();
    println!("{:?}", res);
}

fn second_part() {
    let mut data = String::new();
    stdin().read_to_string(&mut data).expect("lines");

    let commands = Day7Parser::parse(Rule::commands, data.as_str()).expect("parse");

    let mut max_depth = 0;
    let mut files = Vec::new();
    let mut path = Vec::new();
    for command in commands.into_iter() {
        match command.as_rule() {
            Rule::command => {
                let inner = command.into_inner().nth(0).unwrap();
                match inner.as_rule() {
                    Rule::cd => {
                        let dir = inner.into_inner().nth(0).unwrap().as_str().to_string();
                        if dir == ".." {
                            path.pop();
                        } else {
                            path.push(dir);
                        }
                    }
                    Rule::ls => {
                        for inner in inner.into_inner() {
                            match inner.as_rule() {
                                Rule::dir => {}
                                Rule::file => {
                                    let mut inner = inner.into_inner();
                                    let number =
                                        inner.next().unwrap().as_str().parse::<u64>().unwrap();
                                    // let name = inner.next().unwrap().as_str().to_string();

                                    max_depth = max_depth.max(path.len());

                                    let mut new_path = path.clone();

                                    while !new_path.is_empty() {
                                        files.push((new_path.clone(), number));
                                        new_path.pop();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let mut new_files = Vec::new();
    // this is quite fucked up because we need to sum all the directories in some weird way
    for (key, _) in files.iter() {
        let files = files
            .iter()
            .filter(|x| x.0.eq(key))
            .group_by(|x| key.clone())
            .into_iter()
            .map(|(key, values)| (key, values.map(|x| x.1).sum::<u64>()))
            .collect::<Vec<_>>();

        for file in files {
            new_files.push(file);
        }
    }

    let mut new_files = new_files.iter().unique_by(|x| x.0.clone()).collect::<Vec<_>>();
    let root = new_files.iter().find(|x| x.0.len() == 1).unwrap();

    let left = 70_000_000 - root.1;
    let enough = 30_000_000u64 - left;

    new_files.sort_by(|a, b| a.1.cmp(&b.1));

    let res = new_files.iter().find(|x| x.1 >= enough).unwrap();

    println!("{:?}", res);
}
