use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

static SET_VALUE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+) -> ([a-z]+)$").unwrap());
static COPY_VALUE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([a-z]+) -> ([a-z]+)$").unwrap());
static AND: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([a-z]+) AND ([a-z]+) -> ([a-z]+)$").unwrap());
static AND_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"^1 AND ([a-z]+) -> ([a-z]+)$").unwrap());
static OR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([a-z]+) OR ([a-z]+) -> ([a-z]+)$").unwrap());
static LSHIFT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-z]+) LSHIFT (\d+) -> ([a-z]+)$").unwrap());
static RSHIFT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(^[a-z]+) RSHIFT (\d+) -> ([a-z]+)$").unwrap());
static NOT: Lazy<Regex> = Lazy::new(|| Regex::new(r"NOT ([a-z]+) -> ([a-z]+)").unwrap());
fn main() {
    let raw = fs::read_to_string("src/bin/day_07/input.data").unwrap();
    //note: data here is updated for part 2 with the result of part 1! recopy from source

    let input_1 = raw.lines();

    let mut map_1: HashMap<String, u32> = HashMap::new();
    process(&mut map_1, input_1.clone().map(|i| i.to_string()).collect());

    println!("part1: {}", map_1.get("a").unwrap());
    println!("part1: {}", map_1.get("b").unwrap());
}

fn process(registers: &mut HashMap<String, u32>, ins: Vec<String>) {
    let mut next: Vec<String> = Vec::new();
    println!("processing {} instructions", ins.len());
    ins.iter().for_each(|i| {
        if let Some(c) = SET_VALUE.captures(i) {
            let key = c.get(2).unwrap().as_str().to_string();
            let value = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
            registers.insert(key, value);
        } else if let Some(c) = COPY_VALUE.captures(i) {
            let s = c.get(1).unwrap().as_str();
            let d = c.get(2).unwrap().as_str();
            if let Some(v) = registers.get(s) {
                registers.insert(d.to_string(), *v);
            } else {
                next.push(i.to_string());
            }
        } else if let Some(c) = AND.captures(i) {
            let f = c.get(1).unwrap().as_str();
            let s = c.get(2).unwrap().as_str();
            let d = c.get(3).unwrap().as_str();
            if let (Some(v1), Some(v2)) = (registers.get(f), registers.get(s)) {
                registers.insert(d.to_string(), v1 & v2);
            } else {
                next.push(i.to_string());
            }
        } else if let Some(c) = AND_1.captures(i) {
            let s = c.get(1).unwrap().as_str();
            let d = c.get(2).unwrap().as_str();
            if let Some(v) = registers.get(s) {
                registers.insert(d.to_string(), 1 & v);
            } else {
                next.push(i.to_string());
            }
        } else if let Some(c) = OR.captures(i) {
            let f = c.get(1).unwrap().as_str();
            let s = c.get(2).unwrap().as_str();
            let d = c.get(3).unwrap().as_str();
            if let (Some(v1), Some(v2)) = (registers.get(f), registers.get(s)) {
                registers.insert(d.to_string(), v1 | v2);
            } else {
                next.push(i.to_string());
            }
        } else if let Some(c) = LSHIFT.captures(i) {
            let s = c.get(1).unwrap().as_str();
            let amount = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let d = c.get(3).unwrap().as_str();
            if let Some(v) = registers.get(s) {
                registers.insert(d.to_string(), v << amount);
            } else {
                next.push(i.to_string());
            }
        } else if let Some(c) = RSHIFT.captures(i) {
            let s = c.get(1).unwrap().as_str();
            let amount = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let d = c.get(3).unwrap().as_str();
            if let Some(v) = registers.get(s) {
                registers.insert(d.to_string(), v >> amount);
            } else {
                next.push(i.to_string());
            }
        } else if let Some(c) = NOT.captures(i) {
            let s = c.get(1).unwrap().as_str();
            let d = c.get(2).unwrap().as_str();
            if let Some(v) = registers.get(s) {
                registers.insert(d.to_string(), !v);
            } else {
                next.push(i.to_string());
            }
        } else {
            panic!("omg!!!! {}", i)
        }
    });
    if !next.is_empty() {
        process(registers, next);
    }
}
