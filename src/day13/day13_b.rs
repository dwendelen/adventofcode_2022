extern crate core;

use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Node::{Leaf, Lst};

fn main() {
    let file = File::open("src/day13/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut nodes: Vec<Node> = lines.iter()
        .filter(|l|**l != "")
        .map(|l|map_line(l))
        .collect();

    let start = map_line(&String::from("[[2]]"));
    let stop = map_line(&String::from("[[6]]"));
    nodes.push(start.clone());
    nodes.push(stop.clone());

    nodes.sort();

    let strt = nodes.iter().position(|i|i == &start).unwrap();
    let stp = nodes.iter().position(|i|i == &stop).unwrap();

    let res = (strt + 1) * (stp + 1);

    println!("{res}")
}

fn map_chunk(lines: &[String]) -> (Node, Node) {
    return (map_line(&lines[0]), map_line(&lines[1]))
}

fn map_line(line: &String) -> Node {
    return parse(line.as_str()).0
}

fn parse(stream: &str) -> (Node, &str) {
    return match &stream[0..1] {
        "[" => parse_lst(stream),
        _ => parse_leaf(stream)
    }
}

fn parse_lst(stream: &str) -> (Node, &str) {
    let mut strm = &stream[1..];
    let mut acc = Vec::new();
    loop {
        if &strm[0..1] == "]" {
            return (Lst(acc), &strm[1..])
        } else {
            let (n, strmm) = parse(strm);
            strm = strmm;
            acc.push(n);
            if &strm[0..1] == "," {
                strm = &strm[1..];
            }
        }
    }
}

fn parse_leaf(stream: &str) -> (Node, &str) {
    let mut idx = 0;
    loop {
        if &stream[idx..idx+1] == "," || &stream[idx..idx+1] == "]" {
            let val: i32 = stream[..idx].parse().unwrap();
            return (Leaf(val), &stream[idx..])
        }
        idx += 1;
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Node {
    Lst(Vec<Node>),
    Leaf(i32)
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        return match (self, other) {
            (Leaf(l1), Leaf(l2)) => l1.cmp(&l2),
            (Lst(_), Leaf(_)) => self.cmp(&Lst(Vec::from([other.clone()]))),
            (Leaf(_), Lst(_)) => Lst(Vec::from([self.clone()])).cmp(other),
            (Lst(l1), Lst(l2)) => {
                let mut i = 0;
                loop {
                    if i >= l1.len() || i >= l2.len() {
                        break;
                    }
                    let c = l1[i].cmp(&l2[i]);
                    if c != Ordering::Equal {
                        return c;
                    }
                    i += 1;
                }
                return l1.len().cmp(&l2.len())
            }
        }
    }
}