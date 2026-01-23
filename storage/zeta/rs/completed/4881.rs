use std::{
    collections::HashMap,
    io::{read_to_string, stdin},
    u64,
};

fn transform(mut x: u64) -> u64 {
    let mut s = 0;
    while x > 0 {
        let t = x % 10;
        s += t * t;
        x = x / 10;
    }
    s
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    loop {
        let mut dist_from_a: HashMap<u64, u64> = HashMap::new();
        let mut dist_from_b: HashMap<u64, u64> = HashMap::new();

        let (a, b) = (iter.next().unwrap(), iter.next().unwrap());

        if a == 0 && b == 0 {
            break;
        }
        let mut curr = a;
        let mut step = 1;
        while !dist_from_a.contains_key(&curr) {
            dist_from_a.insert(curr, step);
            curr = transform(curr);
            step += 1;
        }
        curr = b;
        step = 1;
        while !dist_from_b.contains_key(&curr) {
            dist_from_b.insert(curr, step);
            curr = transform(curr);
            step += 1;
        }

        let mut res = u64::MAX;
        for (&n, &dist_a) in dist_from_a.iter() {
            if let Some(dist_b) = dist_from_b.get(&n) {
                res = res.min(dist_a + dist_b);
            }
        }
        if res == u64::MAX {
            res = 0;
        }

        println!("{} {} {}", a, b, res);
    }
}
