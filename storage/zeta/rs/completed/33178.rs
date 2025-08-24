use std::io::stdin;

fn main() {
    let n = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    println!("{}", n / 10);
}
