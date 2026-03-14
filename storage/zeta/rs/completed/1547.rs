use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();

    let mut cups = vec![false; 3];
    cups[0] = true;

    (0..n).for_each(|_| {
        let (x, y) = (iter.next().unwrap() - 1, iter.next().unwrap() - 1);

        let temp = cups[y];
        cups[y] = cups[x];
        cups[x] = temp;
    });
    println!(
        "{}",
        if cups[0] {
            1
        } else if cups[1] {
            2
        } else {
            3
        }
    )
}
