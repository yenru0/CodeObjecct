use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let s = iter.next().unwrap();
    let n = iter.next().unwrap();

    let arr = (0..n).map(|_| iter.next().unwrap());

    let mut seat_stat = vec![false; s];

    for i in arr {
        seat_stat[i - 1] = true;
    }

    let mut cnt = 0;
    for i in 0..s {
        if !seat_stat[i] {
            let left = if i == 0 { s - 1 } else { i - 1 };
            let right = (i + 1) % s;

            if !seat_stat[left] && !seat_stat[right] {
                seat_stat[i] = true;
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
