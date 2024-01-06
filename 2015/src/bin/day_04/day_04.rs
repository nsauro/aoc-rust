use aoc_2015::time_execution;
use tailcall::tailcall;

fn main() {
    let key = "ckczppom";
    let part1_res = time_execution!(find_hash_with_prefix, 0, key, "00000");
    println!("{}", part1_res);
    let part2_res = time_execution!(find_hash_with_prefix, 0, key, "000000");
    println!("{}", part2_res);

    let part1_res_no_rec = time_execution!(find_hash_with_prefix_no_rec, key, "00000");
    println!("{}", part1_res_no_rec);
    let part2_res_no_rec = time_execution!(find_hash_with_prefix_no_rec, key, "000000");
    println!("{}", part2_res_no_rec);
}


#[tailcall]
fn find_hash_with_prefix(i: i32, key: &str, prefix: &str) -> i32 {
    let s = format!("{}{}", key, i);
    let digest = md5::compute(s);
    let hash = format!("{:x}", digest);
    if hash.starts_with(prefix){
        i
    }else{
        find_hash_with_prefix(i + 1, key, prefix)
    }
}

fn find_hash_with_prefix_no_rec(key: &str, prefix: &str) -> i32 {
    let mut i = 0;
    let mut done = false;
    while(!done){
        let s = format!("{}{}", key, i);
        let digest = md5::compute(s);
        let hash = format!("{:x}", digest);
        if hash.starts_with(prefix){
            done = true;
        }else{
            i += 1;
        }
    }
    i
}