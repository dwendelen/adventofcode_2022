use std::cmp::max;
use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Captures, Regex};

fn main() {
    let blue_prints = parse(
        "src/day19/input.txt",
        r"Blueprint (\d*): Each ore robot costs (\d*) ore. Each clay robot costs (\d*) ore. Each obsidian robot costs (\d*) ore and (\d*) clay. Each geode robot costs (\d*) ore and (\d*) obsidian.",
        |cap| {
            return BluePrint {
                number: cap[1].parse().unwrap(),
                ore_ore: cap[2].parse().unwrap(),
                clay_ore: cap[3].parse().unwrap(),
                obsidian_ore: cap[4].parse().unwrap(),
                obsidian_clay: cap[5].parse().unwrap(),
                geode_ore: cap[6].parse().unwrap(),
                geode_obsidian: cap[7].parse().unwrap(),
            };
        });
    let res: i32 = blue_prints.iter()
        .map(|b| {
            let best = solve(b);
            println!("{}: {}", b.number, best);
            best * b.number
        })
        .sum();
    println!("{}", res)
}

fn solve(blueprint: &BluePrint) -> i32 {
    let initial_state = State {
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        time_left: 24,
    };
    return find_best(blueprint, initial_state);
}

fn find_best(blueprint: &BluePrint, state: State) -> i32 {
    if state.time_left == 0 {
        return state.geode;
    } else if state.time_left < 0 {
        return 0;
    }

    let time_ore = max(
        div_ceil(blueprint.ore_ore - state.ore, state.ore_bots),
        0
    );
    let state_ore = State {
        ore_bots: state.ore_bots + 1,
        clay_bots: state.clay_bots,
        obsidian_bots: state.obsidian_bots,
        geode_bots: state.geode_bots,
        ore: state.ore + (time_ore + 1) * state.ore_bots - blueprint.ore_ore,
        clay: state.clay + (time_ore + 1) * state.clay_bots,
        obsidian: state.obsidian + (time_ore + 1) * state.obsidian_bots,
        geode: state.geode + (time_ore + 1) * state.geode_bots,
        time_left: state.time_left - (time_ore + 1),
    };
    let best_ore = find_best(blueprint, state_ore);

    let time_clay = max(
        div_ceil(blueprint.clay_ore - state.ore, state.ore_bots),
        0
    );
    let state_clay = State {
        ore_bots: state.ore_bots,
        clay_bots: state.clay_bots + 1,
        obsidian_bots: state.obsidian_bots,
        geode_bots: state.geode_bots,
        ore: state.ore + (time_clay + 1) * state.ore_bots - blueprint.clay_ore,
        clay: state.clay + (time_clay + 1) * state.clay_bots,
        obsidian: state.obsidian + (time_clay + 1) * state.obsidian_bots,
        geode: state.geode + (time_clay + 1) * state.geode_bots,
        time_left: state.time_left - (time_clay + 1),
    };
    let best_clay = find_best(blueprint, state_clay);

    let time_obsidian = max(max(
        div_ceil(blueprint.obsidian_ore - state.ore, state.ore_bots),
        div_ceil(blueprint.obsidian_clay - state.clay, state.clay_bots)),
        0
    );
    let state_obsidian = State {
        ore_bots: state.ore_bots,
        clay_bots: state.clay_bots,
        obsidian_bots: state.obsidian_bots + 1,
        geode_bots: state.geode_bots,
        ore: state.ore + (time_obsidian + 1) * state.ore_bots - blueprint.obsidian_ore,
        clay: state.clay + (time_obsidian + 1) * state.clay_bots - blueprint.obsidian_clay,
        obsidian: state.obsidian + (time_obsidian + 1) * state.obsidian_bots,
        geode: state.geode + (time_obsidian + 1) * state.geode_bots,
        time_left: state.time_left - (time_obsidian + 1),
    };
    let best_obsidian = find_best(blueprint, state_obsidian);

    let time_geode = max(max(
        div_ceil(blueprint.geode_ore - state.ore, state.ore_bots),
        div_ceil(blueprint.geode_obsidian - state.obsidian, state.obsidian_bots)),
        0
    );
    let state_geode = State {
        ore_bots: state.ore_bots,
        clay_bots: state.clay_bots,
        obsidian_bots: state.obsidian_bots,
        geode_bots: state.geode_bots + 1,
        ore: state.ore + (time_geode + 1) * state.ore_bots - blueprint.geode_ore,
        clay: state.clay + (time_geode + 1) * state.clay_bots ,
        obsidian: state.obsidian + (time_geode + 1) * state.obsidian_bots - blueprint.geode_obsidian,
        geode: state.geode + (time_geode + 1) * state.geode_bots,
        time_left: state.time_left - (time_geode + 1),
    };
    let best_geode = find_best(blueprint, state_geode);

    let time_nothing = state.time_left;
    let state_geode = State {
        ore_bots: state.ore_bots,
        clay_bots: state.clay_bots,
        obsidian_bots: state.obsidian_bots,
        geode_bots: state.geode_bots,
        ore: state.ore + time_nothing * state.ore_bots,
        clay: state.clay + time_nothing * state.clay_bots,
        obsidian: state.obsidian + time_nothing * state.obsidian_bots,
        geode: state.geode + time_nothing * state.geode_bots,
        time_left: state.time_left - time_nothing
    };
    let best_nothing = find_best(blueprint, state_geode);

    return max(max(max(best_ore, best_clay), max(best_obsidian, best_geode)), best_nothing);
}

struct BluePrint {
    number: i32,
    ore_ore: i32,
    clay_ore: i32,
    obsidian_ore: i32,
    obsidian_clay: i32,
    geode_ore: i32,
    geode_obsidian: i32,
}

struct State {
    ore_bots: i32,
    clay_bots: i32,
    obsidian_bots: i32,
    geode_bots: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    time_left: i32,
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

fn div_ceil(a: i32, b: i32) -> i32 {
    if b == 0 {
        return i32::MAX / 500
    }
    return (a + b - 1) / b
}