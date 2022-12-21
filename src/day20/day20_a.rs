extern crate core;

use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;

fn main() {
    let file = File::open("src/day20/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let initial: Vec<Num> = lines.iter()
        .enumerate()
        .map(|(i,l)| Num {idx: i, val: l.parse().unwrap()})
        .collect();

    let len = initial.len();

    let mut current = initial.clone();

    //print(&current);
    for i in 0..len {
        let num = initial[i].clone();
        let old_pos = current.iter().position(|j| j.idx == i).unwrap();
        let new_pos = (old_pos as i32 + num.val).rem_euclid(len as i32 - 1) as usize;

        if new_pos < old_pos {
            for j in (new_pos..old_pos).rev() {
                current[j + 1] = current[j].clone()
            }
            current[new_pos] = num;
        } else {
            for j in old_pos..new_pos {
                current[j] = current[j + 1].clone();
            }
            current[new_pos] = num;
        }
    }

    let pos_0 = current.iter().position(|i| i.val == 0).unwrap();
    let num_1000 = current[(pos_0 + 1000) % len].val;
    let num_2000 = current[(pos_0 + 2000) % len].val;
    let num_3000 = current[(pos_0 + 3000) % len].val;

    println!("{} {} {} -> {}", num_1000, num_2000, num_3000, num_1000 + num_2000 + num_3000);
}

fn print(current: &Vec<Num>) {
    for x in current {
        print!("{}", x.val);
        print!(",");
    }
    println!();
}

#[derive(Clone)]
struct Num {
    idx: usize,
    val: i32
}