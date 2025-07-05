use std::io::stdin;

fn interpret_map(size: (usize, usize), map_strings: &Vec<String>) -> Vec<usize> {
    let mut edges: Vec<usize> = vec![usize::MAX; size.0 * size.1];

    for i in 0..size.0 {
        let mut s = map_strings[i].chars();
        for j in 0..size.1 {
            let c = s.next().unwrap();
            match c {
                'D' => {
                    if i != size.0 - 1 {
                        edges[i * size.1 + j] = (i + 1) * size.1 + j;
                    }
                }
                'U' => {
                    if i != 0 {
                        edges[i * size.1 + j] = (i - 1) * size.1 + j;
                    }
                }
                'L' => {
                    if j != 0 {
                        edges[i * size.1 + j] = i * size.1 + j - 1;
                    }
                }
                'R' => {
                    if j != size.1 - 1 {
                        edges[i * size.1 + j] = i * size.1 + j + 1;
                    }
                }
                _ => {}
            }
        }
    }

    edges
}

fn disjoint_find(parents: &mut Vec<usize>, x: usize) -> usize {
    let mut nd = x;
    while nd != parents[nd] {
        parents[nd] = parents[parents[nd]];
        nd = parents[nd];
    }
    nd
}

fn disjoint_union(parents: &mut Vec<usize>, x: usize, y: usize) -> bool {
    let xr = disjoint_find(parents, x);
    let yr = disjoint_find(parents, y);
    if xr == yr {
        return false;
    }
    if xr > yr {
        parents[yr] = xr;
    } else {
        parents[xr] = yr;
    }
    true
}

fn get_count_of_safezone(size: (usize, usize), edges: &Vec<usize>) -> usize {
    let mut parents = (0..size.0 * size.1).collect::<Vec<_>>();
    let mut cnt = size.0 * size.1;
    for i in 0..size.0 * size.1 {
        if edges[i] == usize::MAX {
            continue;
        }
        if disjoint_union(&mut parents, i, edges[i]) {
            cnt -= 1;
        }
    }

    cnt
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line.split_ascii_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    line.clear();

    let map_strings = (0..n)
        .map(|_| {
            stdin().read_line(&mut line).unwrap();
            let t = line.trim().to_string();
            line.clear();
            t
        })
        .collect::<Vec<_>>();

    let edges = interpret_map((n, m), &map_strings);

    println!("{}", get_count_of_safezone((n, m), &edges));
}
