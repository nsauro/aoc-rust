use std::fs;

fn main() {
    let raw = fs::read_to_string("src/bin/day_08/input.data").unwrap();

    let (total_size, total_chars) = &raw.lines().fold((0u32, 0u32), |(size, chars), s| {
        (size + s.len() as u32, chars + get_chars(s)) //safe b/c these sizes are small
    });

    let new_encoded_size = raw
        .lines()
        .fold(0u32, |size, s| size + get_encoded_length(s));

    let part1_ans = total_size - total_chars;
    println!("part 1: {}", part1_ans);

    let part2_ans = new_encoded_size - total_size;
    println!("part 2: {}", part2_ans);
}

fn get_chars(s: &str) -> u32 {
    let mut count = 0;
    let mut iter = s.chars().into_iter();
    while let Some(c) = iter.next() {
        match c {
            '"' => (),
            '\\' => match iter.next().unwrap() {
                '"' => count += 1,
                '\\' => count += 1,
                'x' => {
                    count += 1;
                    iter.next();
                    iter.next();
                }
                _ => panic!("omg!!"),
            },
            _ => count += 1,
        }
    }
    count
}

fn get_encoded_length(s: &str) -> u32 {
    let mut count = 0;
    let mut iter = s.chars().into_iter();
    while let Some(c) = iter.next() {
        match c {
            '"' => count += 2,  //needs needs encoding
            '\\' => count += 2, //needs encoding
            _ => count += 1,
        }
    }
    count + 2
}
