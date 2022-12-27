extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    assert_eq!(from_snafu("1121-1110-1=0"), 314159265);
    assert_eq!("1121-1110-1=0", to_snafu(314159265));

    let file = File::open("src/day25/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let sum: u64 = lines.iter()
        .map(|l|from_snafu(l.as_str()))
        .sum();
    println!("{}", to_snafu(sum))
}

fn from_snafu(text: &str) -> u64 {
    let mut acc: u64 = 0;
    for char in text.chars() {
        acc *= 5;
        match char {
            '0' => { acc += 0; }
            '1' => { acc += 1; }
            '2' => { acc += 2; }
            '-' => { acc -= 1; }
            '=' => { acc -= 2; }
            _ => panic!()
        }
    }
    acc
}

fn to_snafu(num: u64) -> String {
    let mut todo = num;
    let mut acc = String::from("");
    loop {
        if todo == 0 {
            return acc
        }
        match todo % 5 {
            0 => { acc = String::from("0") + acc.as_str() },
            1 => { acc = String::from("1") + acc.as_str() },
            2 => { acc = String::from("2") + acc.as_str() },
            3 => { acc = String::from("=") + acc.as_str(); todo += 5 },
            4 => { acc = String::from("-") + acc.as_str(); todo += 5 },
            _ => panic!()
        }
        todo = todo / 5
    }
}