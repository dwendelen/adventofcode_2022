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
                let op: Box<dyn Op> = match &line[11..12] {
                    "+" => Box::new(Plus{}),
                    "*" => Box::new(Multiply{}),
                    "/" => Box::new(Divide{}),
                    "-" => Box::new(Minus{}),
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
                    op: Box::new(NoOp{}),
                    val: Some(*(&line[6..].parse().unwrap())),
                    left: String::from(""),
                    right: String::from(""),
                }
            }
        })
        .collect();


    let root_monkey = monkeys.iter().find(|m| m.name == "root").unwrap();
    let l = monkeys.iter().find(|m|m.name == root_monkey.left).unwrap();
    let r = monkeys.iter().find(|m|m.name == root_monkey.right).unwrap();
    let val_l = val(l, &monkeys);
    let val_r = val(r, &monkeys);

    let res = match (val_l, val_r) {
        (Some(vl), None) => {
            make_eq(r, &monkeys, vl)
        }
        (None, Some(vr)) => {
            make_eq(l, &monkeys, vr)
        }
        _ => panic!()
    };
    println!("{}", res)
}

fn val(monkey: &Monkey, monkeys: &Vec<Monkey>) -> Option<i64> {
    if monkey.name == "humn" {
        return None
    }
    match monkey.val {
        Some(v) => Some(v),
        None => {
            let l = monkeys.iter().find(|m|m.name == monkey.left).unwrap();
            let r = monkeys.iter().find(|m|m.name == monkey.right).unwrap();

            val(l, monkeys)
                .and_then(|lv|val(r, monkeys)
                    .map(|rv|monkey.op.apply(lv, rv))
                )
        }
    }
}

fn make_eq(monkey: &Monkey, monkeys: &Vec<Monkey>, to_eq: i64) -> i64 {
    if monkey.name == "humn" {
        return to_eq;
    }
    let l = monkeys.iter().find(|m|m.name == monkey.left).unwrap();
    let r = monkeys.iter().find(|m|m.name == monkey.right).unwrap();
    let val_l = val(l, &monkeys);
    let val_r = val(r, &monkeys);
    match (val_l, val_r) {
        (Some(vl), None) => {
            make_eq(r, monkeys, monkey.op.solve_r(to_eq, vl))
        }
        (None, Some(vr)) => {
            make_eq(l, monkeys, monkey.op.solve_l(to_eq, vr))
        }
        _ => panic!()
    }
}

struct Monkey {
    name: String,
    op: Box<dyn Op>,
    val: Option<i64>,
    left: String,
    right: String
}

trait Op {
    fn apply(self: &Self, l: i64, r: i64) -> i64;
    fn solve_l(self: &Self, eq: i64, r: i64) -> i64;
    fn solve_r(self: &Self, eq: i64, l: i64) -> i64;
}

struct Plus {

}

impl Op for Plus {
    fn apply(self: &Self, l: i64, r: i64) -> i64 {
        l + r
    }

    fn solve_l(self: &Self, eq: i64, r: i64) -> i64 {
        eq - r
    }

    fn solve_r(self: &Self, eq: i64, l: i64) -> i64 {
        eq - l
    }
}

struct Minus {

}

impl Op for Minus {
    fn apply(self: &Self, l: i64, r: i64) -> i64 {
        l - r
    }

    fn solve_l(self: &Self, eq: i64, r: i64) -> i64 {
        // eq = l - r
        // l = eq + r
        eq + r
    }

    fn solve_r(self: &Self, eq: i64, l: i64) -> i64 {
        // eq = l - r
        // r = l - eq
        l - eq
    }
}

struct Multiply {

}

impl Op for Multiply {
    fn apply(self: &Self, l: i64, r: i64) -> i64 {
        l * r
    }

    fn solve_l(self: &Self, eq: i64, r: i64) -> i64 {
        eq / r
    }

    fn solve_r(self: &Self, eq: i64, l: i64) -> i64 {
        eq / l
    }
}

struct Divide {

}
impl Op for Divide {
    fn apply(self: &Self, l: i64, r: i64) -> i64 {
        l / r
    }

    fn solve_l(self: &Self, eq: i64, r: i64) -> i64 {
        // eq = l / r
        // l = eq * r
        eq * r
    }

    fn solve_r(self: &Self, eq: i64, l: i64) -> i64 {
        // eq = l / r
        // r = l / eq
        l / eq
    }
}

struct NoOp {

}

impl Op for NoOp {
    fn apply(self: &Self, l: i64, r: i64) -> i64 {
        panic!()
    }

    fn solve_l(self: &Self, eq: i64, r: i64) -> i64 {
        panic!()
    }

    fn solve_r(self: &Self, eq: i64, l: i64) -> i64 {
        panic!()
    }
}