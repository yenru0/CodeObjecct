use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let total_expected_happinesses: i64 = (0..n).map(|_| iter.next().unwrap()).sum();

    let res = if total_expected_happinesses + n >= m {
        0
    } else {
        let actual_non_happy_day = m - (total_expected_happinesses + n);
        let d = actual_non_happy_day / (n + 1);
        let remain = actual_non_happy_day % (n + 1);

        if remain > 0 {
            remain * ((d + 1) * (d + 2) * (2 * d + 3) / 6)
                + (n + 1 - remain) * (d * (d + 1) * (2 * d + 1) / 6)
        } else {
            (d * (d + 1) * (2 * d + 1) / 6) * (n + 1)
        }
    };

    println!("{}", res);
}
