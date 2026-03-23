use std::io::{read_to_string, stdin};

fn cantor_string(line: &mut Vec<char>, lo: usize, n: usize) {
    if n == 0 {
        return;
    }
    let x = 3usize.pow(n as u32 - 1);
    for i in lo + x..lo + 2 * x {
        line[i] = ' ';
    }
    cantor_string(line, lo, n - 1);
    cantor_string(line, lo + 2 * x, n - 1);
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    loop {
        let n = match iter.next() {
            Some(n) => n,
            None => {
                break;
            }
        };
        let x = 3usize.pow(n as u32);
        let mut line = vec!['-'; x];
        cantor_string(&mut line, 0, n);
        println!("{}", line.iter().collect::<String>());
    }
}
