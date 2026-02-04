use std::io::{read_to_string, stdin};

fn fact(x: usize) -> usize {
    static F: [usize; 16] = [
        1,
        1,
        2,
        6,
        24,
        120,
        720,
        5040,
        40320,
        362880,
        3628800,
        39916800,
        479001600,
        6227020800,
        87178291200,
        1307674368000,
    ];
    F[x]
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let perm = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<usize>>();

    let mut vis = vec![false; n];

    let mut res = 0;

    for i in 0..n {
        let k = perm[i];
        let r = n - i - 1;
        let mut cnt = 0;
        for j in 0..k {
            if !vis[j] {
                cnt += 1;
            }
        }
        res += fact(r) * cnt;
        // cl
        vis[k] = true;
    }

    println!("{}", res + 1);
}
