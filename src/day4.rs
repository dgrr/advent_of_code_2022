use std::env;
use std::io::stdin;
use std::time::Instant;

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
    let started = Instant::now();
    let res = stdin().lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let (first, second) = x.split_once(',').unwrap();
            let first_range = first.split_once('-').unwrap();
            let second_range = second.split_once('-').unwrap();

            let range_a = (
                first_range.0.parse::<u64>().unwrap(),
                first_range.1.parse::<u64>().unwrap(),
            );
            let range_b = (
                second_range.0.parse::<u64>().unwrap(),
                second_range.1.parse::<u64>().unwrap(),
            );

            (range_a.0 >= range_b.0 && range_a.1 <= range_b.1) ||
                (range_b.0 >= range_a.0 && range_b.1 <= range_a.1)
        })
        .map(|x| if x { 1u64 } else { 0u64 })
        .sum::<u64>();

    println!("{} in {:?}", res, started.elapsed());
}

fn second_part() {
    let res = stdin().lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let (first, second) = x.split_once(',').unwrap();
            let first_range = first.split_once('-').unwrap();
            let second_range = second.split_once('-').unwrap();

            let range_a = (
                first_range.0.parse::<u64>().unwrap(),
                first_range.1.parse::<u64>().unwrap(),
            );
            let range_b = (
                second_range.0.parse::<u64>().unwrap(),
                second_range.1.parse::<u64>().unwrap(),
            );

            (range_a.0 >= range_b.0 && range_a.0 <= range_b.1) ||
                (range_b.0 >= range_a.0 && range_b.0 <= range_a.1)
        })
        .map(|x| if x { 1u64 } else { 0u64 })
        .sum::<u64>();

    println!("{}", res);
}
