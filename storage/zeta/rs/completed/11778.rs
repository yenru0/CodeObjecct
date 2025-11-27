use std::io::stdin;

const MOD: u64 = 1_000_000_007;

struct sqmat2 {
    data: [[u64; 2]; 2],
}

impl sqmat2 {
    fn mul(&self, other: &sqmat2) -> sqmat2 {
        let mut res = sqmat2 { data: [[0; 2]; 2] };
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    res.data[i][j] =
                        (res.data[i][j] + self.data[i][k] * other.data[k][j] % MOD) % MOD;
                }
            }
        }
        res
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn fib(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut f = sqmat2 {
        data: [[1, 0], [0, 1]],
    };
    let mut curr = sqmat2 {
        data: [[1, 1], [1, 0]],
    };
    while n > 0 {
        if n & 1 == 1 {
            f = f.mul(&curr);
        }
        n >>= 1;
        curr = curr.mul(&curr);
    }

    f.data[0][1] % MOD
}

fn gcd_fib(n: u64, m: u64) -> u64 {
    if m == 0 {
        return n;
    }
    fib(gcd(m, n % m))
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line.split(' ').map(|x| x.trim().parse::<u64>().unwrap());

    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    println!("{}", gcd_fib(n, m));
}
