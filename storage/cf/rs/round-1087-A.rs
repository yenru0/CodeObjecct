use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let t = iter.next().unwrap();
    (0..t).for_each(|_| {
        let (n, mut c, mut k) = (
            // number, combat power, k of flip flop
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        let mut arr = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<_>>();
        arr.sort();
        loop {
            let key = arr.binary_search(&c);
            match key {
                Ok(idx) => {
                    c += arr[idx];
                    arr.remove(idx);
                }
                Err(idx) => {
                    if idx == 0 {
                        break;
                    } else {
                        let delta = c - arr[idx - 1];
                        let consume = if k >= delta { delta } else { k };
                        k -= consume;
                        c += consume + arr[idx - 1];
                        arr.remove(idx - 1);
                    }
                }
            }
        }
        println!("{}", c);
    });
}
