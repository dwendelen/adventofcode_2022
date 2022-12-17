use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Captures, Regex};

fn main() {
    let valves = parse(
        "src/day16/input.txt",
        r"Valve (\w*) has flow rate=(\d*); tunnels? leads? to valves? (\w*(, \w*)*)",
        |cap| {
        return ParsedValve {
            name: String::from(&cap[1]),
            flow: cap[2].parse().unwrap(),
            neighbours: cap[3].split(", ").map(String::from).collect()
        }
    });
    let mut start: usize = 9999;
    let mut processed: Vec<ProcessedValve> = Vec::new();
    for i in 0..valves.len() {
        let valve = &valves[i];
        if valve.flow == 0 && valve.name != "AA" {
            continue
        }
        if valve.name == "AA" {
            start = processed.len()
        }
        processed.push(ProcessedValve{name: valve.name.clone(), flow: valve.flow, distances: Vec::new()})
    }
    // Update distances
    for i in 0..processed.len() {
        let name = &processed[i].name;
        let mut dist = HashMap::<String, i32>::new();
        dist.insert(name.clone(), 0);
        let mut todo = LinkedList::<String>::new();
        todo.push_back(name.clone());
        while todo.len() != 0 {
            let td = todo.pop_front().unwrap();
            let d = *dist.get(&td).unwrap();
            let friends = &valves.iter().find(|v|v.name == td).unwrap().neighbours;
            for friend in friends {
                let fd = *dist.get(friend).unwrap_or(&i32::MAX);
                if fd > d + 1 {
                    dist.insert(friend.clone(), d + 1);
                    todo.push_back(friend.clone());
                }
            }
        }
        let dists: Vec<i32> = (0..processed.len())
            .map(|i|*dist.get(&processed[i].name).unwrap())
            .collect();
        let toupdate = &mut processed[i];
        toupdate.distances = dists;
    }

    let mut initial_state = State{
        pos: start,
        valve_open: (0..processed.len()).map(|_|false).collect(),
        time_left: 30,
        flow_acc: 0,
    };
    initial_state.valve_open[initial_state.pos] = true;
    let res = best_score(&processed, initial_state);
    println!("{}", res);
}

fn best_score(valves: &Vec<ProcessedValve>, state: State) -> i32 {
    if state.time_left <= 0 {
        return state.flow_acc;
    }
    let my_valve = &valves[state.pos];
    let acc = state.flow_acc + state.time_left * my_valve.flow;
    let mut best = -1;
    for i in 0..state.valve_open.len() {
        if state.valve_open[i] {
            continue;
        }
        let mut new_valve_open = state.valve_open.clone();
        new_valve_open[i] = true;
        let new_state = State {
            pos: i,
            valve_open: new_valve_open,
            time_left: state.time_left - my_valve.distances[i] - 1,
            flow_acc: acc,
        };
        let score = best_score(valves, new_state);
        if score > best {
            best = score;
        }
    }
    if best == -1 {
        return acc
    }
    return best;
}

struct ParsedValve {
    name: String,
    flow: i32,
    neighbours: Vec<String>
}

struct ProcessedValve {
    name: String,
    flow: i32,
    distances: Vec<i32>
}

#[derive(Clone)]
struct State {
    pos: usize,
    valve_open: Vec<bool>,
    time_left: i32,
    flow_acc: i32
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