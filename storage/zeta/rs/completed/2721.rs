use std::io::{read_to_string, stdin};

fn W(n: usize) -> usize {
    // W(1) = 1 * T(2) = 3
    // W(2) = 1 * T(2) + 2 * T(3) = 12 + 3 = 15
    // W(n + 1) - W(n) = n * T(n + 1) = n * (n + 1) * (n + 2) / 2
    if n == 1 {
        3
    } else {
        n * (n + 1) * (n + 2) / 2 + W(n - 1)
    }
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    (0..iter.next().unwrap()).for_each(|_| {
        let n = iter.next().unwrap();
        println!("{}", W(n));
    });
}
