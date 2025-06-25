use std::io::stdin;

fn get_score(a: u32, b: u32) -> u32 {
    let mut k = a * b;
    vec![100000, 10000, 1000, 100, 10, 1]
        .iter()
        .map(|&x| {
            let t = k / x;
            k %= x;
            t
        })
        .sum()
}

fn get_mult_score(a: &Vec<u32>) -> u32 {
    let mut a_cnt = vec![0; 1000];
    for item in a {
        a_cnt[*item as usize] += 1;
    }

    a_cnt
        .iter()
        .enumerate()
        .flat_map(|(ia, &ca)| {
            a_cnt
                .iter()
                .enumerate()
                .skip(ia)
                .map(|(ib, &cb)| (ia, ca, ib, cb))
                .collect::<Vec<_>>()
        })
        .map(|(ia, ca, ib, cb)| {
            if ca == 0 || cb == 0 {
                0 as u32
            } else if ia == ib && ca <= 1 {
                0 as u32
            } else {
                get_score(ia as u32, ib as u32)
            }
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    let mut lines = stdin().lines();
    lines.next();
    let a: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("{}", get_mult_score(&a));
}
