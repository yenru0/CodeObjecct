use std::cmp::max;
use std::io::stdin;

fn longest_sub_regular_bracket_string(s: String) -> usize {
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];

    for dx in 1..n {
        for left in 0..n - dx {
            let right = left + dx;
            if (s.as_bytes()[left] == b'(' && s.as_bytes()[right] == b')')
                || (s.as_bytes()[left] == b'[' && s.as_bytes()[right] == b']')
            {
                dp[left][right] = dp[left + 1][right - 1] + 2;
            }
            for k in left..right {
                dp[left][right] = max(dp[left][right], dp[left][k] + dp[k + 1][right]);
            }
        }
    }

    dp[0][n - 1]
}

fn main() {
    let mut line = String::new();

    loop {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        if line.starts_with("end") {
            break;
        }

        let s = line.trim().to_string();

        println!("{}", longest_sub_regular_bracket_string(s));
    }
}
