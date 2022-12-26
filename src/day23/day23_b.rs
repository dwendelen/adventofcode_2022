extern crate core;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day23/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut elfs: Vec<Elf> = lines.iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| *char == '#')
                .map(move |(c, _)| Elf::new(r as i32, c as i32))
        })
        .collect();

    let mut directions = vec![
        Direction::NORTH,
        Direction::SOUTH,
        Direction::WEST,
        Direction::EAST,
    ];
    let mut positions = HashSet::<(i32, i32)>::new();

    for elf in &elfs {
        positions.insert(elf.pos);
    }

    let mut round= 0;
    for i in 1..i32::MAX {
        let mut some_moved = false;
        let mut proposals = HashMap::<(i32, i32), usize>::new();
        for elf in &mut elfs {
            elf.update_proposal(&positions, &directions);
            if elf.pos != elf.proposal {
                let maybe_prop = proposals.get_mut(&elf.proposal);
                match maybe_prop {
                    Some(p) => { *p += 1; },
                    None => { proposals.insert(elf.proposal, 1); }
                }
            }
        }
        positions.clear();
        for elf in &mut elfs {
            let elf_moved = elf.update_positions(&proposals);
            some_moved = some_moved | elf_moved;
            positions.insert(elf.pos);
        }
        let removed = directions.remove(0);
        directions.push(removed);
        // print(&positions);
        if !some_moved {
            round = i;
            break
        }
    };
    println!("{}", round)
}


struct Elf {
    pos: (i32, i32),
    proposal: (i32, i32),
}

impl Elf {
    fn update_proposal(&mut self, positions: &HashSet<(i32, i32)>, directions: &Vec<Direction>) {
        let has_at_least_one_friend = directions.iter()
            .flat_map(|d|d.to_check(self.pos))
            .any(|tc|positions.contains(&tc));
        if has_at_least_one_friend {
            for dir in directions {
                let can_move_in_this_dir = dir.to_check(self.pos).iter()
                    .all(|tc|!positions.contains(tc));
                if can_move_in_this_dir {
                    self.proposal = dir.jump(self.pos);
                    break;
                }
            }
        }
    }

    fn update_positions(&mut self, proposals: &HashMap<(i32, i32), usize>) -> bool {
        if self.proposal == self.pos {
            return false;
        }
        let count = proposals.get(&self.proposal).unwrap();
        if *count == 1 {
            self.pos = self.proposal;
            true
        } else {
            self.proposal = self.pos;
            false
        }
    }
}

impl Elf {
    fn new(row: i32, col: i32) -> Elf {
        return Elf {
            pos: (row, col),
            proposal: (row, col),
        };
    }
}

#[derive(Copy, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn jump(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::NORTH => (pos.0 - 1, pos.1),
            Direction::EAST => (pos.0, pos.1 + 1),
            Direction::SOUTH => (pos.0 + 1, pos.1),
            Direction::WEST => (pos.0, pos.1 - 1),
        }
    }
    fn to_check(self, pos: (i32, i32)) -> [(i32, i32); 3] {
        match self {
            Direction::NORTH => [(pos.0 - 1, pos.1), (pos.0 - 1, pos.1 - 1), (pos.0 - 1, pos.1 + 1)],
            Direction::EAST  => [(pos.0, pos.1 + 1), (pos.0 - 1, pos.1 + 1), (pos.0 + 1, pos.1 + 1)],
            Direction::SOUTH => [(pos.0 + 1, pos.1), (pos.0 + 1, pos.1 - 1), (pos.0 + 1, pos.1 + 1)],
            Direction::WEST  => [(pos.0, pos.1 - 1), (pos.0 - 1, pos.1 - 1), (pos.0 + 1, pos.1 - 1)],
        }
    }
}

fn print(pos: &HashSet<(i32, i32)>) {
    let ((r_min, r_max), (c_min, c_max)) = pos.iter()
        .fold(((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)), |((acc_r_min, acc_r_max), (acc_c_min, acc_c_max)), &(pos_r, pos_c)|
            ((min(acc_r_min, pos_r), max(acc_r_max, pos_r)), (min(acc_c_min, pos_c), max(acc_c_max, pos_c)))
        );
    for r in r_min..r_max + 1 {
        for c in c_min..c_max + 1 {
            if pos.contains(&(r, c)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}