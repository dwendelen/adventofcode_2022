use std::cmp::min;
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
        persons: [Person{tgt: start, time: 0}, Person{tgt: start, time: 0}],
        valve_open: (0..processed.len()).map(|_|false).collect(),
        time_left: 26,
        flow_acc: 0,
    };
    initial_state.valve_open[start] = true;
    let res = best_score(&processed, initial_state);
    println!("{}", res);
}

fn best_score(valves: &Vec<ProcessedValve>, state: State) -> i32 {
    if state.time_left <= 0 {
        return state.flow_acc;
    }

    let mut best = -1;

    let current_person = if state.persons[0].time == 0 {
        0
    } else {
        1
    };

    let my_person = &state.persons[current_person];
    let my_valve = &valves[my_person.tgt];
    let acc = state.flow_acc + state.time_left * my_valve.flow;
    for next_tgt in 0..state.valve_open.len() {
        if state.valve_open[next_tgt] {
            continue;
        }
        let mut new_valve_open = state.valve_open.clone();
        new_valve_open[next_tgt] = true;
        let mut new_persons = state.persons.clone();
        new_persons[current_person].tgt = next_tgt;
        new_persons[current_person].time = my_valve.distances[next_tgt] + 1;
        let time_progression = min(new_persons[0].time, new_persons[1].time);
        new_persons[0].time -= time_progression;
        new_persons[1].time -= time_progression;
        let new_state = State {
            persons: new_persons,
            valve_open: new_valve_open,
            time_left: state.time_left - time_progression,
            flow_acc: acc,
        };
        let score = best_score(valves, new_state);
        if score > best {
            best = score;
        }
    }
    if best == -1 {
        let other_person = 1 - current_person;
        if(state.persons[other_person].time != 999) {
            let mut new_persons = state.persons.clone();
            new_persons[current_person].tgt = 0;
            new_persons[current_person].time = 999;
            let time_progression = min(new_persons[0].time, new_persons[1].time);
            new_persons[0].time -= time_progression;
            new_persons[1].time -= time_progression;
            let new_state = State {
                persons: new_persons,
                valve_open: state.valve_open,
                time_left: state.time_left - time_progression,
                flow_acc: acc,
            };
            return best_score(valves, new_state);
        } else {
            return acc
        }

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
    persons: [Person; 2],
    valve_open: Vec<bool>,
    time_left: i32,
    flow_acc: i32
}

#[derive(Clone)]
struct Person {
    tgt: usize,
    time: i32
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