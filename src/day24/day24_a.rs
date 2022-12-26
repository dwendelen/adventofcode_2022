extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day24/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let blizzards_left = calc_blizzards_horizontal(&lines, '<');
    let blizzards_right = calc_blizzards_horizontal(&lines, '>');
    let blizzards_up = calc_blizzards_vertical(&lines, '^');
    let blizzards_down = calc_blizzards_vertical(&lines, 'v');

    let rows = lines.len() - 2;
    let cols = lines.first().unwrap().len() - 2;
    let steps = rows * cols / gcd(rows, cols);

    let mut grid: Vec<Vec<Cell>> = (0..rows)
        .map(|r|
            (0..cols)
                .map(|c| {
                    Cell {
                        time_table: (0..steps)
                            .map(|t|
                                blizzards_right[r].iter().all(|b| (*b as i32 + t as i32).rem_euclid(cols as i32) != c as i32) &&
                                    blizzards_left[r].iter().all(|b| (*b as i32 - t as i32).rem_euclid(cols as i32) != c as i32) &&
                                    blizzards_down[c].iter().all(|b| (*b as i32 + t as i32).rem_euclid(rows as i32) != r as i32) &&
                                    blizzards_up[c].iter().all(|b| (*b as i32 - t as i32).rem_euclid(rows as i32) != r as i32)
                            )
                            .collect(),
                        fastest_table: (0..steps).map(|_|usize::MAX).collect()
                    }
                })
                .collect()
        )
        .collect();

    let best = do_run(&mut grid, 0, 0, 0, rows - 1, cols - 1);

    println!("{}", best)
}

fn do_run(grid: &mut Vec<Vec<Cell>>, start_row: usize, start_col: usize, start_t: usize, end_row: usize, end_col: usize) -> usize {
    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let steps = rows * cols / gcd(rows, cols);

    for row in 0..rows {
        for col in 0..cols {
            for t in 0..steps {
                grid[row][col].fastest_table[t] = usize::MAX - 1;
            }
        }
    }

    let mut todo = Vec::<(usize, usize, usize)>::new();
    for t in (start_t + 1)..(start_t + 1 + steps) {
        let slot = t % steps;
        if grid[start_col][start_row].time_table[slot] {
            grid[start_col][start_row].fastest_table[slot] = t;
            todo.push((start_col, start_row, t))
        }
    }

    let mut idx = 0;
    loop {
        if idx >= todo.len() {
            break
        }
        let head = todo[idx];
        let new_time = head.2 + 1;
        let new_slot = new_time % steps;
        for (row, col, new_slot) in [(head.0 as i32, head.1 as i32, new_slot), (head.0 as i32, head.1 as i32 - 1, new_slot), (head.0 as i32, head.1 as i32 + 1, new_slot), (head.0 as i32 - 1, head.1 as i32, new_slot), (head.0 as i32 + 1, head.1 as i32, new_slot)] {
            if row < 0 || rows as i32 <= row || col < 0 || cols as i32 <= col {
                continue
            }
            let u_row = row as usize;
            let u_col = col as usize;
            let other_cell = &grid[u_row][u_col];
            if other_cell.time_table[new_slot] && new_time < other_cell.fastest_table[new_slot] {
                grid[u_row][u_col].fastest_table[new_slot] = new_time;
                todo.push((u_row, u_col, new_time));
            }
        }

        idx += 1;
        if idx > 10000000 {
            todo = Vec::from(&todo[10000000..]);
            idx -= 10000000;
        }
    }
    let time = grid[end_row][end_col].fastest_table
        .iter()
        .min()
        .unwrap();
    return *time + 1;
}

fn calc_blizzards_horizontal(lines: &Vec<String>, char: char) -> Vec<Vec<usize>> {
    lines.iter()
        .skip(1)
        .take(lines.len() - 2)
        .map(|line| line.chars()
            .skip(1)
            .take(line.len() - 2)
            .enumerate()
            .filter(|(_, c)| *c == char)
            .map(|(c, _)| c)
            .collect()
        )
        .collect()
}

fn calc_blizzards_vertical(lines: &Vec<String>, char: char) -> Vec<Vec<usize>> {
    let rows = lines.len() - 2;
    let cols = lines.first().unwrap().len() - 2;
    (0..cols).map(|c|
        (0..rows)
            .map(|r| lines[r + 1].chars().nth(c + 1).unwrap())
            .enumerate()
            .filter(|(_, c)| *c == char)
            .map(|(c, _)| c)
            .collect()
    )
        .collect()
}

struct Cell {
    time_table: Vec<bool>,
    fastest_table: Vec<usize>
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}