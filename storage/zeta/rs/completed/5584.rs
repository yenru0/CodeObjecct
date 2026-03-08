use std::{
    collections::HashMap,
    io::{read_to_string, stdin},
};

fn main() {
    let temp = read_to_string(stdin()).unwrap();

    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let (w, h) = (iter.next().unwrap(), iter.next().unwrap());

    let n = iter.next().unwrap();

    let tapes = (0..n)
        .map(|_| {
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut horizontal_map = HashMap::new();
    let mut vertical_map = HashMap::new();
    let mut horionztal_pivots = vec![];
    let mut vertical_pivots = vec![];
    horionztal_pivots.push(0);
    horionztal_pivots.push(w);
    vertical_pivots.push(0);
    vertical_pivots.push(h);
    for &(x1, y1, x2, y2) in tapes.iter() {
        horionztal_pivots.push(x1);
        horionztal_pivots.push(x2);
        vertical_pivots.push(y1);
        vertical_pivots.push(y2);
    }

    horionztal_pivots.sort();
    vertical_pivots.sort();
    let mut cnt = 0;
    for pivot in horionztal_pivots.iter() {
        if !horizontal_map.contains_key(pivot) {
            horizontal_map.insert(*pivot, cnt);
            cnt += 1;
        }
    }
    let width = cnt - 1;
    let mut cnt = 0;
    for pivot in vertical_pivots.iter() {
        if !vertical_map.contains_key(pivot) {
            vertical_map.insert(*pivot, cnt);
            cnt += 1;
        }
    }
    let height = cnt - 1;

    let mut field = vec![vec![false; height]; width];

    for (x1, y1, x2, y2) in tapes.iter() {
        let (x1, y1, x2, y2) = (
            horizontal_map[x1],
            vertical_map[y1],
            horizontal_map[x2],
            vertical_map[y2],
        );

        for i in x1..x2 {
            for j in y1..y2 {
                field[i][j] = true;
            }
        }
    }

    let mut res = 0;
    let mut vis = field.clone();
    let delta = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    for i in 0..width {
        for j in 0..height {
            if vis[i][j] {
                continue;
            }

            let mut deq = vec![(i, j)];

            while !deq.is_empty() {
                let curr = deq.pop().unwrap();
                if vis[curr.0][curr.1] {
                    continue;
                } else {
                    vis[curr.0][curr.1] = true;
                }
                for (dx, dy) in delta.iter() {
                    let nxt_x = curr.0 as isize + dx;
                    if nxt_x < 0 || nxt_x >= width as isize {
                        continue;
                    }
                    let nxt_y = curr.1 as isize + dy;
                    if nxt_y < 0 || nxt_y >= height as isize {
                        continue;
                    }
                    if vis[nxt_x as usize][nxt_y as usize] {
                        continue;
                    }
                    deq.push((nxt_x as usize, nxt_y as usize));
                }
            }
            res += 1;
        }
    }

    println!("{}", res);
}
