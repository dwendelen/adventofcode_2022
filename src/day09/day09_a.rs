extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day09/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut all_places: HashSet<(i32, i32)> = HashSet::new();
    all_places.insert(tail);

    for line in lines {
        let dir: char = line.chars().nth(0).unwrap();
        let amount: i32 = String::from(&line.as_str()[2..]).parse().unwrap();
        for _ in 0..amount {
            head = match dir {
                'U' => { (head.0, head.1 + 1) }
                'D' => { (head.0, head.1 - 1) }
                'L' => { (head.0 - 1, head.1) }
                'R' => { (head.0 + 1, head.1) }
                _ => panic!()
            };
            let diff = match (head.0 - tail.0, head.1 - tail.1) {
                (-2,  0) => { (-1,  0) }
                ( 2,  0) => { ( 1,  0) }
                ( 0, -2) => { ( 0, -1) }
                ( 0,  2) => { ( 0,  1) }

                (-2, -1) => { (-1, -1) }
                (-2,  1) => { (-1,  1) }
                ( 2, -1) => { ( 1, -1) }
                ( 2,  1) => { ( 1,  1) }

                (-1, -2) => { (-1, -1) }
                (-1,  2) => { (-1,  1) }
                ( 1, -2) => { ( 1, -1) }
                ( 1,  2) => { ( 1,  1) }

                _ => (0, 0)
            };
            tail = (tail.0 + diff.0, tail.1 + diff.1);
            all_places.insert(tail);
            // let h0 = head.0;
            // let h1 = head.1;
            // let t0 = tail.0;
            // let t1 = tail.1;
            // println!("({h0}, {h1}) ({t0}, {t1})")
        }
    }
    let res = all_places.len();
    println!("{res}")
}
