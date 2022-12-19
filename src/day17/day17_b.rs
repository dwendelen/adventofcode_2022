use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let (y_max_start, blocks_start, y_max_stop, blocks_stop ) = find_loop_size(instructions.clone(), blocks.clone());
    let loop_size = blocks_stop - blocks_start;
    let y_max_two_loops = simulate_n_blocks(instructions.clone(), blocks.clone(), blocks_stop + 1 * loop_size);
    let y_max_three_loops = simulate_n_blocks(instructions.clone(), blocks.clone(), blocks_stop + 2 * loop_size);
    let y_max_four_loops = simulate_n_blocks(instructions.clone(), blocks.clone(), blocks_stop + 3 * loop_size);
    let y_max_five_loops = simulate_n_blocks(instructions.clone(), blocks.clone(), blocks_stop + 4 * loop_size);
    let blocks_end_three_loops = blocks_start + 3 * loop_size;
    let y_max_delta = y_max_three_loops - y_max_two_loops;


    let total_blocks: i64 = 1000000000000;
    let nb_extra_loops: i64 = (total_blocks - blocks_end_three_loops as i64)  / loop_size as i64;
    let rest = (total_blocks - blocks_end_three_loops as i64) % loop_size as i64;

    let y_max_rest_plus_three_loops_plus_prelude = simulate_n_blocks(instructions.clone(), blocks.clone(), blocks_end_three_loops + rest as usize);

    let res = (y_max_rest_plus_three_loops_plus_prelude as i64 + 1) + (nb_extra_loops * y_max_delta as i64);

    println!("{}", res)
}

// (y_max_start, blocks_start, y_max_stop, blocks_stop)
fn find_loop_size(instructions: Vec<bool>, blocks: [Vec<u8>; 5]) -> (usize, usize, usize, usize) {
    let nb_blocks = blocks.len();
    let nb_instructions = instructions.len();
    let mut state = vec!(0 as u8; 10000000);
    let mut max_y: i32 = -1;
    let mut i  = 0;
    let mut b: usize = 0;
    let mut offsets: Vec<(usize, usize)> = instructions.iter().map(|_|(0,0)).collect();

    // Warmup to lock into the pattern
    for _ in 0..100000 {
        simulate_one_block(&instructions, &blocks, &mut state, &mut max_y, &mut i, &mut b);
    }
    loop {
        if b % nb_blocks == 0 {
            let old = offsets[i % nb_instructions];
            if old == (0, 0) {
                offsets[i % nb_instructions] = (max_y as usize, b);
            } else {
                return (old.0, old.1, max_y as usize, b);
            }
        }
        simulate_one_block(&instructions, &blocks, &mut state, &mut max_y, &mut i, &mut b);
    }
}

// (state, max_y)
fn simulate_n_blocks(instructions: Vec<bool>, blocks: [Vec<u8>; 5], n: usize) -> i32 {
    let mut state = vec!(0 as u8; 10000000);
    let mut max_y: i32 = -1;
    let mut i  = 0;
    let mut b: usize = 0;
    loop {
        if b >= n {
            return max_y
        }
        simulate_one_block(&instructions, &blocks, &mut state, &mut max_y, &mut i, &mut b);
    }
}

fn simulate_one_block(instructions: &Vec<bool>, blocks: &[Vec<u8>; 5], state: &mut Vec<u8>, max_y: &mut i32, i: &mut usize, b: &mut usize) {
    let mut block = blocks[*b % blocks.len()].clone();
    let mut y: i32 = *max_y + 4;
    loop {
        let instruction = instructions[*i % instructions.len()];
        *i += 1;

        if instruction {
            if block.iter().all(|l| *l & 0b00000010 == 0) {
                let new_block = block.iter()
                    .map(|l| *l >> 1)
                    .collect();
                if !crashes(y, &new_block, &state) {
                    block = new_block;
                }
            }
        } else {
            if block.iter().all(|l| *l & 0b10000000 == 0) {
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
                    *max_y = max((y + j), *max_y);
                }
            }
            break
        } else {
            y -= 1;
        }
    }
    *b += 1;
}

fn crashes(y: i32, block: &Vec<u8>, state: &Vec<u8>) -> bool {
    return y == -1 ||
        block[0] & state[y as usize] != 0 ||
        block[1] & state[(y + 1) as usize] != 0 ||
        block[2] & state[(y + 2) as usize] != 0 ||
        block[3] & state[(y + 3) as usize] != 0
}

fn print(state: &Vec<u8>, max_y: i32, amount: i32) {
    for y in ((max_y + 1 - amount )..(max_y + 1)).rev() {
        let mut bit = 0b10000000;
        for _ in 0..8 {
            let mut c = if state[y as usize] & bit == 0 {
                "."
            } else {
                "#"
            };
            // let yyy = (60 - y - 1) as i32;
            // if yyy == yy + 0 && block[0] & bit != 0 ||
            //     yyy == yy + 1 && block[1] & bit != 0 ||
            //     yyy == yy + 2 && block[2] & bit != 0 ||
            //     yyy == yy + 3 && block[3] & bit != 0 {
            //     c = "@";
            // }
            print!("{}", c);
            bit = bit >> 1
        }
        println!()
    }
    println!()
}