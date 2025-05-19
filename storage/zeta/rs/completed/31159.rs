use std::io::stdin;

fn solve_max_pair_matching(n: usize, vs: &Vec<(i64, i64)>) -> i64 {
    let mut linear_scale: Vec<i64> = vs.iter().map(|&x| x.0 + x.1).collect();

    let height_scale: Vec<i64> = vs
        .iter()
        .zip(linear_scale.iter())
        .map(|(&(a, _), &l)| (2 * a - l).abs())
        .collect();

    linear_scale.sort();

    let sum_height = height_scale.iter().sum::<i64>();
    let max_diff_linear = (0..n)
        .map(|i| linear_scale[2 * n - i - 1] - linear_scale[i])
        .sum::<i64>();

    (sum_height + max_diff_linear) / 2
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    drop(line);
    let vs = (0..(2 * n))
        .map(|_| {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            let v: Vec<i64> = line
                .trim()
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (v[0], v[1])
        })
        .collect::<Vec<(i64, i64)>>();

    println!("{}", solve_max_pair_matching(n, &vs));
}
