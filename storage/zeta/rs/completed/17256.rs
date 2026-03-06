use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line.trim().split(' ').map(|s| s.parse::<usize>().unwrap());
    let a = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );
    line.clear();
    stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split(' ').map(|s| s.parse::<usize>().unwrap());
    let c = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let b = (c.0 - a.2, c.1 / a.1, c.2 - a.0);

    println!("{} {} {}", b.0, b.1, b.2);
}
