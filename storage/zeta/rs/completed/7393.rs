use std::{
    io::{read_to_string, stdin},
    iter::zip,
};

fn get_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![2];
    for i in (3..=n).step_by(2) {
        for p in primes.iter() {
            if i % p == 0 {
                break;
            } else if i < p * p {
                primes.push(i);
                break;
            }
        }
    }
    primes
}

fn legendre(n: usize, p: usize) -> usize {
    let mut ppow = p;
    let mut curr = n / ppow;
    let mut res = curr;

    while curr != 0 {
        ppow = ppow * p;
        curr = n / ppow;
        res += curr;
    }

    res
}

fn prime_factorization(m: usize) -> Vec<(usize, usize)> {
    let primes = get_primes(32000);
    let mut curr = m;
    let mut res = vec![];
    for &p in primes.iter() {
        let mut cnt = 0;
        while curr % p == 0 {
            curr /= p;
            cnt += 1;
        }
        if cnt > 0 {
            res.push((p, cnt));
        }
    }

    if curr > 1 {
        res.push((curr, 1));
    }

    res
}

fn combinatorics_p_cnt(n: usize, k: usize, p: usize) -> usize {
    legendre(n, p) - legendre(n - k, p) - legendre(k, p)
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let (n, m) = (iter.next().unwrap(), iter.next().unwrap());

    let factors = prime_factorization(m);

    let mut res = vec![];

    for k in 0..=(n - 1) {
        let comb = factors
            .iter()
            .map(|&(p, _)| combinatorics_p_cnt(n - 1, k, p));

        let flg = zip(comb, factors.iter()).all(|(q1, &q2)| q1 >= q2.1);

        if flg {
            res.push(k + 1);
        }
    }

    println!("{}", res.len());
    for i in res {
        print!("{} ", i);
    }
    println!()
}
