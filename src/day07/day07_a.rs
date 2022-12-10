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
    let (_, res) = count(&root);
    println!("{res}")
}

fn resolve<'a>(root: &'a mut Dir, path: &Vec<&str>) -> &'a mut Dir {
    return path.iter()
        .fold(root, |d,n|d.dirs.get_mut(&String::from(*n)).unwrap())
}

fn count(dir: &Dir) -> (u32, u32) {
    let file_sum = dir.fils.iter().fold(0, |a, (_, f)| a + f.size);
    let (dir_sum, acc_sum) = dir.dirs.iter().fold((0, 0), |(size, acc), (_, dir)|{
        let (ds, da) = count(dir);
        (size + ds, acc + da)
    });
    let size = dir_sum + file_sum;
    return if size < 100000 {
        (size, acc_sum + size)
    } else {
        (size, acc_sum)
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
