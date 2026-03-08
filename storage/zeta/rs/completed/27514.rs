use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{read_to_string, stdin},
};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let arr = (0..n)
        .map(|_| Reverse(iter.next().unwrap()))
        .filter(|x| x.0 != 0)
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::from(arr);

    while (heap.len() >= 2) {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        if a != b {
            heap.push(Reverse(a.max(b)));
        } else {
            heap.push(Reverse(a + b));
        }
    }
    let res = heap.iter().max().unwrap().0;
    println!("{}", res);
}
