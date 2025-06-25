use std::io::stdin;

fn get_primes_upto(n: u32) -> Vec<u32> {
    let mut primes = vec![2];
    for i in (3..=n).step_by(2) {
        let mut flag = true;
        for &p in primes.iter() {
            if p > (i as f32).sqrt() as u32 {
                break;
            }
            if i % p == 0 {
                flag = false;
                break;
            }
        }
        if flag {
            primes.push(i);
        }
    }

    primes
}

fn get_ways_sum_primes(n: u32) -> usize {
    let primes = get_primes_upto(n);

    let cumulative_primes: Vec<u32> = std::iter::once(0)
        .chain(primes.iter().scan(0, |acc, &p| {
            *acc += p;
            Some(*acc)
        }))
        .collect();

    let mut lo: usize = 0;
    let mut hi: usize = 1;

    let mut ways: usize = 0;

    let get_sum = |lo: usize, hi: usize| cumulative_primes[hi] - cumulative_primes[lo];

    while lo <= hi && lo < cumulative_primes.len() && hi < cumulative_primes.len() {
        let s = get_sum(lo, hi);
        if n == s {
            ways += 1;
            lo += 1;
        } else if n > s {
            hi += 1;
        } else {
            lo += 1;
        }
    }

    ways
}

fn main() {
    let n = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    println!("{}", get_ways_sum_primes(n));
}
