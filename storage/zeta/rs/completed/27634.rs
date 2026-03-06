use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    
    let mut iter = line.split_whitespace();
    
    let k: i64 = iter.next().unwrap().parse().unwrap();
    let s: i64 = iter.next().unwrap().parse().unwrap();
    let n: i64 = iter.next().unwrap().parse().unwrap();

    if n < s * k {
        println!("NO");
        return;
    }

    if k == 2 {
        if n % 2 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    } else {
        if s == 1 {
            if n <= 2 * k - 2 {
                println!("YES");
            } else if n % 2 == 0 {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            println!("YES");
        }
    }
}