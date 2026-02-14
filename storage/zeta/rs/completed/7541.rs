use std::{io::stdin, u8};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();

    (1..=n).for_each(|case| {
        line.clear();
        stdin().read_line(&mut line).unwrap();

        let mut id = line
            .trim()
            .chars()
            .map(|x| match x {
                '?' => u8::MAX,
                _ => x as u8 - '0' as u8,
            })
            .collect::<Vec<u8>>();

        let mut idx = 0;
        let mut k: u64 = 0;
        let len = id.len();
        let r_s: u64 = id
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                if x == u8::MAX {
                    idx = i;
                    k = [9, 3, 7][(len - i - 1) % 3];
                    0
                } else {
                    let t = x as u64 * [9, 3, 7][(len - i - 1) % 3] as u64;
                    t
                }
            })
            .sum();
        let mut x = 0;
        for i in 0..10 {
            if (i * k + r_s) % 10 == 0 {
                x = i;
                break;
            }
        }
        id[idx] = x as u8;
        println!(
            "Scenario #{}:\n{}\n",
            case,
            id.iter()
                .map(|&x| ('0' as u8 + x as u8) as char)
                .collect::<String>()
        );
    });
}
