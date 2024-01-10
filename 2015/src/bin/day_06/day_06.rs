use std::fs;
use regex::Regex;
use std::fmt;
use std::fmt::Formatter;
use aoc_2015::time_execution;

fn main() {
    let raw = fs::read_to_string("src/bin/day_06/input.data").unwrap();

    let turn_on: Regex = Regex::new(r"turn on (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let turn_off: Regex = Regex::new(r"turn off (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let toggle: Regex = Regex::new(r"toggle (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let ins: Vec<Instruction> = raw.lines().map( |l| {

        try_parse_instruction(&turn_on, l, Instruction::turn_on)
            .or_else(|| try_parse_instruction(&turn_off, l, Instruction::turn_off))
            .or_else(|| try_parse_instruction(&toggle, l, Instruction::toggle))
            .unwrap()
    }).collect();

    /*let part1_res = part1(&ins);
    println!("part 1: {}", part1_res);*/

    let part2_res = time_execution!(part2, &ins);
    println!("part 2: {}", part2_res);


    fn try_parse_instruction<'a>(r: &Regex, s: &'a str, f: fn(u32, u32, u32, u32) -> Instruction<'a>) -> Option<Instruction<'a>> {

        let mut v : Vec<Instruction> = r.captures_iter(s).map(|v| {
            let (_, [min_x, min_y, max_x, max_y]) = v.extract();
            f(min_x.parse().unwrap(), max_x.parse().unwrap(), min_y.parse().unwrap(), max_y.parse().unwrap())
        }).collect();
        if v.is_empty() {
            None
        }else{
            Some(v.remove(0))
        }
    }

    fn part1(ins: &Vec<Instruction>) -> u32 {

        let mut count: u32 = 0;
        for x in 0u32 .. 1000u32{
            for y in 0u32 .. 1000u32 {
                let res = ins.iter().fold(0, |acc, ins| ins.exec(x, y, acc));
                if res > 0 {
                    count += 1
                }
            }
        }
        count

    }

    fn part2(ins: &Vec<Instruction>) -> u32 {

        let mut sum: u32 = 0;
        for x in 0u32 .. 1000u32{
            for y in 0u32 .. 1000u32 {
                let res = ins.iter().fold(0, |acc, ins| ins.exec(x, y, acc));
                sum += res
            }
        }
        sum

    }
}



struct Instruction<'b> {
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
    ins: &'b str,
    update: fn(u32) -> u32
}

impl Instruction<'_>{
    fn exec(&self, x: u32, y:u32, value:u32) -> u32 {
        if self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y {
            (self.update)(value)
        }else{
            value
        }
    }

    fn turn_on<'a>(min_x: u32, max_x:u32, min_y:u32, max_y: u32) -> Instruction<'a> {
        Instruction{
            min_x,
            max_x,
            min_y,
            max_y,
            ins: "turn on",
            //update: (|_| 1),
            update: (|x| x + 1)
        }
    }

    fn turn_off<'a>(min_x: u32, max_x:u32, min_y:u32, max_y: u32) -> Instruction<'a> {
        Instruction{
            min_x,
            max_x,
            min_y,
            max_y,
            ins: "turn off",
            update: (|x| {
                if x == 0 {
                  x
                } else{
                    x - 1
                }

            } )
        }
    }

    fn toggle<'a>(min_x: u32, max_x:u32, min_y:u32, max_y: u32) -> Instruction<'a> {
        Instruction{
            min_x,
            max_x,
            min_y,
            max_y,
            ins: "toggle",
            update: (|x| x + 2)
            /*update: (|v| {
                if v == 1 {
                    0
                }else{
                    1
                }
            }),*/
        }
    }
}

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} - ({}, {}) to ({}, {})", self.ins, self.min_x, self.min_y, self.max_x, self.max_y)
    }
}