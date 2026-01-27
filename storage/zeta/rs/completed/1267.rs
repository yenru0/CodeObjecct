use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let times_call = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<usize>>();

    let y: usize = times_call
        .iter()
        .map(|x| {
            let d = x / 30;
            d * 10 + 10
        })
        .sum();
    let m: usize = times_call
        .iter()
        .map(|x| {
            let d = x / 60;
            d * 15 + 15
        })
        .sum();

    if y < m {
        println!("Y {}", y);
    } else if y > m {
        println!("M {}", m);
    } else {
        println!("Y M {}", y);
    }
}
