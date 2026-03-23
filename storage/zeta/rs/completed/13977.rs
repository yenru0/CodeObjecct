use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

const MOD: usize = 1_000_000_007;
const N_MAX: usize = 4_000_000;

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

    let mut fac = vec![0; N_MAX + 1];
    let mut facinv = vec![0; N_MAX + 1];

    fac[0] = 1;
    facinv[0] = 1;
    fac[1] = 1;
    facinv[1] = 1;
    for i in 2..=N_MAX {
        fac[i] = (fac[i - 1] * i) % MOD;
    }

    facinv[N_MAX] = get_mod_power(fac[N_MAX], MOD - 2);
    for i in (2..N_MAX).rev() {
        facinv[i] = facinv[i + 1] * (i + 1) % MOD;
    }

    let get_nck = |n: usize, k: usize| (fac[n] * facinv[n - k] % MOD) * facinv[k] % MOD;

    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    (0..m).for_each(|_| {
        let (n, k) = (iter.next().unwrap(), iter.next().unwrap());

        write!(writer, "{}\n", get_nck(n, k)).unwrap();
    });
}
