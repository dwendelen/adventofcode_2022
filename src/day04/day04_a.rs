use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Captures, Regex};

fn main() {
    let items = parse("src/day04/input.txt", r"(\d*)-(\d*),(\d*)-(\d*)", |cap| {
        Item {
            start1: cap[1].parse().unwrap(),
            end1: cap[2].parse().unwrap(),
            start2: cap[3].parse().unwrap(),
            end2: cap[4].parse().unwrap(),
        }
    });

    let res: usize = items.iter()
        .filter(|i| i.start1 <= i.start2 && i.end2 <= i.end1 || i.start2 <= i.start1 && i.end1 <= i.end2)
        .count();
    println!("{res}");
}

struct Item {
    start1: i32,
    end1: i32,
    start2: i32,
    end2: i32
}

fn parse<T, F: Fn(&Captures) -> T>(file: &str, regex: &str, factory: F) -> Vec<T> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let re = Regex::new(regex).unwrap();
    lines.iter()
        .map(|l| {
            let captures = re.captures(l.as_str()).unwrap();
            factory(&captures)
        })
        .collect()
}