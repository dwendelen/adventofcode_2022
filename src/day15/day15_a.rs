use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Captures, Regex};

fn main() {
    let line = 2000000;
    let scanners = parse(
        "src/day15/input.txt",
        r"Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)",
        |cap| {
        return Scanner {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            beacon_x: cap[3].parse().unwrap(),
            beacon_y: cap[4].parse().unwrap()
        }
    });

    let mut root: Tree = Tree::Block(false);

    for scanner in &scanners {
        /*
                #
               ###    <- line
              ##S##
               ###
                B
        */
        let dist = (scanner.beacon_x - scanner.x).abs() + (scanner.beacon_y - scanner.y).abs();
        let half_x = dist - (scanner.y - line).abs();
        if half_x >= 0 {
            let mn = scanner.x - half_x;
            let mx = scanner.x + half_x + 1;
            let tree = Tree::range(mn, mx, true, false);
            root = combine(&root, &tree, or);
        }
    }

    for scanner in &scanners {
        if scanner.beacon_y == line {
            let tree = Tree::dot(scanner.beacon_x, false, true);
            root = combine(&root, &tree, and);
        }
        if scanner.y == line {
            let tree = Tree::dot(scanner.x, false, true);
            root = combine(&root, &tree, and);
        }
    }

    print(&root, (i32::MIN / 4), -(i32::MIN / 4));
    println!("{}", total(&root, -(i32::MIN / 2)));

}

fn or(a: bool, b: bool) -> bool {
    return a || b;
}

fn and(a: bool, b: bool) -> bool {
    return a && b;
}

fn total(tree: &Tree, size: i32) -> i32 {
    return match tree {
        Tree::Block(b) => if *b {
            size
        } else {
            0
        },
        Tree::Node(l, r) => total(l, size / 2) + total(r, size / 2)
    }
}

fn print(tree: &Tree, left: i32, right: i32) {
    let center = (right + left) / 2;
    return match tree {
        Tree::Block(b) => if *b {
            println!("{} -> {} = {}", left, right, right - left);
        },
        Tree::Node(l, r) => {
            print(l, left, center);
            print(r, center, right);
        }
    }
}

struct Scanner {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32
}

#[derive(Clone)]
enum Tree {
    Node(Box<Tree>, Box<Tree>),
    Block(bool)
}

impl Tree {
    fn simplify(&self) -> Tree {
        match self {
            Tree::Node(t1, t2) =>
                match(t1.as_ref(), t2.as_ref()) {
                    (Tree::Block(b1), Tree::Block(b2)) =>
                        if *b1 == *b2 {
                            Tree::Block(*b1)
                        } else {
                            self.clone()
                        }
                    _ => self.clone()
                }
            _ => self.clone()
        }
    }
}

impl Tree {
    fn range(start: i32, end: i32, val_in: bool, val_out: bool) -> Tree {
        return Tree::range2((i32::MIN / 4), start, end, -(i32::MIN / 4), val_in, val_out)
    }
    fn range2(left: i32, start: i32, end: i32, right: i32, val_in: bool, val_out: bool) -> Tree {
        return if start <= left && right <= end {
            Tree::Block(val_in)
        } else if end <= left || right <= start {
            Tree::Block(val_out)
        } else {
            let center = (right + left) / 2;
            let l = Tree::range2(left, start, end, center, val_in, val_out);
            let r = Tree::range2(center, start, end, right, val_in, val_out);
            Tree::Node(Box::new(l), Box::new(r))
        }
    }
    fn dot(x: i32, val_in: bool, val_out: bool) -> Tree {
        return Tree::range(x, x + 1, val_in, val_out)
    }
}

fn combine(tree1: &Tree, tree2: &Tree, combiner: fn (bool, bool) -> bool) -> Tree {
    match(tree1, tree2) {
        (Tree::Block(v1), Tree::Block(v2)) =>
            Tree::Block(combiner(*v1, *v2)),
        (Tree::Block(_), Tree::Node(l, r)) =>
            Tree::Node(Box::new(combine(tree1, l, combiner)), Box::new(combine(tree1, r, combiner))).simplify(),
        (Tree::Node(l, r), Tree::Block(_)) =>
            Tree::Node(Box::new(combine(l, tree2, combiner)), Box::new(combine(r, tree2, combiner))).simplify(),
        (Tree::Node(l1, r1), Tree::Node(l2, r2)) =>
            Tree::Node(Box::new(combine(l1, l2, combiner)), Box::new(combine(r1, r2, combiner))).simplify()
    }
}

fn parse<T, F: Fn(&Captures) -> T>(file: &str, regex: &str, factory: F) -> Vec<T> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let re = Regex::new(regex).unwrap();
    lines.iter()
        .map(|l| {
            let captures = re.captures(l.as_str()).unwrap();
            factory(&captures)
        })
        .collect()
}