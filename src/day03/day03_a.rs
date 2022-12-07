use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day03/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let res: i32 = lines.iter()
        .map(|l| to_priority(l))
        .sum();
    println!("{res}");
}

fn to_priority(string: &String) -> i32 {
    let part1: Vec<char> = string.chars().take(string.len() / 2).collect();
    let part2: Vec<char> = string.chars().skip(string.len() / 2).collect();

    for p1 in &part1 {
        for p2 in &part2 {
            if p1 == p2 {
                let pp1 = *p1 as u8;
                if b'a' <= pp1 && pp1 <= b'z' {
                    return (pp1 - b'a' + 1) as i32
                } else if b'A' <= pp1 && pp1 <= b'Z'{
                    return (pp1 - b'A' + 27) as i32
                }
            }
        }
    }
    panic!();
}