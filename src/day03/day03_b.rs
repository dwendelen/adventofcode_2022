use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day03/example.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let res: i32 = lines
        .chunks(3)
        .map(|l| to_priority(l))
        .sum();
    println!("{res}");
}

fn to_priority(string: &[String]) -> i32 {
    let str1: Vec<char> = string[0].chars().collect();
    let str2: Vec<char> = string[1].chars().collect();
    let str3: Vec<char> = string[2].chars().collect();

    for c1 in &str1 {
        for c2 in &str2 {
            if c1 == c2 {
                for c3 in &str3 {
                    if c1 == c3 {
                        let pp1 = *c1 as u8;
                        if b'a' <= pp1 && pp1 <= b'z' {
                            return (pp1 - b'a' + 1) as i32
                        } else if b'A' <= pp1 && pp1 <= b'Z'{
                            return (pp1 - b'A' + 27) as i32
                        }
                    }
                }
            }
        }
    }

    panic!();
}