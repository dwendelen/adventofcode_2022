extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::raw::gid_t;
use crate::Instruction::{Move, Rotate};
use crate::Tile::{Open, Limbo, Wall};

fn main() {
    let file = File::open("src/day22/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let instructions: Vec<Instruction> = parse_instructions(lines.last().unwrap());
    let grid: Vec<Vec<Tile>> = lines.iter()
        .take(lines.len() - 2)
        .map(|l| l.chars()
            .map(|c| match c {
                ' ' => Limbo,
                '.' => Open,
                '#' => Wall,
                _ => panic!()
            })
            .collect()
        )
        .collect();

    let mut pos: (i32, i32) = (0, grid[0].iter().position(|a| *a == Open).unwrap() as i32);
    let mut dir: (i32, i32) = (0, 1);

    for instruction in instructions {
        // println!("{} {} {} {}", pos.0, pos.1, dir.0, dir.1);
        match instruction {
            Rotate(r) => {
                // right => r = c, c = -r
                // left => r = -c, c = r
                // r = dir.0
                // c = dir.1
                // left = -1
                // right = 1
                dir = (r * dir.1, -r * dir.0)
            }
            Move(m) => {
                for _ in 0..m {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    let new_tile = get_tile(&grid, new_pos);
                    pos = match new_tile {
                        Wall => pos,
                        Open => new_pos,
                        Limbo => wrap_around(&grid, new_pos, pos, dir)
                    }
                }
            }
        }
    }

    let face = match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => panic!()
    };

    let res = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + face;

    println!("{}", res)
    // 191146 = too high
}

fn get_tile(grid: &Vec<Vec<Tile>>, new_pos: (i32, i32)) -> Tile {
    grid.get(new_pos.0 as usize)
        .and_then(|l| l.get(new_pos.1 as usize))
        .map(|t|*t)
        .unwrap_or(Limbo)
}

fn wrap_around(grid: &Vec<Vec<Tile>>, pos: (i32, i32), old_pos:(i32, i32), dir: (i32, i32)) -> (i32, i32) {
    let mut new_pos = match dir {
        (0, 1) => (pos.0, 0),
        (0, -1) => (pos.0, grid[pos.0 as usize].len() as i32 - 1),
        (1, 0) => (0, pos.1),
        (-1, 0) => (grid.len() as i32 - 1, pos.1),
        _ => panic!()
    };
    loop {
        let tile = get_tile(grid, new_pos);
        match tile {
            Limbo => { new_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1) }
            Open => { return new_pos }
            Wall => { return old_pos }
        }
    }
}

fn parse_instructions(str: &str) -> Vec<Instruction> {
    let mut instr = Vec::new();
    let mut start = 0;
    let mut idx = 0;

    loop {
        if idx == str.len() {
            instr.push(Move((&str[start..idx]).parse().unwrap()));
            return instr;
        }
        let nxt = &str[idx..idx + 1];
        if nxt == "L" || nxt == "R" {
            instr.push(Move((&str[start..idx]).parse().unwrap()));
            instr.push(Rotate(if nxt == "R" { 1 } else { -1 }));
            start = idx + 1;
            idx = start;
        } else {
            idx += 1;
        }
    }
}

enum Instruction {
    Rotate(i32), // left = -1, right = 1
    Move(i32)
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Limbo,
    Open,
    Wall
}