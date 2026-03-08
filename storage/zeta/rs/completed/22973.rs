use std::io::{read_to_string, stdin};

fn main() {
    let k = read_to_string(stdin())
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    let k = k.abs() as u64;
    if k == 0 {
        println!("{}", 0);
    }
    else if k % 2 == 0 {
        println!("{}", -1);
    } else {
        let mut max_i = 48;
        for i in 0..=48 {
            if k >> i == 0 {
                max_i = i;
                break;
            }
        }
        println!("{}", max_i);
    }
}
