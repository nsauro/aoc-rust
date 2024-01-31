use itertools::Itertools;

fn main() {

    let mut pwd: Vec<char> = String::from("cqjxxyzz").chars().collect();

    loop {
        increment(&mut pwd);
        if is_valid(&pwd) {
            break
        }
    }

    println!("{}", pwd.iter().format(""));


    fn is_valid(s: &Vec<char>) -> bool {
        let mut has_straight = false;
        let mut cur_straight_count = 1;
        let mut pair_count = 0;
        let mut last_letter = '!';
        let mut cur_matching_size = 1;
        s.iter().for_each(|x| {
            if !has_straight {
                if (*x as i16) - (last_letter as i16) == 1 {
                    cur_straight_count += 1;
                    if cur_straight_count == 3 {
                        has_straight = true
                    }
                } else{
                    cur_straight_count = 1;
                }
            }
            if pair_count < 2 {
                if last_letter == *x {
                    cur_matching_size += 1;
                    if cur_matching_size == 2 {
                        pair_count += 1;
                    }
                    if cur_matching_size == 4 {
                        pair_count += 1
                    }
                } else{
                    cur_matching_size = 1;
                }
            }
            last_letter = *x;

        });
        has_straight && pair_count >= 2

    }



    fn increment(s: &mut Vec<char>) {
        for i in  (0 .. s.len()).rev(){
            if s[i] == 'z' {
                s[i] = 'a';
            }else{
                let counter = if s[i] == 'i' || s[i] == 'o' || s[i] == 'l'{
                    2
                } else {
                    1
                };
                s[i] = (s[i] as u8 + counter) as char;
                break;
            }
        }

    }


}