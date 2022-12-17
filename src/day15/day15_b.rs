use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Captures, Regex};

fn main() {
    // let search_max = 20;
    // let grid_space = 32;
    let search_max = 4000000;
    let grid_space = 4194304;
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

    let mut grid: Tree = Tree::Block(true);

    //TODO transform space

    for scanner in &scanners {
        let dist = (scanner.beacon_x - scanner.x).abs() + (scanner.beacon_y - scanner.y).abs();
        println!("{}", dist);

        grid = apply(scanner.x, scanner.y, dist, &grid, 0, grid_space, 0, grid_space);
        // print(&grid, grid_space);
    }

    let find = find(&grid, 0, grid_space, 0, grid_space, search_max).unwrap_or((-1, -1,-1,-1));
    if find.1 - find.0 != 1 || find.3 - find.2 != 1 {
        panic!()
    }
    let res = 4000000* find.0 as u64 + find.2 as u64;
    println!("l {} r {} b {} t {} => {}", find.0, find.1, find.2, find.3, res);
}

fn apply(x:i32, y:i32, dist: i32, tree: &Tree, grid_l: i32, grid_r: i32, grid_b: i32, grid_t: i32) -> Tree {
    if grid_l == grid_r || grid_b == grid_t {
        panic!()
    }
    match tree {
        Tree::Block(false) => return Tree::Block(false),
        _ => {}
    }
    let bl = (grid_l - x).abs() + (grid_b - y).abs();
    let br = (grid_r - 1 - x).abs() + (grid_b - y).abs();
    let tl = (grid_l - x).abs() + (grid_t - 1 - y).abs();
    let tr = (grid_r - 1 - x).abs() + (grid_t - 1 - y).abs();
    if bl <= dist && br <= dist && tl <= dist && tr <= dist {
        // inside the diamond
        Tree::Block(false)
    } else if some_overlap(x, y, dist, grid_l, grid_r, grid_b, grid_t) {
        // overlaps the diamond
        let center_x = (grid_l + grid_r) / 2;
        let center_y = (grid_b + grid_t) / 2;
        simplify(match(tree) {
            Tree::Block(false) => Tree::Block(false),
            Tree::Block(true) => Tree::Node(
                Box::new(apply(x, y, dist, tree, grid_l, center_x, grid_b, center_y)),
                Box::new(apply(x, y, dist, tree, center_x, grid_r, grid_b, center_y)),
                Box::new(apply(x, y, dist, tree, grid_l, center_x, center_y, grid_t)),
                Box::new(apply(x, y, dist, tree, center_x, grid_r, center_y, grid_t)),
            ),
            Tree::Node(bl, br, tl, tr) => Tree::Node(
                Box::new(apply(x, y, dist, bl, grid_l, center_x, grid_b, center_y)),
                Box::new(apply(x, y, dist, br, center_x, grid_r, grid_b, center_y)),
                Box::new(apply(x, y, dist, tl, grid_l, center_x, center_y, grid_t)),
                Box::new(apply(x, y, dist, tr, center_x, grid_r, center_y, grid_t)),
            )
        })
    } else {
        // outside the diamond
        tree.clone()
    }
}

fn some_overlap(x:i32, y:i32, dist: i32, grid_l: i32, grid_r: i32, grid_b: i32, grid_t: i32) -> bool {
    if grid_l <= x - dist && x + dist < grid_r && grid_b <= y - dist && y + dist < grid_r {
        // is inside us
        return true
    }
    // left
    let left_half_y = dist - (grid_l - x).abs();
    let left_overlap = if left_half_y >= 0 {
        let mn = y - left_half_y;
        let mx = y + left_half_y;
        grid_b <= mx && mn <= (grid_t - 1)
    } else {
        false
    };
    // right
    let right_half_y = dist - (grid_r - 1 - x).abs();
    let right_overlap = if right_half_y >= 0 {
        let mn = y - right_half_y;
        let mx = y + right_half_y;
        grid_b <= mx && mn <= (grid_t - 1)
    } else {
        false
    };
    // bottom
    let bottom_half_x = dist - (grid_b - y).abs();
    let bottom_overlap = if bottom_half_x >= 0 {
        let mn = x - bottom_half_x;
        let mx = x + bottom_half_x;
        grid_l <= mx && mn <= (grid_r - 1)
    } else {
        false
    };
    // top
    let top_half_x = dist - (grid_t - 1 - y).abs();
    let top_overlap = if top_half_x >= 0 {
        let mn = x - top_half_x;
        let mx = x + top_half_x;
        grid_l <= mx && mn <= (grid_r - 1)
    } else {
        false
    };
    return left_overlap || right_overlap || top_overlap || bottom_overlap
}

