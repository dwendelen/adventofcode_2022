use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day07/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut root = Dir::new();
    let mut pwd: Vec<&str> = Vec::new();

    let mut i = 0;
    loop {
        if i >= lines.len() {
            break
        }

        let line = &lines[i];
        i += 1;

        if *line == "$ cd /" {
            pwd.clear();
        } else if *line == "$ cd .." {
            pwd.pop();
        } else if line.starts_with("$ cd ") {
            pwd.push(&line[5..]);
        } else if *line == "$ ls" {
            loop {
                if i >= lines.len() {
                    break
                }
                let line2 = &lines[i];
                i += 1;
                if line2.starts_with("$") {
                    i -= 1;
                    break;
                } else if line2.starts_with("dir ") {
                    let name = String::from(&line2[4..]);
                    resolve(&mut root, &pwd).dirs.insert(name, Dir::new());
                } else {
                    let idx = line2.find(" ").unwrap();
                    let size: u32 = line2[..idx].parse().unwrap();
                    let name = String::from(&line2[(idx + 1)..]);
                    resolve(&mut root, &pwd).fils.insert(name, Fil::new(size));
                }
            }
        } else {
            panic!()
        }
    }
    let total = size(&root);
    let to_clean_up = total - 40000000;
    let res = solve(&root, to_clean_up);
    println!("{res}")
}

fn resolve<'a>(root: &'a mut Dir, path: &Vec<&str>) -> &'a mut Dir {
    return path.iter()
        .fold(root, |d,n|d.dirs.get_mut(&String::from(*n)).unwrap())
}

fn size(dir: &Dir) -> u32 {
    let file_sum = dir.fils.iter().fold(0, |a, (_, f)| a + f.size);
    let dir_sum = dir.dirs.iter().fold(0, |a, (_, d)| a + size(d));
    return file_sum + dir_sum;
}

fn solve(dir: &Dir, space_needed: u32) -> u32 {
    let best = dir.dirs.iter().fold(u32::MAX, |a, (_, dir)| {
        let s = solve(dir, space_needed);
        if s < a {
            s
        } else {
            a
        }
    });

    let my_size = size(dir);
    return if my_size > space_needed && my_size < best {
        my_size
    } else {
        best
    }
}

struct Dir {
    fils: HashMap<String, Fil>,
    dirs: HashMap<String, Dir>
}

impl Dir {
    fn new() -> Dir {
        return Dir { fils: HashMap::new(), dirs: HashMap::new() }
    }
}

struct Fil {
    size: u32
}

impl Fil {
    fn new(size: u32) -> Fil {
        return Fil { size }
    }
}
