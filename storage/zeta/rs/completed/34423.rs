use std::cmp::max;
use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let (t0, t1, t2) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let ts = [[0, t0, t1], [t0, 0, t2], [t1, t2, 0]];

    let n = iter.next().unwrap() as usize;

    let pp = {
        let mut pp = vec![vec![0; 3]; n];

        for i in 0..3 {
            for j in 0..n {
                pp[j][i] = iter.next().unwrap();
            }
        }

        pp
    };

    let mut dp = vec![vec![0; 3]; n];

    dp[0][0] = pp[0][0];
    dp[0][1] = pp[0][1];
    dp[0][2] = pp[0][2];

    for i in 1..n {
        dp[i][0] = max(
            dp[i - 1][0] + pp[i][0],
            max(
                dp[i - 1][1] + pp[i][0] - ts[1][0],
                dp[i - 1][2] + pp[i][0] - ts[2][0],
            ),
        );

        dp[i][1] = max(
            dp[i - 1][1] + pp[i][1],
            max(
                dp[i - 1][0] + pp[i][1] - ts[0][1],
                dp[i - 1][2] + pp[i][1] - ts[2][1],
            ),
        );

        dp[i][2] = max(
            dp[i - 1][2] + pp[i][2],
            max(
                dp[i - 1][0] + pp[i][2] - ts[0][2],
                dp[i - 1][1] + pp[i][2] - ts[1][2],
            ),
        );
    }

    let max_cost = dp[n - 1].iter().max().unwrap();

    println!("{}", max_cost);
}
