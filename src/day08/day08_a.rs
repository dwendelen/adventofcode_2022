use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day08/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut trees: Vec<Vec<Tree>> = lines.iter()
        .map(|l|
            l.chars()
                .map(|c|
                    Tree::new(c.to_string().parse().unwrap())
                )
                .collect()
        )
        .collect();

    let height = trees.len();
    let width = trees.first().unwrap().len();

    for y in 0..height {
        let mut tallest = -1;
        // left to right
        for x in 0..width {
            let tree = &mut trees[y][x];
            if tree.height > tallest {
                tree.visible = true;
                tallest = tree.height;
            }
        }
        // right to left
        tallest = -1;
        for x in 0..width {
            let tree = &mut trees[y][width - x - 1];
            if tree.height > tallest {
                tree.visible = true;
                tallest = tree.height;
            }
        }
    }
    for x in 0..width {
        let mut tallest = -1;
        // top to bottom
        for y in 0..height {
            let tree = &mut trees[y][x];
            if tree.height > tallest {
                tree.visible = true;
                tallest = tree.height;
            }
        }
        // bottom to top
        tallest = -1;
        for y in 0..height {
            let tree = &mut trees[height - y - 1][x];
            if tree.height > tallest {
                tree.visible = true;
                tallest = tree.height;
            }
        }
    }

    let res = trees.iter()
        .fold(0, |a, ts| a + ts.iter()
            .fold(0, |a,t| a + if t.visible { 1 } else { 0 })
        );
    println!("{res}")
}

struct Tree {
    height: i8,
    visible: bool
}

impl Tree {
    fn new(height: i8) -> Tree {
        return Tree { height, visible: false }
    }
}