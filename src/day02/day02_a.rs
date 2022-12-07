use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day02/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let res: i32 = lines.iter()
        .map(|l| to_score(l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
        .sum();
    println!("{res}");
}

fn to_score(other: char, me: char) -> i32 {
    return match (other, me) {
        ('A', 'Y') => { 2 + 6 }
        ('B', 'Z') => { 3 + 6 }
        ('C', 'X') => { 1 + 6 }
        ('A', 'X') => { 1 + 3 }
        ('B', 'Y') => { 2 + 3 }
        ('C', 'Z') => { 3 + 3 }
        ('A', 'Z') => { 3 }
        ('B', 'X') => { 1 }
        ('C', 'Y') => { 2 }
        _ => { panic!() }
    }
}