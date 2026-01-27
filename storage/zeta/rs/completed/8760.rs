use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let z = iter.next().unwrap();

    (0..z).for_each(|_| {
        let (w, k) = (iter.next().unwrap(), iter.next().unwrap());

        let w_r = w % 2;
        let k_r = k % 2;

        let res = if w_r == 0 {
            w * k / 2
        } else if k_r == 0 {
            w * k / 2
        } else {
            w * (k - 1) / 2 + (w - 1) / 2
        };

        println!("{}", res);
    });
}
