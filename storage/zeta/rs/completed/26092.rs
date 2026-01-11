use std::io::{read_to_string, stdin};

fn get_primes(to: u64) -> Vec<u64> {
    let mut primes = vec![2];
    for i in (3..=to).step_by(2) {
        if i * i > to {
            break;
        }
        for p in primes.iter() {
            if i % p == 0 {
                break;
            }
            if p * p > i {
                primes.push(i);
                break;
            }
        }
    }
    primes
}

fn prime_factorization(mut n: u64, primes: &Vec<u64>) -> Vec<u64> {
    let mut factors = vec![];
    for p in primes {
        while n % p == 0 {
            factors.push(*p);
            n /= p;
        }

        if n == 1 {
            break;
        }
    }
    if n != 1 {
        factors.push(n);
    }
    factors
}

fn get_mathematical_least_common_parent(a: u64, b: u64) -> u64 {
    let primes = get_primes(a.max(b));

    let mut a_factors = prime_factorization(a, &primes);
    let mut b_factors = prime_factorization(b, &primes);

    a_factors.reverse();
    b_factors.reverse();

    let mut res = 1;
    for (p1, p2) in a_factors.iter().zip(b_factors.iter()) {
        if p1 == p2 {
            res *= p1;
        } else {
            break;
        }
    }
    res
}

fn main() {
    let line = read_to_string(stdin()).unwrap();

    let mut iter = line.trim().split(' ').map(|x| x.parse::<u64>().unwrap());

    let (a, b) = (iter.next().unwrap(), iter.next().unwrap());

    println!("{}", get_mathematical_least_common_parent(a, b));
}
