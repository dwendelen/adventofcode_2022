extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut c = 0;
    let mut x = 1;

    for line in lines {
        if line.starts_with("addx ") {
            let num: i32 = line[5..].parse().unwrap();
            tick(&mut c, x);
            tick(&mut c, x);
            x += num;
        } else if line == "noop" {
            tick(&mut c, x);
        } else {
            panic!()
        }
    }
}

fn tick(c: &mut i32, x: i32) {
    let col = *c % 40;
    if col == 0 {
        println!()
    }
    if col == x - 1 || col == x || col == x + 1 {
        print!("#")
    } else {
        print!(" ")
    }
    *c += 1;
}