use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let res = lines.split(|l| l == "")
        .map(|g| g.iter().fold(0, |a, b| a + b.parse::<i32>().unwrap()))
        .max()
        .unwrap();
    println!("{res}");
}
