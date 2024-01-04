use std::fs;
fn main() {
    let s = fs::read_to_string("input.data").unwrap();
    println!("part1: {}", part1(&s));
    println!("part2: {}", part2(&s))
}

fn part1(s: &String) -> i32 {
    s.chars()
        .fold(0, |sum, val| if val == ')' { sum - 1 } else { sum + 1 })
}

fn part2(s: &String) -> usize {
    let mut sum = 0;
    for (i, ch) in s.char_indices() {
        if ch == '(' {
            sum += 1
        } else {
            sum -= 1
        }
        if sum == -1 {
            return i + 1;
        }
    }
    return 0;
}
