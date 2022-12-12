extern crate core;

use std::cmp::Ordering;
use std::collections::binary_heap::Iter;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day12/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let nb_lines = lines.len();
    let nb_cols = lines.first().unwrap().len();
    let mut end: (usize, usize) = (usize::MAX, usize::MAX);

    let mut dist: Vec<Vec<usize>> = Vec::new();
    dist.resize(nb_lines, {
        let mut vec = Vec::new();
        vec.resize(nb_cols, usize::MAX);
        vec
    });
    let mut todo: BinaryHeap<State> = BinaryHeap::new();

    let height: Vec<Vec<u8>> = lines.iter()
        .enumerate()
        .map(|(l, line)| {
            line.bytes()
                .enumerate()
                .map(|(c, b)| {
                    if b == b'S' {
                        dist[l][c] = 0;
                        todo.push(State {node: (l, c), distance: 0});
                        0 as u8
                    } else if b == b'E' {
                        end = (l, c);
                        25 as u8
                    } else {
                        if b == b'a' {
                            dist[l][c] = 0;
                            todo.push(State {node: (l, c), distance: 0});
                        }
                        b - b'a'
                    }
                })
                .collect()
        })
        .collect();

    let dist = loop {
        let item = todo.pop().unwrap();
        let item_dist = dist[item.node.0][item.node.1];
        let item_height = height[item.node.0][item.node.1];
        if item.distance > item_dist {
            continue;
        }

        let new_distance = item_dist + 1;

        if item.node == end {
            break item_dist;
        }

        neighbours(item.node, nb_lines, nb_cols).iter()
            .filter(|n|height[(**n).0][(**n).1] <= item_height + 1)
            .for_each(|n| {
                let bro_dist = dist[(*n).0][(*n).1];
                if bro_dist > new_distance {
                    dist[(*n).0][(*n).1] = new_distance;
                    todo.push(State { node: *n, distance: new_distance})
                }
            });
    };

    println!("{dist}")
}

fn neighbours((l, c): (usize, usize), lines: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut bros = Vec::new();
    if 0 < l {
        bros.push((l - 1, c));
    }
    if l < lines - 1 {
        bros.push((l + 1, c));
    }
    if 0 < c {
        bros.push((l, c - 1))
    }
    if c < cols - 1 {
        bros.push((l, c + 1));
    }

    return bros;
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: (usize, usize),
    distance: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}