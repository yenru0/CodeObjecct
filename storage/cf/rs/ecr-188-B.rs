use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let t = iter.next().unwrap();

    (0..t).for_each(|_| {
        let n = iter.next().unwrap();
        let arr = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<_>>();
        let mut cnt = 0;
        let mut before_most = 0;
        for i in 0..n {
            if arr[i] > before_most {
                cnt += 1;
                before_most = arr[i];
            } else if arr[i] == before_most {
                cnt += 1;
            }
        }
        println!("{}", cnt);
    });
}
