use std::{
    cmp::min,
    collections::HashMap,
    io::{read_to_string, stdin},
};

fn remap(x: (i64, i64, i64, i64)) -> (i64, i64, i64, i64) {
    let (x1, y1, x2, y2) = x;
    let x = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let y = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    (x.0, y.0, x.1, y.1)
}

fn is_bound_out(xf0: i64, xf1: i64, x: i64) -> bool {
    x < xf0 || x > xf1
}

fn compensate_bounded_out(xf0: i64, xf1: i64, x: i64) -> i64 {
    if x < xf0 {
        xf0
    } else if x > xf1 {
        xf1
    } else {
        x
    }
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let plate = remap((
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ));

    let n = iter.next().unwrap() as usize;

    let chips = (0..n)
        .map(|_| {
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .map(remap)
        .collect::<Vec<_>>();
    let (h_map, width) = {
        let mut h_map: HashMap<i64, usize> = HashMap::new();

        let h_base = vec![plate.0, plate.2].into_iter();
        let h1 = chips
            .iter()
            .map(|&(x, _, _, _)| x)
            .filter(|x| !is_bound_out(plate.0, plate.2, *x));
        let h2 = chips
            .iter()
            .map(|&(_, _, x, _)| x)
            .filter(|x| !is_bound_out(plate.0, plate.2, *x));

        let mut h = h_base.chain(h1).chain(h2).collect::<Vec<_>>();
        h.sort();
        let mut cnt = 0;
        for &v in h.iter() {
            if !h_map.contains_key(&v) {
                h_map.insert(v, cnt);
                cnt += 1;
            }
        }
        (h_map, cnt)
    };

    let (v_map, height) = {
        let mut v_map: HashMap<i64, usize> = HashMap::new();

        let v_base = vec![plate.1, plate.3].into_iter();
        let v1 = chips
            .iter()
            .map(|&(_, x, _, _)| x)
            .filter(|x| !is_bound_out(plate.1, plate.3, *x));
        let v2 = chips
            .iter()
            .map(|&(_, _, _, x)| x)
            .filter(|x| !is_bound_out(plate.1, plate.3, *x));

        let mut v = v_base.chain(v1).chain(v2).collect::<Vec<_>>();
        v.sort();

        let mut cnt = 0;

        for &val in v.iter() {
            if !v_map.contains_key(&val) {
                v_map.insert(val, cnt);
                cnt += 1;
            }
        }
        (v_map, cnt)
    };

    let chips = chips
        .into_iter()
        .map(|(x1, y1, x2, y2)| {
            (
                h_map[&compensate_bounded_out(plate.0, plate.2, x1)],
                v_map[&compensate_bounded_out(plate.1, plate.3, y1)],
                h_map[&compensate_bounded_out(plate.0, plate.2, x2)],
                v_map[&compensate_bounded_out(plate.1, plate.3, y2)],
            )
        })
        .collect::<Vec<_>>();

    let mut field = vec![vec![false; height - 1]; width - 1];

    for chip in chips {
        let (x1, y1, x2, y2) = chip;

        for i in x1..x2 {
            for j in y1..y2 {
                field[i][j] = true;
            }
        }
    }

    let mut vis = field.clone();

    let delta = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut res = 0;
    for i in 0..width - 1 {
        for j in 0..height - 1 {
            if vis[i][j] {
                continue;
            }

            res += 1;
            let mut deq = vec![(i, j)];

            while !deq.is_empty() {
                let curr = deq.pop().unwrap();
                if vis[curr.0][curr.1] {
                    continue;
                } else {
                    vis[curr.0][curr.1] = true;
                }
                for dx in delta {
                    let nxt = (curr.0 as isize + dx.0, curr.1 as isize + dx.1);
                    if !(nxt.0 < 0
                        || nxt.0 >= width as isize - 1
                        || nxt.1 < 0
                        || nxt.1 >= height as isize - 1)
                    {
                        let nxt = (nxt.0 as usize, nxt.1 as usize);
                        if !field[nxt.0][nxt.1] {
                            deq.push((nxt.0 as usize, nxt.1 as usize));
                        }
                    }
                }
            }
        }
    }

    println!("{}", res);
}
