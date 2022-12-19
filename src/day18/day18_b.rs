use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day18/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let droplet: Vec<(usize, usize, usize)> = lines.iter()
        .map(|l|{
            let pieces: Vec<&str> = l.split(",").collect();
            (pieces[0].parse().unwrap(), pieces[1].parse().unwrap(), pieces[2].parse().unwrap())
        })
        .collect();

    let mut inner_shell = [[[ false; 20]; 20]; 20];
    for drop in droplet {
        inner_shell[drop.0][drop.1][drop.2] = true;
    }
    let xs = inner_shell.len() as i32;
    let ys = inner_shell[0].len() as i32;
    let zs = inner_shell[0][0].len() as i32;
    let directions = [(0, 0, -1), (0, 0, 1), (0, -1, 0), (0, 1, 0), (-1, 0, 0), (1, 0, 0)];

    let mut todo: Vec<(i32, i32, i32)> = Vec::new();
    let mut outer_shell = [[[ false; 22]; 22]; 22];
    outer_shell[0][0][0] = true;
    todo.push((0, 0, 0));
    while todo.len() > 0 {
        let top = todo[0];
        todo = todo.iter().skip(1).map(|a|*a).collect();
        for dir in directions {
            let inner_coor = (top.0 + dir.0 - 1, top.1 + dir.1 - 1, top.2 + dir.2 - 1);
            let outer_coor = (top.0 + dir.0, top.1 + dir.1, top.2 + dir.2);
            let inner = inner_shell.get(inner_coor.0 as usize)
                .and_then(|g|g.get(inner_coor.1 as usize))
                .and_then(|g| g.get(inner_coor.2 as usize))
                .and_then(|g|Some(*g))
                .unwrap_or(false);
            let outer = outer_shell.get(outer_coor.0 as usize)
                .and_then(|g|g.get(outer_coor.1 as usize))
                .and_then(|g| g.get(outer_coor.2 as usize))
                .and_then(|g|Some(*g))
                .unwrap_or(true);
            if inner == false && outer == false {
                outer_shell[outer_coor.0 as usize][outer_coor.1 as usize][outer_coor.2 as usize] = true;
                todo.push(outer_coor);
            }
        }
    }

    let mut outer_shell_surface = 0;
    for x in 0..(xs + 2) {
        for y in 0..(ys + 2) {
            for z in 0..(zs + 2) {
                if outer_shell[x as usize][y as usize][z as usize] {
                    let nb_neighbours = directions.iter().filter(|d| {
                        let neighbour = outer_shell.get((x + d.0) as usize)
                            .and_then(|g|g.get((y + d.1) as usize))
                            .and_then(|g| g.get((z + d.2) as usize))
                            .and_then(|g|Some(*g))
                            .unwrap_or(true);
                        !neighbour
                    }).count();
                    outer_shell_surface += nb_neighbours;
                }
            }
        }
    }

    println!("{}", outer_shell_surface as i32);
}