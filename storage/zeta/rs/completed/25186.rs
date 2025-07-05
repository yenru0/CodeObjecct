use std::io::stdin;

fn is_happy(clothes: &Vec<u64>) -> bool {
    let sum = clothes.iter().sum::<u64>();
    if sum == 1 {
        return true;
    }
    let max = clothes.iter().max().unwrap();

    sum >= max * 2
}

fn main() {
    let mut lines = stdin().lines();
    lines.next();

    let clothes: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    println!(
        "{}",
        if is_happy(&clothes) {
            "Happy"
        } else {
            "Unhappy"
        }
    )
}
