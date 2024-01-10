use aoc_2015::time_execution;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day_03/input.data").unwrap();
    let part1_res = time_execution!(part1, &input);
    println!("part 1 result: {}", part1_res);

    let part2_res = time_execution!(part2, &input);
    println!("part 2 result: {}", part2_res);

    fn part1(s: &String) -> i32 {
        let (locs, _) = s
            .chars()
            .fold((HashMap::new(), (0, 0)), |(mut acc, cur_loc), c| {
                let new_dir = get_next(cur_loc, c);
                let count = acc.entry(new_dir).or_insert(0);
                *count += 1;
                (acc, new_dir)
            });

        let x = locs.iter().filter(|&(_, &value)| value >= 1);
        x.count() as i32
    }

    fn part2(s: &String) -> i32 {
        let (locs, _) = s.chars().fold(
            (HashMap::new(), ((0, 0), (0, 0))),
            |(mut acc, (cur_loc, other_loc)), c| {
                let new_dir = get_next(cur_loc, c);
                let count = acc.entry(new_dir).or_insert(0);
                *count += 1;
                (acc, (other_loc, new_dir))
            },
        );

        let x = locs.iter().filter(|&(_, &value)| value >= 1);
        x.count() as i32
    }

    fn get_next(cur_loc: (i32, i32), c: char) -> (i32, i32) {
        match c {
            '^' => (cur_loc.0 - 1, cur_loc.1),
            '<' => (cur_loc.0, cur_loc.1 - 1),
            'v' => (cur_loc.0 + 1, cur_loc.1),
            '>' => (cur_loc.0, cur_loc.1 + 1),
            _ => cur_loc,
        }
    }
}
