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

    let mut grid = [[[ false ;20]; 20]; 20];
    for drop in droplet {
        grid[drop.0][drop.1][drop.2] = true;
    }
    let mut surface = 0;
    let xs = grid.len() as i32;
    let ys = grid[0].len() as i32;
    let zs = grid[0][0].len() as i32;
    let directions = [(0, 0, -1), (0, 0, 1), (0, -1, 0), (0, 1, 0), (-1, 0, 0), (1, 0, 0)];

    for x in 0..xs {
        for y in 0..ys {
            for z in 0..zs {
                if grid[x as usize][y as usize][z as usize] {
                    let nb_neighbours = directions.iter().filter(|d| {
                        let neighbour = grid.get((x + d.0) as usize)
                            .and_then(|g|g.get((y + d.1) as usize))
                            .and_then(|g| g.get((z + d.2) as usize))
                            .and_then(|g|Some(*g))
                            .unwrap_or(false);
                        !neighbour
                    }).count();
                    surface += nb_neighbours;
                }
            }
        }
    }

    println!("{}", surface);
}