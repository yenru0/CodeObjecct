use std::io::{read_to_string, stdin};

const MOD: usize = 1_000_000_007;

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap());

    let a = iter.next().unwrap();
    let mut x = iter.next().unwrap();
    
    let mut res = 1;
    let mut curr = a % MOD;
    while x > 0 {
        if x & 1 == 1usize {
            res *= curr;
            res %= MOD;
        }
        x >>= 1;
        curr *= curr;
        curr %= MOD;
    }
    println!("{}", res);
}