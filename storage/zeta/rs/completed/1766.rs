use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn split_by_2(s: &str) -> (usize, usize) {
    let mut iter = s.split_ascii_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn get_order(n: usize, precedents: Vec<(usize, usize)>) -> Vec<usize> {
    let mut order = Vec::new();

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    let mut indeg = vec![0; n];

    let mut queue = BinaryHeap::new();

    for (prec, succ) in precedents {
        indeg[succ - 1] += 1;
        graph[prec - 1].push(succ - 1);
    }
    for i in 0..n {
        graph[i].sort();
    }

    for (i, indeg) in indeg.iter().enumerate() {
        if *indeg == 0 {
            queue.push(Reverse(i));
        }
    }

    while !queue.is_empty() {
        let now = queue.pop().unwrap().0;
        order.push(now);

        for succ in graph[now].iter() {
            indeg[*succ] -= 1;
            if indeg[*succ] == 0 {
                queue.push(Reverse(*succ));
            }
        }
    }

    order
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let (n, m): (usize, usize);
    (n, m) = split_by_2(&line);
    let precedents: Vec<(usize, usize)>;

    precedents = (0..m)
        .map(|_| {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            split_by_2(&line)
        })
        .collect();

    get_order(n, precedents)
        .into_iter()
        .for_each(|i| print!("{} ", i + 1));
    println!();
}
