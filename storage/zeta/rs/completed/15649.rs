use std::io::{BufWriter, stdin, Write};
enum Operation {
    VisIn,
    VisOut
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut out = BufWriter::new(std::io::stdout());
    let mut iter = line
        .trim()
        .split(' ')
        .map(|x| x.trim().parse::<u64>().unwrap());

    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut vis = vec![false; n as usize];
    let mut arr: Vec<u64> = vec![];
    let mut stack: Vec<(Operation, u64)> = vec![];

    for i in (0..n).rev() {
        stack.push((Operation::VisOut, i));
        stack.push((Operation::VisIn, i));
    }

    while !stack.is_empty() {
        let item = stack.pop().unwrap();
        match item.0 {
            Operation::VisIn => {
                arr.push(item.1);
                if arr.len() == m as usize {
                    for &val in &arr {
                        write!(out, "{} ", val + 1).unwrap();
                    }
                    writeln!(out, "").unwrap();
                    continue;
                }
                let idx = item.1 as usize;

                vis[idx] = true;
                for j in (0..n).rev() {
                    if !vis[j as usize] {
                        stack.push((Operation::VisOut, j));
                        stack.push((Operation::VisIn, j));
                    }
                }
            }
            Operation::VisOut => {
                let idx = item.1 as usize;
                vis[idx] = false;
                arr.pop();
            }
        }

    }
}
