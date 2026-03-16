use std::io::{read_to_string, stdin};

fn conv(s: &str) -> Vec<usize> {
    let mut v = vec![0; 10];
    for ch in s.chars() {
        v[(ch as usize - '0' as usize) as usize] += 1;
    }
    v
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    (0..iter.next().unwrap().parse::<usize>().unwrap()).for_each(|_| {
        let s = iter.next().unwrap().trim();
        let cnts = conv(s);
    })
}
