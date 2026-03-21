use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let p = (0..n).map(|_| iter.next().unwrap());

    let flg = p.fold(0, |acc, x| acc ^ x);

    if flg == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
