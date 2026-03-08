use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let (mut h, mut m, mut s) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );
    let cooking_time = iter.next().unwrap();

    for _ in 0..cooking_time {
        s += 1;
        if s == 60 {
            s = 0;
            m += 1;
        }
        if m == 60 {
            m = 0;
            h += 1;
        }
        if h == 24 {
            h = 0;
        }
    }

    println!("{} {} {}", h, m, s);
}
