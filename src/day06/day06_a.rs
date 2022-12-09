use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day06/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let line = &lines[0];
    let mut window = line.as_bytes();
    let mut i = 4;
    loop {
        if window[0] != window[1] &&
            window[0] != window[2] && window[1] != window[2] &&
            window[0] != window[3] && window[1] != window[3] && window[2] != window[3]
        {
            println!("{i}");
            break;
        } else {
            window = &window[1..];
            i += 1;
        }
    }
}
