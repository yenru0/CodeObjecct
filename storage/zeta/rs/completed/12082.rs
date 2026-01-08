use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

fn convert_mapping_set(pairs: Vec<(String, String)>) -> (usize, Vec<(usize, usize)>) {
    let mut i = 0;

    let mut mapping: HashMap<String, usize> = HashMap::new();
    let pairs = pairs
        .into_iter()
        .map(|(x, y)| {
            let ix = if mapping.contains_key(&x) {
                *mapping.get(&x).unwrap()
            } else {
                let before = i;
                mapping.insert(x, i);
                i += 1;
                before
            };
            let iy = if mapping.contains_key(&y) {
                *mapping.get(&y).unwrap()
            } else {
                let before = i;
                mapping.insert(y, i);
                i += 1;
                before
            };

            (ix, iy)
        })
        .collect::<Vec<(usize, usize)>>();
    (i, pairs)
}

fn check_split_bad_horse(pairs: Vec<(String, String)>) -> bool {
    let (n, pairs) = convert_mapping_set(pairs);

    let edges = {
        let mut edges = vec![];

        for _ in 0..n {
            edges.push(vec![]);
        }

        for (i, j) in pairs {
            edges.get_mut(i).unwrap().push(j);
            edges.get_mut(j).unwrap().push(i);
        }
        edges
    };

    let mut deque = vec![];
    let mut vis = vec![0; n];
    (0..n).rev().for_each(|i| deque.push((i, 0)));
    let mut flag = true;
    while !deque.is_empty() {
        let (curr, before) = deque.pop().unwrap();

        if before == 0 {
            if vis[curr] == 0 {
                // Separate Graph
                vis[curr] = 1;
                for &nxt in edges[curr].iter() {
                    deque.push((nxt, 1));
                }
            } else {
                continue;
            }
        } else {
            let now = -before;
            if vis[curr] == 0 {
                vis[curr] = now;
                for &nxt in edges[curr].iter() {
                    deque.push((nxt, now));
                }
            } else {
                if vis[curr] == now {
                } else {
                    flag = false;
                    break;
                }
            }
        }
    }

    flag
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t = line.trim_ascii_end().parse::<usize>().unwrap();
    (0..t).for_each(|case| {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let m = line.trim_ascii_end().parse::<usize>().unwrap();
        let pairs = (0..m)
            .map(|_| {
                line.clear();
                stdin().read_line(&mut line).unwrap();
                let mut iter = line.trim().split(' ');
                (
                    iter.next().unwrap().to_string(),
                    iter.next().unwrap().to_string(),
                )
            })
            .collect::<Vec<(String, String)>>();
        println!(
            "Case #{}: {}",
            case + 1,
            if check_split_bad_horse(pairs) {
                "Yes"
            } else {
                "No"
            }
        );
    });
}
