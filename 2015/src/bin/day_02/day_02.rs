use std::fs;

fn main() {
    let binding = fs::read_to_string("src/bin/day_02/input.data").unwrap();
    let s = binding.lines();
    let part1_res: i32 = s.clone().map(|x| part_1(x)).sum();
    let part2_res: i32 = s.clone().map(|x| part_2(x)).sum();
    println!("part 1: {}", part1_res);
    println!("part 2: {}", part2_res)
}
fn part_1(s: &str) -> i32 {
    let mut parts = s.split("x");
    let l = parts.next().and_then(to_int).unwrap();
    let w = parts.next().and_then(to_int).unwrap();
    let h = parts.next().and_then(to_int).unwrap();

    let lw = l * w;
    let lh = l * h;
    let hw = h * w;

    let sides = vec![lw, lh, hw];
    (2 * lw) + (2 * lh) + (2 * hw) + sides.iter().min().unwrap()
}

fn part_2(s: &str) -> i32 {
    let mut parts = s.split("x");
    let l = parts.next().and_then(to_int).unwrap();
    let w = parts.next().and_then(to_int).unwrap();
    let h = parts.next().and_then(to_int).unwrap();
    let mut v : Vec<i32> = vec![l,w,h];
    v.sort();
    let (left, _) = v.split_at(v.len() - 1);
    let perim : i32 = left.iter().map(|x| 2 * x).sum();
    perim + v.iter().product::<i32>()
}

fn to_int(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

