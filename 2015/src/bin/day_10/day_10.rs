
fn main() {
    let input = "1321131112";

    let res = expand(input.clone(), 40);
    println!("part_1: {}", res.len());

    let res_2 = expand(input, 50);
    println!("part_2: {}", res_2.len());


    fn expand(s: &str, times_left: u32) -> String {
        if times_left == 0 {
            String::from(s)
        }else{
            let mut res = String::new();
            let mut cur = 'x';
            let mut cur_count = 0u32;
            s.chars().for_each(|c| {
                if c == cur {
                    cur_count += 1;
                }else {
                    if cur != 'x' {
                        res.push_str(format!("{}", cur_count).as_str());
                        res.push(cur);
                    }
                    cur = c;
                    cur_count = 1
                }
            });
            res.push_str(format!("{}", cur_count).as_str());
            res.push(cur);
            expand(res.as_str(), times_left - 1)
        }

    }



}