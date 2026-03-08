use std::{io::read_to_string, io::stdin};

// 1 2 3 4 5
// 0 1 3
//

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (n, k) = (iter.next().unwrap(), iter.next().unwrap());
    let arr = {
        let mut arr = vec![];
        let org = (0..n).map(|_| iter.next().unwrap() % 2).collect::<Vec<_>>();
        let mut cnt = 0;
        for ele in org {
            if ele == 0 {
                arr.push(cnt);
                cnt = 0;
            } else {
                cnt += ele;
            }
        }
        arr
    };

    let cum_arr = {
        let mut carr = vec![0];
        for ele in arr.iter() {
            carr.push(carr.last().unwrap() + ele);
        }
        carr
    };

    let mut lo = 1;
    let mut hi = 1;
    let mut res = 0;
    while hi < cum_arr.len() {
        let key = cum_arr[hi] - cum_arr[lo];
        let len = hi - lo + 1;
        if key > k {
            lo += 1;
        } else {
            res = res.max(len);
            hi += 1;
        }
    }
    println!("{}", res);
}
