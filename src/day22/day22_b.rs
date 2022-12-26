extern crate core;

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    let mut special_points: Vec<(i32, i32)> = Vec::new();
    for r in -1..grid.len() as i32 {
        for c in -1..grid[max(0, r) as usize].len() as i32 {
            let n1 = get_tile(&grid, (r + 1, c + 1));
            let n2 = get_tile(&grid, (r + 1, c));
            let n3 = get_tile(&grid, (r, c + 1));
            let n4 = get_tile(&grid, (r, c));
            let sum = [n1, n2, n3, n4].iter().filter(|t|**t == Limbo).count();
            if sum % 2 == 1 {
                special_points.push((r, c))
            }
        }
    }

    let face_size = special_points.iter().map(|a| a.0)
        .flat_map(|a| special_points.iter().map(move |b|(a - b.0).abs()))
        .filter(|a| *a != 0)
        .fold(i32::MAX, min);

    let start_pos: (i32, i32) = (0, grid[0].iter().position(|a| *a == Open).unwrap() as i32);

    let mut faces: Vec<Face> = Vec::new();
    let init_face = Face{
        map: start_pos,
        sides: [
            Side { other: 9, rot: Rotation::None},
            Side { other: 9, rot: Rotation::None},
            Side { other: 9, rot: Rotation::None},
            Side { other: 9, rot: Rotation::None},
        ]
    };
    faces.push(init_face);
    face_updated(&grid, &mut faces, 0, face_size);

    let mut dir = Direction::Right;
    let mut pos = (0, 0);
    let mut face = 0;

    for instruction in instructions {
        println!("{}: {} {}", face, pos.0, pos.1);
        match instruction {
            Rotate(r) => {
                dir = dir.turn(r)
            }
            Move(m) => {
                for _ in 0..m {
                    let tmp_pos = dir.move_along(pos, 1);
                    let (new_pos, new_dir, new_face) =
                        if tmp_pos.0 < 0 || tmp_pos.0 >= face_size || tmp_pos.1 < 0 || tmp_pos.1 >= face_size {
                            wrap_around(&faces, tmp_pos, dir, face, face_size)
                        } else {
                            (tmp_pos, dir, face)
                        };
                    let new_tile = get_tile_2(&grid, &faces, new_face, new_pos);
                    (pos, dir, face) = match new_tile {
                        Wall => (pos, dir, face),
                        Open => (new_pos, new_dir, new_face),
                        Limbo => panic!()
                    }
                }
            }
        }
    }
    println!("{}: {} {}", face, pos.0, pos.1);
    let real_row = faces[face].map.0 + pos.0 + 1;
    let real_col = faces[face].map.1 + pos.1 + 1;
    let res = 1000 * real_row + 4 * real_col + dir.idx() as i32;

    println!("{}", res);
    // 191146 = too high
}

fn face_updated(grid: &Vec<Vec<Tile>>, faces: &mut Vec<Face>, curr: usize, size: i32) {
    let pos = faces[curr].map;
    for dir in [Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
        if faces[curr].sides[dir.idx()].other == 9 {
            let face_pos = dir.move_along(pos, size);
            if get_tile(grid, face_pos) != Limbo {
                let new_face = Face {
                    map: face_pos,
                    sides: [
                        Side { other: 9, rot: Rotation::None },
                        Side { other: 9, rot: Rotation::None },
                        Side { other: 9, rot: Rotation::None },
                        Side { other: 9, rot: Rotation::None },
                    ]
                };
                faces.push(new_face);
                let new_other = faces.len() - 1;
                faces[curr].sides[dir.idx()] = Side { other: new_other, rot: Rotation::None };
                faces[new_other].sides[dir.reverse().idx()] = Side { other: curr, rot: Rotation::None };
                face_updated(grid, faces, faces.len() - 1, size);
                face_updated(grid, faces, curr, size);
            }
            for rot in [Rotation::Left, Rotation::Right] {
                let rot2 = rot.reverse();
                let face_1 = &faces[curr];
                if face_1.sides[dir.idx()].other != 9 {
                    break
                }
                //    111
                //    111
                //    111
                // 333222
                // 333222
                // 333222
                let dir_1_out = dir.turn(rot);
                let side_1_2 = &face_1.sides[dir_1_out.idx()];
                if side_1_2.other == 9 {
                    break;
                }
                let dir_2_in = dir_1_out.turn(side_1_2.rot);
                let dir_2_out = dir_2_in.turn(rot2);
                let face_2 = &faces[side_1_2.other];
                let side_2_3 = &face_2.sides[dir_2_out.idx()];
                let face_3_idx = side_2_3.other;
                if face_3_idx == 9 {
                    break
                }
                let dir_3_in = dir_2_out.turn(side_2_3.rot);
                let dir_3_out = dir_3_in.turn(rot2);
                let dir_3_in_from_1 = dir_3_out.reverse();
                let rot_1_3 = dir_3_in_from_1.minus(dir);
                let rot_3_1 = dir.minus(dir_3_in_from_1);
                faces[curr].sides[dir.idx()] = Side { other: face_3_idx, rot: rot_1_3 };
                faces[face_3_idx].sides[dir_3_out.idx()] = Side { other: curr, rot: rot_3_1 };
                face_updated(grid, faces, face_3_idx, size);
                face_updated(grid, faces, curr, size);
            }
        }
    }
}

