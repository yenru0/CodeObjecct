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

    let n = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut fac = vec![];
    let mut facinv = vec![];

    fac.push(1);
    facinv.push(1);
    fac.push(1);
    facinv.push(1);
    for i in 2..=n {
        let before = fac.last().unwrap();
        fac.push((before * i) % MOD);
        facinv.push(get_mod_power(*fac.last().unwrap(), MOD - 2));
    }
    let nck = (fac[n] * facinv[n - k] % MOD) * facinv[k] % MOD;
    println!("{}", nck);
}
