use std::cmp::max;
use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Captures, Regex};

fn main() {
    let file = File::open("src/day17/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let instructions: Vec<bool> = lines[0].chars()
        .map(|c|match c {
            '<' => false,
            '>' => true,
            _ => panic!()
        })
        .collect();
    let blocks: [Vec<u8>; 5] = [
        Vec::from([
            0b00111100,
            0b00000000,
            0b00000000,
            0b00000000
        ]),
        Vec::from([
            0b00010000,
            0b00111000,
            0b00010000,
            0b00000000
        ]),
        Vec::from([
            0b00111000,
            0b00001000,
            0b00001000,
            0b00000000
        ]),
        Vec::from([
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000
        ]),
        Vec::from([
            0b00110000,
            0b00110000,
            0b00000000,
            0b00000000
        ])
    ];
    let mut state = vec!(0 as u8; 10000);
    let mut max_y: i32 = -1;
    let mut i  = 0;
    for b in 0..2022 {
        let mut block = blocks[b % blocks.len()].clone();
        let mut y: i32 = max_y + 4;
        loop {
            let instruction = instructions[i % instructions.len()];
            i += 1;

            if instruction {
                if block.iter().all(|l|*l & 0b00000010 == 0) {
                    let new_block = block.iter()
                        .map(|l| *l >> 1)
                        .collect();
                    if !crashes(y, &new_block, &state) {
                        block = new_block;
                    }
                }
            } else {
                if block.iter().all(|l|*l & 0b10000000 == 0) {
                    let new_block = block.iter()
                        .map(|l| *l << 1)
                        .collect();
                    if !crashes(y, &new_block, &state) {
                        block = new_block;
                    }
                }
            }

            if crashes(y - 1, &block, &state) {
                for j in 0..4 {
                    let idx = (y + j) as usize;
                    state[idx] = state[idx] | block[j as usize];
                    if state[idx] != 0 {
                        max_y = max((y + j), max_y);
                    }
                }
                println!("{}", max_y);
                break
            } else {
                y -= 1;
            }
        }
    }

    println!("{}", max_y + 1)
}

fn crashes(y: i32, block: &Vec<u8>, state: &Vec<u8>) -> bool {
    return y == -1 ||
        block[0] & state[y as usize] != 0 ||
        block[1] & state[(y + 1) as usize] != 0 ||
        block[2] & state[(y + 2) as usize] != 0 ||
        block[3] & state[(y + 3) as usize] != 0
}

fn print(state: &Vec<u8>, yy: i32, block: &Vec<u8>) {
    for y in 0..60 {
        let mut bit = 0b10000000;
        for _ in 0..8 {
            let mut c = if state[60 - y - 1] & bit == 0 {
                "."
            } else {
                "#"
            };
            let yyy = (60 - y - 1) as i32;
            if yyy == yy + 0 && block[0] & bit != 0 ||
                yyy == yy + 1 && block[1] & bit != 0 ||
                yyy == yy + 2 && block[2] & bit != 0 ||
                yyy == yy + 3 && block[3] & bit != 0 {
                c = "@";
            }
            print!("{}", c);
            bit = bit >> 1
        }
        println!()
    }
}