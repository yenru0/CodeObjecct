use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let d = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut bcoef: (usize, usize) = (1, 0);
    let mut coef: (usize, usize) = (0, 1);

    for _ in 0..d - 2 {
        let temp = coef;
        coef = (coef.0 + bcoef.0, coef.1 + bcoef.1);
        bcoef = temp;
    }
    let mut res = (0, 0);
    'L1: for i in 1..k / coef.0 {
        for j in i..k / coef.1 {
            if coef.0 * i + coef.1 * j == k {
                res = (i, j);
                break 'L1;
            }
        }
    }

    println!("{}\n{}", res.0, res.1);
}
