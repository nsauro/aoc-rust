use std::collections::HashMap;
use std::fs;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let raw = fs::read_to_string("src/bin/day_09/input.data").unwrap();

    let distance = Regex::new(r"^([a-zA-Z]+) to ([a-zA-Z]+) = (\d+)$").unwrap();

    let mut destinations : Vec<&str> =  Vec::new();
    let mut distances : HashMap<String, u32> = HashMap::new();

    raw.lines().for_each(|l| {
        if let Some(c) = distance.captures(l){
            let f = c.get(1).unwrap().as_str();
            let s = c.get(2).unwrap().as_str();
            let  d = c.get(3).unwrap().as_str().parse::<u32>().unwrap();
            if !destinations.contains(&f) {
                destinations.push(f);
            }
            if !destinations.contains(&s){
                destinations.push(s);
            }
            distances.insert(create_key(f, s), d);

        }
    });

    let all_distances : Vec<u32> = destinations.iter().permutations(destinations.len()).map( |c| {
      c.windows(2).fold(0, |acc, ps| {
          let key = create_key(&ps.first().unwrap().trim(), &ps.last().unwrap().trim().trim());
          acc + distances.get(&key).unwrap()
      })
    }
    ).collect();

    println!("part 1: {}", all_distances.iter().min().unwrap());
    println!("part 2: {}", all_distances.iter().max().unwrap())

}

fn create_key(f : &str, s: &str) -> String {
    if f.le(&s) {
        String::from(f) + s

    }else{
        String::from(s) + f
    }
}