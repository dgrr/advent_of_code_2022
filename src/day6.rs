use std::env;
use std::io::stdin;

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

fn get_unique(window_size: usize) -> Vec<usize> {
    stdin()
        .lines()
        .map(|x| x.unwrap())
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .windows(window_size)
                .position(|win| {
                    for i in 0..window_size {
                        for j in 0..window_size {
                            if i != j && win[i] == win[j] {
                                return false;
                            }
                        }
                    }
                    true
                })
                .unwrap()
                + window_size
        })
        .collect::<Vec<usize>>()
}

fn first_part() {
    let res = get_unique(4);
    println!("{:?}", res);
}

fn second_part() {
    let res = get_unique(14);
    println!("{:?}", res);
}
