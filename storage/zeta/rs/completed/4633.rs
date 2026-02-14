use std::io::{read_to_string, stdin};

fn decrypt(x: u64, s: Vec<char>, p: Vec<char>, c: Vec<char>) -> Vec<char> {
    let size = s.len();
    let n = c.len();
    let mut m: Vec<char> = vec!['*'; n];

    let d = ((n as f64).powf(1.5) as u64 + x) as usize % n;
    for i in 0..size {
        if s[i] == c[d] {
            m[d] = p[i];
            break;
        }
    }
    for i in 0..n - 1 {
        let mut pp = 0;
        let mut ps = 0;
        let j = (d + n - i - 1) % n;
        let j_nxt = (d + n - i) % n;
        for jj in 0..size {
            if m[j_nxt] == s[jj] {
                ps = jj;
                break;
            }
        }
        for jj in 0..size {
            if c[j] == s[jj] {
                pp = jj;
                break;
            }
        }

        m[j] = p[pp ^ ps];
    }

    m
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    loop {
        let x = iter.next().unwrap().parse::<u64>().unwrap();
        if x == 0 {
            break;
        }

        let s = iter.next().unwrap().chars().collect();
        let p = iter.next().unwrap().chars().collect();
        let c = iter.next().unwrap().chars().collect();
        let msg = decrypt(x, s, p, c);
        println!("{}", msg.iter().collect::<String>());
    }
}
