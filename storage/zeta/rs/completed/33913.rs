use std::io::stdin;

const MOD: usize = 1_000_000_007;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let n = line.trim().parse::<usize>().unwrap();

    /*
        f[0] even =  1 // empty
        f[0] odd  =  0
        f[1] even = 24 // without SC
        f[1] odd  =  2 // with SC

        f[2] even = f[1] even * 24 + f[1] odd * 2
        f[2] odd  = f[1] even * 2  + f[1] odd * 24 

    */

    let mut f0e = 1;
    let mut f0i = 0;
    let mut f1e = 24;
    let mut f1i = 2;

    for _ in 2..=n {
        let f2e = (f1e * 24 + f1i * 2) % MOD;
        let f2i = (f1e * 2 + f1i * 24) % MOD;
        f0e = f1e;
        f0i = f1i;
        f1e = f2e;
        f1i = f2i;
    }

    println!("{}", f1e);
}
