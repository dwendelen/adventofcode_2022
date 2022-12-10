extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut t = 0;
    let mut x = 1;

    let mut score = 0;
    let mut next_t = 20;

    for line in lines {
        let old_t = t;
        let old_x = x;
        if line.starts_with("addx ") {
            let num: i32 = line[5..].parse().unwrap();
            x += num;
            t += 2
        } else if line == "noop" {
            t += 1
        } else {
            panic!()
        }
        if old_t < next_t && next_t <= t {
            score += old_x * next_t;
            println!("{next_t}: {old_x}");
            next_t += 40;
        }
    }

    println!("{score}")
}
