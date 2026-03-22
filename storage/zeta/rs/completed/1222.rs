use std::cmp::max;
use std::io::{read_to_string, stdin};
const MAX_PARTS: usize = 2000000;
fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let mut parts = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<_>>();
    parts.sort();

    let mut res = 0;

    let mut maps = vec![0; MAX_PARTS + 1];
    for p in parts {
        maps[p] += 1;
    }

    for d in 1..=MAX_PARTS {
        let mut q = 0;
        let mut cnt = 0;
        for i in (0..=MAX_PARTS).step_by(d) {
            q += d * maps[i];
            cnt += maps[i];
        }
        if cnt < 2 {
            continue;
        }
        res = max(res, q);
    }
    println!("{}", res);
}
