extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day11/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut monkeys: Vec<Monkey> = lines.chunks(7)
        .map(|chunk| {
            let items: Vec<i32> = chunk[1][18..]
                .split(|i: char| i == ',')
                .map(|i| i.trim().parse::<i32>().unwrap())
                .collect();
            let op = match &chunk[2][23..24] {
                "*" => { multiply }
                "+" => { sum }
                _ => panic!()
            };
            let val_part: &str = &chunk[2][25..];
            let val: Option<i32> = match val_part {
                "old" => { None }
                _ => { Some(val_part.parse().unwrap()) }
            };
            let test: i32 = (&chunk[3][21..]).parse().unwrap();
            let on_true: usize = (&chunk[4][29..]).parse().unwrap();
            let on_false: usize = (&chunk[5][30..]).parse().unwrap();
            Monkey {
                items,
                op,
                val,
                test,
                on_true,
                on_false,
                inspections: 0,
            }
        })
        .collect();

    let amount_monkey = monkeys.len();
    for _ in 0..20 {
        for i in 0..amount_monkey {
            loop {
                let my_monkey = &mut monkeys[i];
                let head = my_monkey.items.first();
                if head.is_none() {
                    break
                }
                my_monkey.inspections += 1;
                let item = *head.unwrap();
                my_monkey.items = my_monkey.items.iter().skip(1).map(|i| *i).collect();
                let other_val = my_monkey.val.unwrap_or(item);
                let op = my_monkey.op;
                let new_val = op(item, other_val) / 3;
                let other_idx: usize = if new_val % my_monkey.test == 0 {
                    my_monkey.on_true
                } else {
                    my_monkey.on_false
                };
                let other_monkey = &mut monkeys[other_idx];
                other_monkey.items.push(new_val);
            }
        }
    }

    let mut inspections: Vec<i32> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();


    let res = inspections[0] * inspections[1];
    println!("{res}")
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

struct Monkey {
    items: Vec<i32>,
    op: fn(i32, i32) -> i32,
    val: Option<i32>,
    test: i32,
    on_true: usize,
    on_false: usize,
    inspections: i32,
}