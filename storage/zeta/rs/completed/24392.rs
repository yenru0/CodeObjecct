use std::io::stdin;

enum Pane {
    Normal,
    Reinforced,
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line.trim().split(' ').map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let panes = (0..n)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            line.split(' ')
                .map(|x| match x.chars().next().unwrap() {
                    '0' => Pane::Normal,
                    '1' => Pane::Reinforced,
                    _ => unreachable!(),
                })
                .collect::<Vec<Pane>>()
        })
        .collect::<Vec<Vec<Pane>>>()
        .into_iter()
        .rev()
        .collect::<Vec<Vec<Pane>>>();

    /*
       Calc All Possible Paths
    */

    let mut dp = vec![vec![0u64; m]; n];

    panes[0]
        .iter()
        .enumerate()
        .for_each(|(i, pane)| match pane {
            Pane::Normal => dp[0][i] = 0,
            Pane::Reinforced => dp[0][i] = 1,
        });

    for i in 1..n {
        for j in 1..(m - 1) {
            dp[i][j] = match panes[i][j] {
                Pane::Normal => 0,
                Pane::Reinforced => {
                    let left = dp[i - 1][j - 1];
                    let right = dp[i - 1][j + 1];
                    let centre = dp[i - 1][j];
                    (left + right + centre) % 1_000_000_007
                }
            };
        }
        dp[i][0] = match panes[i][0] {
            Pane::Normal => 0,
            Pane::Reinforced => (dp[i - 1][0] + dp[i - 1][1]) % 1_000_000_007,
        };
        dp[i][m - 1] = match panes[i][m - 1] {
            Pane::Normal => 0,
            Pane::Reinforced => (dp[i - 1][m - 2] + dp[i - 1][m - 1]) % 1_000_000_007,
        }
    }

    let res = dp[n - 1]
        .iter()
        .fold(0u64, |acc, x| (acc + x) % 1_000_000_007);
    println!("{}", res);
}
