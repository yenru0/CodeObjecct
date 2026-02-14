use std::io::{read_to_string, stdin};

fn is_win(
    vis: &mut Vec<Vec<usize>>,
    rcum: &Vec<Vec<usize>>,
    ccum: &Vec<Vec<usize>>,
    curr: (usize, usize),
) -> bool {
    if vis[curr.0][curr.1] != usize::MAX {
        return vis[curr.0][curr.1] == 1;
    }

    let r = rcum[curr.0][curr.1];
    let c = ccum[curr.0][curr.1];

    if curr.0 == 0 && curr.1 == 0 {
        let t = r % 2 == 0;
        vis[curr.0][curr.1] = t as usize;
        return t;
    } else if curr.0 == 0 {
        let t = if c % 2 == 0 {
            r % 2 == 0 || !is_win(vis, rcum, ccum, (curr.0, curr.1 - 1))
        } else {
            r % 2 == 0
        };
        vis[curr.0][curr.1] = t as usize;
        return t;
    } else if curr.1 == 0 {
        let t = if r % 2 == 0 {
            c % 2 == 0 || !is_win(vis, rcum, ccum, (curr.0 - 1, curr.1))
        } else {
            c % 2 == 0
        };
        vis[curr.0][curr.1] = t as usize;
        return t;
    }

    if r % 2 == 0 && c % 2 == 0 {
        let t = !is_win(vis, rcum, ccum, (curr.0 - 1, curr.1))
            || !is_win(vis, rcum, ccum, (curr.0, curr.1 - 1));
        vis[curr.0][curr.1] = t as usize;
        t
    } else if c % 2 == 0 {
        let t = !is_win(vis, rcum, ccum, (curr.0, curr.1 - 1));
        vis[curr.0][curr.1] = t as usize;
        t
    } else if r % 2 == 0 {
        let t = !is_win(vis, rcum, ccum, (curr.0 - 1, curr.1));
        vis[curr.0][curr.1] = t as usize;
        t
    } else {
        false
    }
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    loop {
        let n = match iter.next() {
            Some(x) => x,
            None => {
                break;
            }
        };

        let mat = {
            let mut mat = vec![vec![0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    mat[i][j] = iter.next().unwrap();
                }
            }
            mat
        };
        let (rcum, ccum) = {
            let mut rcum = vec![vec![0; n]; n];
            let mut ccum = vec![vec![0; n]; n];

            for i in 0..n {
                rcum[i][0] = mat[i][0];
                ccum[0][i] = mat[0][i];
                for j in 1..n {
                    rcum[i][j] = rcum[i][j - 1] + mat[i][j];
                    ccum[j][i] = ccum[j - 1][i] + mat[j][i];
                }
            }

            (rcum, ccum)
        };

        let mut vis = vec![vec![usize::MAX; n]; n];

        if is_win(&mut vis, &rcum, &ccum, (n - 1, n - 1)) {
            println!("W");
        } else {
            println!("L");
        }
    }
}
