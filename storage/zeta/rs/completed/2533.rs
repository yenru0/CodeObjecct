use std::{io::stdin, usize};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
    Undetermined,
}

#[derive(Clone)]
struct ColorRecord {
    color: Color,
    cnt_white: usize,
    cnt_black: usize,
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];

    let _ = (0..(n - 1)).for_each(|_| {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        let (u, v) = (
            iter.next().unwrap().parse::<usize>().unwrap() - 1,
            iter.next().unwrap().parse::<usize>().unwrap() - 1,
        );

        tree[u].push(v);
        tree[v].push(u);
    });

    println!("{}", minimum_early_adapters(n, &tree));
}

enum StackInst {
    ParseLower { parent: usize, current: usize },
    UpdateCurrent { parent: usize, current: usize },
}

fn minimum_early_adapters(n: usize, tree: &Vec<Vec<usize>>) -> usize {
    let mut records: Vec<ColorRecord> = vec![
        ColorRecord {
            color: Color::Undetermined,
            cnt_white: 0,
            cnt_black: 0
        };
        n
    ];

    let mut deque: Vec<StackInst> = vec![]; //
    deque.push(StackInst::ParseLower {
        parent: usize::MAX,
        current: 0,
    });
    while !deque.is_empty() {
        let inst = deque.pop().unwrap();

        match inst {
            StackInst::ParseLower { parent, current } => {
                deque.push(StackInst::UpdateCurrent { parent, current });
                for &v in tree[current].iter() {
                    if v != parent {
                        deque.push(StackInst::ParseLower {
                            parent: current,
                            current: v,
                        });
                    }
                }
            }
            StackInst::UpdateCurrent { parent, current } => {
                let mut is_white = true;
                let mut cnt_black: usize = 0;
                let mut cnt_white: usize = 0;
                for &v in tree[current].iter() {
                    if v != parent {
                        cnt_black += records[v].cnt_black;
                        cnt_white += records[v].cnt_white;
                        if records[v].color == Color::White {
                            is_white = false;
                        }
                    }
                }
                let record = records.get_mut(current).unwrap();
                record.cnt_black = cnt_black;
                record.cnt_white = cnt_white;
                if is_white {
                    record.color = Color::White;
                    record.cnt_white += 1;
                } else {
                    record.color = Color::Black;
                    record.cnt_black += 1;
                }
            }
        }
    }
    records[0].cnt_black
}
