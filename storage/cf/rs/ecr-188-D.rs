use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let t = iter.next().unwrap();
    (0..t).for_each(|_| {
        let (n, m) = (iter.next().unwrap(), iter.next().unwrap());
        let mut res = 0;
        if m == 0 {
            res = n;
        } else {
            let mut edges = vec![vec![]; n];
            let mut vis = vec![false; n];
            for _ in 0..m {
                let (u, v) = (iter.next().unwrap() - 1, iter.next().unwrap() - 1);
                edges[u].push(v);
                edges[v].push(u);
            }

            let mut deq = vec![];
            for i in 0..n {
                deq.push(i);
            }

            while !deq.is_empty() {
                let ()
            }


        }
    });
}
