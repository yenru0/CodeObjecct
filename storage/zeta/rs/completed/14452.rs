use std::{
    cmp::{max, Reverse},
    collections::BinaryHeap,
    io::{read_to_string, stdin},
};

fn least_k(n: usize, durs: &Vec<usize>, tmax: usize) -> usize {
    let mut lo = 1;
    let mut hi = n;
    let mut res = n;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let mut elapsed = 0;

        let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        for &d in durs[0..mid].iter() {
            elapsed = max(elapsed, d);
            heap.push(Reverse(d));
        }

        for &d in durs[mid..n].iter() {
            let finished = heap.pop().unwrap().0;
            heap.push(Reverse(finished + d));
            elapsed = max(elapsed, finished + d);
        }
        if elapsed <= tmax {
            res = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    res
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let tmax = iter.next().unwrap();

    let duration_orders: Vec<usize> = (0..n).map(|_| iter.next().unwrap()).collect();

    println!("{}", least_k(n, &duration_orders, tmax));
}
