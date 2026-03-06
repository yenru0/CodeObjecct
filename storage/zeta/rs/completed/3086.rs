use std::{
    io::{read_to_string, stdin},
    usize,
};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let (n, m) = {
        let (x, y) = (iter.next().unwrap(), iter.next().unwrap());
        if x >= y { (x, y) } else { (y, x) }
    };

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 1..=m {
        dp[1][i] = i;
        dp[i][i] = 1;
    }

    for i in 1..=n {
        dp[i][1] = i;
    }

    for i in 2..=n {
        for j in 2..=m {
            if dp[i][j] > 0 {
                continue;
            }
            let mut a = usize::MAX;
            for x in 1..=i / 2 {
                a = a.min(dp[i - x][j] + dp[x][j]);
            }
            for x in 1..=j / 2 {
                a = a.min(dp[i][j - x] + dp[i][x]);
            }
            dp[i][j] = a;
            dp[j][i] = a;
        }
    }

    println!("{}", dp[n][m]);
}
