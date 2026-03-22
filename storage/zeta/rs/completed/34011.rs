use std::cmp::max;
use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let mut n = iter.next().unwrap();
    let mut parents = (0..n - 1)
        .map(|_| iter.next().unwrap() - 1)
        .collect::<Vec<_>>();

    let mut edges = vec![vec![]; n];
    for (i, &p) in parents.iter().enumerate() {
        edges[p].push(i + 1);
    }

    let mut depthes = vec![0; n];
    let mut deq = vec![];
    deq.push((0, 0));
    let mut max_depth = 0;
    while !deq.is_empty() {
        let (u, d) = deq.pop().unwrap();
        depthes[d] += 1;
        if edges[u].is_empty() {
            max_depth = max(max_depth, d);
            continue;
        }
        for &v in edges[u].iter() {
            deq.push((v, d + 1));
        }
    }

    let mut res = 0;
    for d in 2..=n {
        let mut q = 0;
        for i in (0..=max_depth).step_by(d) {
            q += depthes[i];
        }
        res = max(res, q);
    }

    println!("{}", res);
}
