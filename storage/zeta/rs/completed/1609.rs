use std::io::{read_to_string, stdin};

fn calc(
    board: &Vec<Vec<usize>>,
    s_row: &Vec<usize>,
    s_col: &Vec<usize>,
    p1: (usize, usize),
    p2: (usize, usize),
) -> usize {
    if p1.0 == p2.0 {
        s_row[p1.0] + s_col[p1.1] + s_col[p2.1] - 2 * board[p1.0][p1.1] - 2 * board[p2.0][p2.1]
    } else if p1.1 == p2.1 {
        s_col[p1.1] + s_row[p1.0] + s_row[p2.0] - 2 * board[p1.0][p1.1] - 2 * board[p2.0][p2.1]
    } else {
        s_row[p1.0] + s_col[p1.1] + s_row[p2.0] + s_col[p2.1]
            - 2 * board[p1.0][p1.1]
            - 2 * board[p2.0][p2.1]
            - board[p1.0][p2.1]
            - board[p2.0][p1.1]
    }
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();

    let board: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..n).map(|_| iter.next().unwrap()).collect())
        .collect();

    let mut s_row = vec![0; n];
    let mut s_col = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            let v = board[i][j];
            s_row[i] += v;
            s_col[j] += v;
        }
    }

    let mut res = 0;
    for i in 0..n {
        let mut curr = (i, i);
        let mut next = (0, 0);
        let mut local_res = 0;
        loop {
            let mut next_cost = 0;
            for i in 0..n {
                for j in 0..n {
                    if curr.0 == i && curr.1 == j {
                        continue;
                    }
                    let cost = calc(&board, &s_row, &s_col, curr, (i, j));
                    if cost > next_cost {
                        next_cost = cost;
                        next = (i, j);
                    }
                }
            }
            if next_cost > local_res {
                local_res = next_cost;
                curr = next;
            } else {
                res = res.max(local_res);
                break;
            }
        }
    }

    println!("{}", res);
}
