use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let t = iter.next().unwrap();
    (0..t).for_each(|_| {
        let n = iter.next().unwrap() as usize;
        let arr = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<_>>();

        // if n == 1 return 0
        for i in 0..n {
            if i == n - 1 {
                print!("{} ", 0);
                continue;
            }

            let key = arr[i];

            let mut cnt_lo = 0;
            let mut cnt_hi = 0;

            for &e in arr[(i + 1)..n].iter() {
                if e > key {
                    cnt_hi += 1;
                } else if e < key {
                    cnt_lo += 1;
                }
            }
            print!("{} ", std::cmp::max(cnt_lo, cnt_hi));
        }
        println!();
    });
}
