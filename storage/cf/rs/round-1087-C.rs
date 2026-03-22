use std::io::{Write, stdin, stdout};

fn client(n: usize) {
    let mut line = String::new();
    for i in 0..n - 1 {
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        println!("? {} {}", left, right);
        stdout().flush().unwrap();
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let q = line.trim().parse::<usize>().unwrap();
        if q == 1 {
            println!("! {}", left);
            stdout().flush().unwrap();
            return;
        }
    }

    println!("? {} {}", 2 * n - 1, 2 * n - 2);
    stdout().flush().unwrap();
    line.clear();
    stdin().read_line(&mut line).unwrap();
    let q = line.trim().parse::<usize>().unwrap();
    if q == 1 {
        println!("! {}", 2 * n - 1);
        stdout().flush().unwrap();
        return;
    }
    println!("? {} {}", 2 * n - 1, 2 * n - 3);
    stdout().flush().unwrap();
    line.clear();
    stdin().read_line(&mut line).unwrap();
    let q = line.trim().parse::<usize>().unwrap();
    if q == 1 {
        println!("! {}", 2 * n - 1);
        stdout().flush().unwrap();
        return;
    }
    println!("! {}", 2 * n);
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t = line.trim().parse::<usize>().unwrap();
    (0..t).for_each(|_| {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let n = line.trim().parse::<usize>().unwrap();
        client(n);
    });
}
