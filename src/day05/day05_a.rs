use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Captures, Regex};

fn main() {
    let file = File::open("src/day05/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let pieces: Vec<&[String]> = lines.split(|l| l == "").collect();
    let init: &[String] = &pieces[0];
    let steps: &[String] = &pieces[1];

    let width = init[init.len() - 1].len();
    let num_stacks = (width + 2) / 4;
    let max_stack = init.len() - 1;

    let initial_stacks: Vec<Vec<char>> = (0..num_stacks)
        .map(|s|{
            (0..max_stack).map(|i|{
                let line: &String = &lines[max_stack - i - 1];
                let c: char = line.chars().nth(s * 4 + 1).unwrap_or(' ');
                c
            })
                .filter(|c|*c != ' ')
                .collect()
        }).collect();

    let instructions = parse(steps, r"move (\d*) from (\d*) to (\d*)", |cap|{
        Instruction{
            from: cap[2].parse().unwrap(),
            to: cap[3].parse().unwrap(),
            amount: cap[1].parse().unwrap(),
        }
    });
    let end_stack: Vec<Vec<char>> = instructions.iter()
        .fold(initial_stacks, |acc, i|i.apply(acc));

    let res: String = end_stack.iter()
        .fold("".to_string(), |acc, s|acc + &*s.last().unwrap().to_string());

    println!("{res}");
}

fn parse<T, F: Fn(&Captures) -> T>(lines: &[String], regex: &str, factory: F) -> Vec<T> {
    let re = Regex::new(regex).unwrap();
    lines.iter()
        .map(|l| {
            let captures = re.captures(l.as_str()).unwrap();
            factory(&captures)
        })
        .collect()
}

struct Instruction {
    from: u32,
    to: u32,
    amount: u32
}

impl Instruction {
    fn apply(&self, mut stack: Vec<Vec<char>>) -> Vec<Vec<char>> {
        for _ in 0..self.amount {
            let from_stack = &mut stack[self.from as usize - 1];
            let c = from_stack.pop().unwrap();
            let to_stack = &mut stack[self.to as usize - 1];
            to_stack.push(c);
        }
        return stack
    }
}