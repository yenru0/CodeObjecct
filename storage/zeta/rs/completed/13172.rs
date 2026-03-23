use std::io::{read_to_string, stdin};

const MOD: usize = 1_000_000_007;

fn get_mod_power(n: usize, p: usize) -> usize {
    let mut x = p;
    let mut curr = n % MOD;
    let mut res = 1;
    while x > 0 {
        if x & 1 == 1 {
            res *= curr;
            res %= MOD;
        }
        x >>= 1;
        curr *= curr;
        curr %= MOD;
    }
    res
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let m = iter.next().unwrap();
    let dices = (0..m)
        .map(|_| (iter.next().unwrap(), iter.next().unwrap()))
        .collect::<Vec<_>>();

    let res = dices
        .iter()
        .map(|&(n, s)| s * get_mod_power(n, MOD - 2) % MOD)
        .sum::<usize>()
        % MOD;
    println!("{}", res);
}
