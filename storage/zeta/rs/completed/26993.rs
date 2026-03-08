use std::{
    collections::VecDeque,
    io::{read_to_string, stdin},
};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let (w, h) = (iter.next().unwrap(), iter.next().unwrap());

    let mut start = (0, 0);
    let mut dest = (0, 0);

    let field = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| {
                    let t = iter.next().unwrap();
                    if t == 2 {
                        start = (i, j);
                    } else if t == 3 {
                        dest = (i, j);
                    }
                    t
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut weight_map = vec![vec![0; w]; h];

    {
        let mut vis = vec![vec![false; w]; h];

        let mut deq = VecDeque::new();
        deq.push_back((start.clone(), 0));

        while !deq.is_empty() {
            let (curr, curr_weight) = deq.pop_front().unwrap();
            if vis[curr.0][curr.1] {
                continue;
            } else {
                vis[curr.0][curr.1] = true;
            }
            let curr_state = field[curr.0][curr.1];

            if curr_state == 4 {
                weight_map[curr.0][curr.1] += curr_weight;
            }

            for (dx, dy) in deltas {
                let nxt = (curr.0 as isize + dx, curr.1 as isize + dy);

                if nxt.0 < 0 || nxt.0 >= h as isize || nxt.1 < 0 || nxt.1 >= w as isize {
                    continue;
                }

                let nxt = (nxt.0 as usize, nxt.1 as usize);

                if field[nxt.0][nxt.1] == 1 {
                    continue;
                }
                deq.push_back((nxt, curr_weight + 1));
            }
        }
    }

    {
        let mut vis = vec![vec![false; w]; h];

        let mut deq = VecDeque::new();
        deq.push_back((dest.clone(), 0));

        while !deq.is_empty() {
            let (curr, curr_weight) = deq.pop_front().unwrap();
            if vis[curr.0][curr.1] {
                continue;
            } else {
                vis[curr.0][curr.1] = true;
            }
            let curr_state = field[curr.0][curr.1];

            if curr_state == 4 {
                weight_map[curr.0][curr.1] += curr_weight;
            }

            for (dx, dy) in deltas {
                let nxt = (curr.0 as isize + dx, curr.1 as isize + dy);

                if nxt.0 < 0 || nxt.0 >= h as isize || nxt.1 < 0 || nxt.1 >= w as isize {
                    continue;
                }

                let nxt = (nxt.0 as usize, nxt.1 as usize);

                if field[nxt.0][nxt.1] == 1 {
                    continue;
                }

                deq.push_back((nxt, curr_weight + 1));
            }
        }
    }

    let mut res = usize::MAX;
    for i in 0..h {
        for j in 0..w {
            let t = weight_map[i][j];
            if t == 0 {
                continue;
            }
            res = res.min(t);
        }
    }

    println!("{}", res);
}
