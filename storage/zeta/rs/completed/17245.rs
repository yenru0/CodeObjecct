use std::io::{read_to_string, stdin};

fn get_count_available_computers(field: &Vec<u64>, height: u64) -> u64 {
    field
        .iter()
        .map(|&x| if x <= height { x } else { height })
        .sum()
}

fn get_thresold(n: usize, field: &Vec<u64>) -> u64 {
    let mut lo = 0;
    let mut hi = 10_000_000;
    let total = get_count_available_computers(field, hi);
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        let cnt = get_count_available_computers(field, mid);
        if 2 * cnt >= total {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let cnt_lo = get_count_available_computers(field, lo);
    if 2 * cnt_lo >= total {
        lo
    } else {
        hi
    }
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let mut field = (0..n * n)
        .map(|_| iter.next().unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    field.sort();
    println!("{}", get_thresold(n, &field))
}
