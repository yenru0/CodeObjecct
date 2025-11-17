use std::io::stdin;

fn get_primes_up_to(n: u64) -> Vec<u64> {
    let mut i = 3;
    let mut primes = vec![2];
    while i <= n {
        let mut is_prime = true;
        for &p in &primes {
            if p * p > i {
                break;
            }
            if i % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
        i += 2;
    }

    primes
}

fn get_divisors(n: u64) -> Vec<(u64, u64)> {
    let primes = get_primes_up_to((n as f64).sqrt() as u64 + 1);
    let mut divisors = vec![];
    let mut remainder = n;

    for &p in &primes {
        if p * p > remainder {
            break;
        }
        let mut count = 0;
        while remainder % p == 0 {
            remainder /= p;
            count += 1;
        }
        if count > 0 {
            divisors.push((p, count));
        }
    }

    if remainder > 1 {
        divisors.push((remainder, 1));
    }
    divisors
}

fn euler_phi(n: u64) -> u64 {
    let divisors = get_divisors(n);
    let mut result = n;
    for (p, _) in divisors {
        result = result / p * (p - 1);
    }
    result
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n: u64 = line.trim().parse().unwrap();
    println!("{}", euler_phi(n));
}
