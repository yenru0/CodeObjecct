use std::{cmp::max, io::stdin};

fn get_value(r: i32, c: i32) -> i32 {
    let d = max(r.abs(), c.abs());

    if d == 0 {
        return 1;
    }

    let prev_max = (2 * d - 1) * (2 * d - 1);

    if c == d && r < d {
        prev_max + (d - r)
    } else if r == -d {
        prev_max + 2 * d + (d - c)
    } else if c == -d {
        prev_max + 4 * d + (r + d)
    } else {
        prev_max + 6 * d + c + d
    }
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut iter = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let (r1, c1, r2, c2) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let mut spiral = vec![vec![0; (c2 - c1 + 1) as usize]; (r2 - r1 + 1) as usize];
    let mut max_v = 0;
    for i in r1..=r2 {
        for j in c1..=c2 {
            let v = get_value(i, j);
            max_v = max(max_v, v);
            spiral[(i - r1) as usize][(j - c1) as usize] = v;
        }
    }

    let offset = max_v.to_string().len();

    for i in 0..(r2 - r1 + 1) as usize {
        for j in 0..(c2 - c1) as usize {
            print!("{:width$} ", spiral[i][j], width = offset);
        }
        print!("{:width$}", spiral[i][(c2 - c1) as usize], width = offset);
        println!();
    }
}
