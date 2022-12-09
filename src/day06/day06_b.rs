use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/day06/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let line = &lines[0];
    let window = line.as_bytes();
    let vec: Vec<(usize, u8)> = window.iter().enumerate().map(|(i, e)|(i,*e)).collect();
    let (i, _) = vec.windows(14)
        .find(|wnd|{
            is_unique(wnd.iter().map(|(i, e)|*e).collect())
        })
        .map(|wnd|{
            wnd.first().unwrap()
        })
        .unwrap();
    let res = i + 14;
    println!("{res}")
}

fn is_unique(elems: Vec<u8>) -> bool {
    for i in 0..elems.len() {
        let x = elems[i];
        for j in i + 1 .. elems.len() {
            let y = elems[j];
            if x == y {
                return false
            }
        }
    }
    return true
}