use std::io::{stdin, stdout, BufWriter};
use std::io::{BufRead, Write};

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim_end().parse().unwrap();
    line.clear();
    for _ in 0..n {
        stdin().read_line(&mut line).unwrap();
        let s = line
            .trim_end()
            .split_ascii_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .sum::<u16>();
        line.clear();

        //println!("{}", s);
        writeln!(out, "{}", s).unwrap();
    }
}
