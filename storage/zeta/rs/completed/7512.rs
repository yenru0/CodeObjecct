use std::io::{read_to_string, stdin};

const P_MAX: usize = 10_000_000;

fn get_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![2];

    for i in (3..=n).step_by(2) {
        for &p in primes.iter() {
            if i % p == 0 {
                break;
            }
            if p * p >= i {
                primes.push(i);
                break;
            }
        }
    }

    primes
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let tc = iter.next().unwrap();

    let primes = get_primes(P_MAX);
    let cum_primes = {
        let mut cp = vec![0];
        for p in primes.iter() {
            cp.push(cp.last().unwrap() + p);
        }
        cp
    };

    (1..=tc).for_each(|case| {
        let m = iter.next().unwrap();
        let ns = (0..m).map(|_| iter.next().unwrap()).collect::<Vec<_>>();
        let mut idxs = vec![0; m];
        let mut values = vec![0; m];
        let mut _cnt = 0;
        loop {
            let finish_flag = values[0] != 0 && values.iter().all(|&x| x == values[0]);

            if finish_flag {
                break;
            }

            let i = values
                .iter()
                .enumerate()
                .min_by_key(|&(_, &v)| v)
                .unwrap()
                .0;
            let n = ns[i];
            let mut idx = idxs[i];
            loop {
                let s = cum_primes[idx + n] - cum_primes[idx];
                match primes.binary_search(&s) {
                    Ok(x) => {
                        values[i] = primes[x];
                        idxs[i] = idx + 1;
                        break;
                    }
                    Err(_) => {}
                }

                idx += 1;
            }
        }

        println!("Scenario {}:\n{}\n", case, values[0]);
    })
}