fn get_tile(grid: &Vec<Vec<Tile>>, new_pos: (i32, i32)) -> Tile {
    grid.get(new_pos.0 as usize)
        .and_then(|l| l.get(new_pos.1 as usize))
        .map(|t|*t)
        .unwrap_or(Limbo)
}

fn get_tile_2(grid: &Vec<Vec<Tile>>, faces: &Vec<Face>, face: usize, new_pos: (i32, i32)) -> Tile {
    let map = faces[face].map;
    get_tile(grid, (map.0 + new_pos.0, map.1 + new_pos.1))
}

// (new_pos, new_dir, new_face)
fn wrap_around(faces: &Vec<Face>, pos: (i32, i32), dir: Direction, face: usize, face_size: i32) -> ((i32, i32), Direction, usize) {
    let side = &faces[face].sides[dir.idx()];
    let new_face = side.other;
    let new_dir = dir.turn(side.rot);

    // rotate around the middle of the square
    let rel_pos_times_two = (pos.0 * 2 - face_size + 1, pos.1 * 2 - face_size + 1);
    let tmp_rel_pos_times_two = side.rot.rotate(rel_pos_times_two);
    let tmp_pos = ((tmp_rel_pos_times_two.0 - 1 + face_size) / 2, (tmp_rel_pos_times_two.1 - 1 + face_size) / 2);
    let new_pos = (tmp_pos.0.rem_euclid(face_size), tmp_pos.1.rem_euclid(face_size));

    println!("Wrap {}: {},{} -> {}: {},{}", face, pos.0, pos.1, new_face, new_pos.0, new_pos.1);

    return (new_pos, new_dir, new_face);
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
            instr.push(Rotate(if nxt == "R" { Rotation::Right } else { Rotation::Left }));
            start = idx + 1;
            idx = start;
        } else {
            idx += 1;
        }
    }
}

enum Instruction {
    Rotate(Rotation),
    Move(i32)
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Limbo,
    Open,
    Wall
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn idx(self: Self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
    fn turn(self: Self, rotation: Rotation) -> Direction {
        match rotation {
            Rotation::Left => self.turn_left(),
            Rotation::Right => self.turn_right(),
            Rotation::Reverse => self.reverse(),
            Rotation::None => self,
        }
    }
    fn turn_left(self: Self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction:: Down
        }
    }
    fn turn_right(self: Self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction:: Up
        }
    }
    fn reverse(self: Self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction:: Right
        }
    }
    fn minus(self, other: Direction) -> Rotation {
        for rot in [Rotation::Reverse, Rotation::None, Rotation::Left, Rotation::Right] {
            if other.turn(rot) == self {
                return rot
            }
        }
        panic!()
    }
    fn vec(self: Self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1)
        }
    }
    fn move_along(self: Self, pos: (i32, i32), amount: i32) -> (i32, i32) {
        let v = self.vec();
        (pos.0 + amount * v.0, pos.1 + amount * v.1)
    }
}

#[derive(Copy, Clone)]
enum Rotation {
    Left, Right, None, Reverse
}

impl Rotation {
    fn reverse(self: Self) -> Rotation {
        match self {
            Rotation::Left => Rotation::Right,
            Rotation::Right => Rotation::Left,
            Rotation::None => Rotation::Reverse,
            Rotation::Reverse => Rotation::None
        }
    }

    fn rotate(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Rotation::Left => (-pos.1, pos.0),
            Rotation::Right => (pos.1, -pos.0),
            Rotation::None => pos,
            Rotation::Reverse => (-pos.0, -pos.1)
        }
    }
}

struct Face {
    map: (i32, i32),
    sides: [Side; 4],
}

struct Side {
    other: usize,
    rot: Rotation
}