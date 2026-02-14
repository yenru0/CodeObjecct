use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (n, k) = (iter.next().unwrap(), iter.next().unwrap());

    let mut edges = vec![0; n];
    let mut flg = false;
    (0..n).for_each(|i| {
        let nxt = iter.next().unwrap();
        edges[i] = nxt;
        if nxt == k {
            flg = true;
        }
    });

    if !flg {
        println!("{}", -1);
        return;
    }

    let mut curr = 0;
    let mut nxt;
    let mut vis = vec![false; n];
    let mut m = 0;
    'a: loop {
        if vis[curr] {
            m = -1;
            break;
        } else {
            vis[curr] = true;
        }
        if curr == k {
            break 'a;
        }
        nxt = edges[curr];

        curr = nxt;
        m += 1;
    }
    println!("{}", m);
}
