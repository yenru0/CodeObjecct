use std::{
    cmp::max,
    io::{read_to_string, stdin},
};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    let (n, m, t, x, y) = (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    );

    let points = (0..m)
        .map(|_| iter.next().unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut logs = (0..y)
        .map(|_| {
            (
                iter.next().unwrap().parse::<i64>().unwrap(), // time
                iter.next().unwrap().parse::<usize>().unwrap() - 1, // participant
                iter.next().unwrap().parse::<usize>().unwrap() - 1, // problem num
                match iter.next().unwrap().chars().next().unwrap() {
                    'o' => 0, // open
                    'c' => 1, // corr
                    'i' => 2, // incr
                    _ => 0,
                },
            )
        })
        .collect::<Vec<_>>();
    logs.sort_by_key(|x| x.0); // sort by time
    let mut open_times = vec![vec![0; m]; n];
    let mut corr_times = vec![vec![-1; m]; n];
    let mut wrong_trials = vec![vec![0; m]; n];
    for log in logs {
        let (time, part, prob, delta) = log;
        match delta {
            0 => {
                open_times[part][prob] = time;
            }
            1 => {
                corr_times[part][prob] = time;
            }
            2 => {
                wrong_trials[part][prob] += 1;
            }
            _ => {}
        }
    }

    for part in 0..n {
        let mut s = 0;
        for prob in 0..m {
            if corr_times[part][prob] == -1 {
                continue; // 0
            }
            s += max(
                points[prob]
                    - (corr_times[part][prob] - open_times[part][prob])
                    - 120 * wrong_trials[part][prob],
                x,
            );
        }
        println!("{}", s);
    }
}
