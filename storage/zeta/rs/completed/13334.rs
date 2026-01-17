use std::{
    cmp::{max, Reverse},
    collections::BinaryHeap,
    i32,
    io::{stdin, Read},
};

fn main() {
    let mut buffer = Vec::new();

    stdin().read_to_end(&mut buffer).unwrap();
    let mut tokens = buffer.split(|&b| b <= b' ');
    let mut next_i32 = || {
        tokens.find(|s| !s.is_empty()).map(|s| {
            let mut res = 0i32;
            let mut neg = false;
            let mut start = 0;
            if s[0] == b'-' {
                neg = true;
                start = 1;
            }
            for i in start..s.len() {
                res = res * 10 + (s[i] - b'0') as i32;
            }
            if neg {
                -res
            } else {
                res
            }
        })
    };

    let n = next_i32().unwrap();

    let mut commute_infos = (0..n)
        .map(|_| {
            let (a, b) = (next_i32().unwrap(), next_i32().unwrap());
            if b > a {
                (a, b)
            } else {
                (b, a)
            }
        })
        .collect::<Vec<(i32, i32)>>();

    let d = next_i32().unwrap();
    commute_infos.sort_unstable_by_key(|&(_, b)| b);
    let mut max_benefit = 0;
    let mut deq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for &(h, o) in commute_infos.iter() {
        if o - h > d {
            continue;
        }
        deq.push(Reverse(h));

        while let Some(ch) = deq.peek() {
            if ch.0 < o - d {
                deq.pop();
            } else {
                break;
            }
        }

        max_benefit = max(max_benefit, deq.len());
    }

    println!("{}", max_benefit);
}
