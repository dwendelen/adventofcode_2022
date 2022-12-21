extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day21/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let monkeys: Vec<Monkey> = lines.iter()
        .map(|line: &String| {
            let name = String::from(&line[0..4]);

            if line.len() == 17 {
                let op = match &line[11..12] {
                    "+" => add,
                    "*" => mul,
                    "/" => div,
                    "-" => min,
                    _ => panic!()
                };
                Monkey {
                    name,
                    op,
                    val: None,
                    left: String::from(&line[6..10]),
                    right: String::from(&line[13..17]),
                }
            } else {
                Monkey {
                    name,
                    op: |a,b| panic!(),
                    val: Some(*(&line[6..].parse().unwrap())),
                    left: String::from(""),
                    right: String::from(""),
                }
            }
        })
        .collect();


    println!("{}", val(monkeys.iter().find(|m|m.name == "root").unwrap(), &monkeys))
}

fn val(monkey: &Monkey, monkeys: &Vec<Monkey>) -> i64 {
    match monkey.val {
        Some(v) => v,
        None => {
            let l = monkeys.iter().find(|m|m.name == monkey.left).unwrap();
            let r = monkeys.iter().find(|m|m.name == monkey.right).unwrap();
            (monkey.op)(val(l, monkeys), val(r, monkeys))
        }
    }
}

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn mul(a: i64, b: i64) -> i64 {
    return a * b;
}

fn min(a: i64, b: i64) -> i64 {
    return a - b;
}

fn div(a: i64, b: i64) -> i64 {
    return a / b;
}

struct Monkey {
    name: String,
    op: fn(i64, i64) -> i64,
    val: Option<i64>,
    left: String,
    right: String
}