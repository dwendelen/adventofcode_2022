extern crate core;

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day14/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut grid = [[Type::None; 500]; 1000];

    for line in lines {
        let (mut prev, mut rest) =  parse_coord(line.as_str());
        loop {
            if rest == "" {
                break
            }
            let (new, restt) = parse_coord(rest);
            for x in min(prev.0, new.0)..max(prev.0, new.0) + 1 {
                for y in min(prev.1, new.1)..max(prev.1, new.1) + 1 {
                    grid[x][y] = Type::Rock;
                }
            }
            rest = restt;
            prev = new;
        }
    }

    let spawn = (500, 0);
    let bottom = grid.iter()
        .map(|c|
            c.iter().enumerate()
                .filter(|(_, v)| **v == Type::Rock)
                .map(|(i, _)| i)
                .max()
                .unwrap_or(0)
        )
        .max()
        .unwrap() + 2;

    loop {
        let mut coord = spawn;
        loop {
            if coord.1 == bottom - 1 {
                break
            }
            let down = (coord.0, coord.1 + 1);
            let down_left = (coord.0 - 1, coord.1 + 1);
            let down_right = (coord.0 + 1, coord.1 + 1);
            if grid[down.0][down.1] == Type::None {
                coord = down;
            } else if grid[down_left.0][down_left.1] == Type::None {
                coord = down_left;
            } else if grid[down_right.0][down_right.1] == Type::None {
                coord = down_right;
            } else {
                break;
            }
        };
        grid[coord.0][coord.1] = Type::Sand;
        if coord == spawn {
            break;
        }
    }

    for y in 0..12 {
        for x in 485..515 {
            print!("{}",
                match grid[x][y] {
                    Type::None => ".",
                    Type::Rock => "#",
                    Type::Sand => "o"
                }
            );
        }
        println!();
    }

    println!("{}", grid.iter()
        .map(|c|
            c.iter()
                .filter(|g|**g == Type::Sand)
                .count()
        )
        .sum::<usize>()
    )

}

fn parse_coord(rest: &str) -> ((usize, usize), &str) {
    let (i1, cnt) = parse_int(rest);
    if &cnt[0..1] != "," {
        panic!()
    }
    let (i2, cnt2) = parse_int(&cnt[1..]);
    let cnt3: &str = if cnt2 == "" {
        cnt2
    } else {
        if &cnt2[0..4] != " -> " {
            panic!()
        }
        &cnt2[4..]
    };
    return ((i1, i2), cnt3);
}

fn parse_int(rest: &str) -> (usize, &str) {
    let mut idx = 0;
    loop {
        if idx >= rest.len() || &rest[idx..idx + 1] == " " || &rest[idx..idx + 1] == "," {
            return (String::from(&rest[..idx]).parse().unwrap(), &rest[idx..])
        }
        idx += 1;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Type {
    None,
    Sand,
    Rock
}