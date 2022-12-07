extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day01/input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        let sz = reader.read_line(&mut line);
        match sz {
            Ok(szz) => {
                if szz == 0 {
                    break
                } else {
                    if line.ends_with("\n") {
                        line = line.chars().take(line.len() - 1).collect()
                    }
                    lines.push(line)
                }
            }
            _ => { panic!("") }
        }
    }
    let res = lines.split(|l| l == "")
        .map(|g| g.iter().fold(0, |a, b| a + b.parse::<i32>().unwrap()))
        .max()
        .unwrap();
    println!("{res}");
}
