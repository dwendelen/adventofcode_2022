use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day08/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let trees: Vec<Vec<Tree>> = lines.iter()
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

    let mut best = 0;
    for y in 0..height {
        for x in 0..width {
            let my_height = trees[y][x].height;
            let mut score = 1;

            // up
            let mut dist = 0;
            for yy in 0..y {
                let tree_height = trees[y - yy - 1][x].height;
                dist += 1;
                if tree_height >= my_height {
                    break;
                }
            }
            score *= dist;

            // down
            dist = 0;
            for yy in (y + 1)..height {
                let tree_height = trees[yy][x].height;
                dist += 1;
                if tree_height >= my_height {
                    break;
                }
            }
            score *= dist;

            // left
            dist = 0;
            for xx in 0..x {
                let tree_height = trees[y][x - xx - 1].height;
                dist += 1;
                if tree_height >= my_height {
                    break;
                }
            }
            score *= dist;

            // right
            dist = 0;
            for xx in (x + 1)..width {
                let tree_height = trees[y][xx].height;
                dist += 1;
                if tree_height >= my_height {
                    break;
                }
            }
             score *= dist;

            if score > best {
                best = score
            }
        }
    }

    println!("{best}")
}

struct Tree {
    height: i8
}

impl Tree {
    fn new(height: i8) -> Tree {
        return Tree { height }
    }
}