use std::io::stdin;

fn split_by_2(s: &str) -> (usize, usize) {
    let mut iter = s.split_ascii_whitespace();
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn get_mult_count(mat1: &(usize, usize), mat2: &(usize, usize)) -> usize {
    // ensure that m1.c == m2.r
    mat1.0 * mat1.1 * mat2.1
}

fn get_mat(mats: &Vec<(usize, usize)>, from: usize, to: usize) -> (usize, usize) {
    (mats[from].0, mats[to].1)
}

fn get_minimum_mult_count(n: usize, mats: Vec<(usize, usize)>) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n]; n];

    for interval in 1..n {
        for i in 0..n - interval {
            dp[i][i + interval] = *(0..interval)
                .map(|split| {
                    dp[i][i + split]
                        + dp[i + split + 1][i + interval]
                        + get_mult_count(
                            &get_mat(&mats, i, i + split),
                            &get_mat(&mats, i + split + 1, i + interval),
                        )
                })
                .collect::<Vec<usize>>()
                .iter()
                .min()
                .unwrap();
        }
    }

    dp[0][n - 1]
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mats: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            split_by_2(&line)
        })
        .collect();

    println!("{}", get_minimum_mult_count(n, mats));
}
