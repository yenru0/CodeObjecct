use std::{
    cmp::Ordering,
    io::{read_to_string, stdin},
};

enum Comp {
    HIGH,
    LOW,
}

// 5! * n + n^2
fn original_ordering(n: usize, ords: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut cmp_map: Vec<Vec<usize>> = vec![];

    for ord in ords {
        cmp_map.push(vec![0; n]);
        let target = cmp_map.last_mut().unwrap();
        for (i, &cow) in ord.iter().enumerate() {
            target[cow - 1] = i;
        }
    }

    let mut cows = (1..=n).collect::<Vec<usize>>();

    cows.sort_by(|&a, &b| {
        let mut cnt = 0;
        for pic in 0..5 {
            if cmp_map[pic][a - 1] < cmp_map[pic][b - 1] {
                cnt += 1;
            }
        }
        if cnt >= 3 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    cows
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let orderings = (0..5)
        .map(|_| {
            (0..n)
                .map(|_| iter.next().unwrap().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    original_ordering(n, &orderings)
        .iter()
        .for_each(|x| println!("{}", x));
}
