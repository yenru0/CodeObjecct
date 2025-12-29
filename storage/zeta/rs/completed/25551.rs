use std::{
    cmp::min,
    io::{stdin, Read},
};

fn main() {
    let mut line = String::new();
    stdin().read_to_string(&mut line).unwrap();
    let mut iter = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let (mw, mb, tw, tb, pw, pb) = {
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let s1 = min(tw, min(mb, pb));
    let s2 = min(tb, min(pw, mw));

    
    let res = if s1 == s2 {
        s1 + s2
    } else if s1 > s2 {
        2 * s2 + 1
    } else {
        2 * s1 + 1
    };

    println!("{}", res);
}
