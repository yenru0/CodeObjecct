use std::cmp::{max, min};
use std::io::stdin;

fn pair_matching(vs: &Vec<(i32, i32)>, i: usize, j: usize) -> i32 {
    let (v0, v1) = (vs[i], vs[j]);
    max(
        max((v0.0 - v1.0).abs(), (v0.1 - v1.1).abs()),
        max((v0.0 - v1.1).abs(), (v0.1 - v1.0).abs()),
    )
}

fn solve_max_pair_matching(n: i32, vs: &Vec<(i32, i32)>) -> i32 {
    let vs_index: Vec<usize> = (0..(2 * n as usize)).collect();

    let mut vs_index_min = vs_index.clone();
    let mut vs_index_max = vs_index.clone();
    vs_index_min.sort_by_key(|&x| min(vs[x].0, vs[x].1));
    vs_index_max.sort_by_key(|&x| max(vs[x].0, vs[x].1));

    let mut vs_index_presence = vec![true; 2 * n as usize];

    let mut s = 0;

    let mut min_point = 0;
    let mut max_point = 2 * n as usize - 1;
    let mut cnt = 0;
    while cnt < n {
        if !vs_index_presence[vs_index_min[min_point]] {
            min_point += 1;
            continue;
        }
        if !vs_index_presence[vs_index_max[max_point]] {
            max_point -= 1;
            continue;
        }
        if vs_index_min[min_point] == vs_index_max[max_point] {
            if min_point == 2 * n as usize - 1 {
                max_point -= 1;
            } else {
                min_point += 1;
            }
            continue;
        }
        cnt += 1;
        s += pair_matching(vs, vs_index_min[min_point], vs_index_max[max_point]);
        vs_index_presence[vs_index_min[min_point]] = false;
        vs_index_presence[vs_index_max[max_point]] = false;
        min_point += 1;
        max_point -= 1;
    }

    s
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<i32>().unwrap();
    drop(line);
    let vs = (0..(2 * n))
        .map(|_| {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            let v: Vec<i32> = line
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            (v[0], v[1])
        })
        .collect::<Vec<(i32, i32)>>();

    println!("{}", solve_max_pair_matching(n, &vs));
}