fn find(tree: &Tree, grid_l: i32, grid_r: i32, grid_b: i32, grid_t: i32, search_max: i32) -> Option<(i32, i32, i32, i32)> {
    if grid_l >= search_max || grid_b >= search_max {
        return None
    }
    return match tree {
        Tree::Block(true) => Some((grid_l, grid_r, grid_b, grid_t)),
        Tree::Block(false) => None,
        Tree::Node(bl, br, tl, tr) => {
            let center_x = (grid_l + grid_r) / 2;
            let center_y = (grid_b + grid_t) / 2;
            let f_bl = find(bl, grid_l, center_x, grid_b, center_y, search_max);
            let f_br = find(br, center_x, grid_r, grid_b, center_y, search_max);
            let f_tl = find(tl, grid_l, center_x, center_y, grid_t, search_max);
            let f_tr = find(tr, center_x, grid_r, center_y, grid_t, search_max);
            f_bl.or(f_br).or(f_tl).or(f_tr)
        }
    }
}

fn print(tree: &Tree, search_space: i32) {
    for y in 0..search_space {
        for x in 0..search_space {
            if(get_val(tree, 0, search_space, 0, search_space, x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}

fn get_val(tree: &Tree, grid_l: i32, grid_r: i32, grid_b: i32, grid_t: i32, x: i32, y: i32) -> bool {
    if x < grid_l || x >= grid_r || y < grid_b || y >= grid_t  {
        panic!()
    }
    return match tree {
        Tree::Block(b) => *b,
        Tree::Node(bl, br, tl, tr) => {
            let center_x = (grid_l + grid_r) / 2;
            let center_y = (grid_b + grid_t) / 2;
            if y < center_y {
                if x < center_x {
                    get_val(bl, grid_l, center_x, grid_b, center_y, x, y)
                } else {
                    get_val(br, center_x, grid_r, grid_b, center_y, x, y)
                }
            } else {
                if x < center_x {
                    get_val(tl, grid_l, center_x, center_y, grid_t, x, y)
                } else {
                    get_val(tr, center_x, grid_r, center_y, grid_t, x, y)
                }
            }
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
    // 3 4
    // 1 2
    //   (-x, -y)   ( x, -y)   (-x,  y)   ( x,  y)
    Node(Box<Tree>, Box<Tree>, Box<Tree>, Box<Tree>),
    Block(bool)
}

fn simplify(tree: Tree) -> Tree {
    match tree {
        Tree::Node(bl, br, tl, tr) =>
            match(bl.as_ref(), br.as_ref(), tl.as_ref(), tr.as_ref()) {
                (Tree::Block(b1), Tree::Block(b2), Tree::Block(b3), Tree::Block(b4)) =>
                    if *b1 == *b2 && *b1 == *b3 && *b1 == *b4{
                        Tree::Block(*b1)
                    } else {
                        Tree::Node(bl, br, tl, tr)
                    }
                _ => Tree::Node(bl, br, tl, tr)
            }
        _ => tree
    }
}

impl Tree {
    fn range(range_l: i32, range_r: i32, range_b: i32, range_t: i32, val_in: bool, val_out: bool) -> Tree {
        return Tree::range2(range_l, range_r, range_b, range_t, (i32::MIN / 4), -(i32::MIN / 4), (i32::MIN / 4), -(i32::MIN / 4), val_in, val_out)
    }
    fn range2(range_l: i32, range_r: i32, range_b: i32, range_t: i32, grid_l: i32, grid_r: i32, grid_b: i32, grid_t: i32, val_in: bool, val_out: bool) -> Tree {
        return if range_l <= grid_l && grid_r <= range_r && range_b <= grid_b && grid_t <= range_t {
            Tree::Block(val_in)
        } else if range_r <= grid_l || grid_r <= range_l || range_t <= grid_b || grid_t <= range_b {
            Tree::Block(val_out)
        } else {
            let center_x = (grid_l + grid_r) / 2;
            let center_y = (grid_b + grid_t) / 2;
            let bl = Tree::range2(range_l, range_r, range_b, range_t, grid_l, center_x, grid_b, center_y, val_in, val_out);
            let br = Tree::range2(range_l, range_r, range_b, range_t, center_x, grid_r, grid_b, center_y, val_in, val_out);
            let tl = Tree::range2(range_l, range_r, range_b, range_t, grid_l, center_x, center_y, grid_t, val_in, val_out);
            let tr = Tree::range2(range_l, range_r, range_b, range_t, center_x, grid_r, center_y, grid_t, val_in, val_out);
            Tree::Node(Box::new(bl), Box::new(br),Box::new(tl), Box::new(tr))
        }
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