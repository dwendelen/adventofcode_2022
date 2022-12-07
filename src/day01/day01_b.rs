use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut all: Vec<i32> = lines.split(|l| l == "")
        .map(|g| g.iter().fold(0, |a, b| a + b.parse::<i32>().unwrap()))
        .collect();
    all.sort();
    all.reverse();
    let res = all.as_slice()[..3]
        .iter().fold(0, |a,b| a + b);

    println!("{res}");
}
