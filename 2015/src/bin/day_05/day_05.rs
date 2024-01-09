use std::collections::HashMap;
use std::fs;


fn main() {
    static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    static BAD: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let binding = fs::read_to_string("src/bin/day_05/input.data").unwrap();
    let s = binding.lines();
    let part1_res = s.clone().filter(|x| part_1(*x)).count();
    println!("part 1 result: {}", part1_res);

    let part2_res = s.filter(|x| part_2(*x)).count();
    println!("part 2 result: {}", part2_res);


    fn part_1(s: &str) -> bool {
        let chars : Vec<char> = s.chars().collect();
        let contains_vowels = s.chars().filter(|x| VOWELS.contains(&x)).count() >= 3;
        if contains_vowels {
            let (cd, cb) = chars.windows(2).fold((false, false), |(contains_doubles, contains_bad), pair | {
                let first = *pair.first().unwrap();
                let second = *pair.last().unwrap();
                let concatted = String::from_iter(pair);
                let bla: &str = &concatted;
                (
                    contains_doubles || first == second,
                    contains_bad || BAD.contains(&bla)
                )
            });
            cd && !cb
        }else{
            contains_vowels //no vowels..bomb
        }
    }

    fn part_2(s: &str) -> bool {
        let binding = s.chars().enumerate().collect::<Vec<(usize, char)>>();
        let x = binding.windows(2);
        let mut acc_map: HashMap<String, Vec<usize>> = HashMap::new();
        x.for_each(|pair| {
            let (first_index, first_char) = *pair.first().unwrap();
            let (_, second_char) = *pair.last().unwrap();
            let mut s = first_char.to_string();
            s.push(second_char);
            acc_map.entry(s).or_insert(vec![]).push(first_index);

        });

        let contains_non_overlaps = acc_map.iter().any(|(_, v)| {
            if v.len() == 2 {
                (v.last().unwrap() - v.first().unwrap()) > 1
            } else {
                v.len() > 2
            }
        });

        let contains_palindrome = s.chars().collect::<Vec<char>>().windows(3).any(|c| {
            *c.first().unwrap() == *c.last().unwrap()
        });

        contains_non_overlaps && contains_palindrome
    }


}
