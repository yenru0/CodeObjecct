use std::io;
use std::io::Write;

fn mutual_difference(v: &Vec<i64>) -> i64 {
    let mut s: i64 = 0;
    for a in v {
        for b in v {
            s += (a - b).abs();
        }
    }
    return s;
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut line = String::new();
    let mut _trash = String::new();
    io::stdin().read_line(&mut _trash).expect("err");
    io::stdin().read_line(&mut line).expect("err");

    let v = line
        .trim()
        .split(' ')
        .map(|it| it.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    writeln!(out, "{}", mutual_difference(&v)).expect("err");
}
