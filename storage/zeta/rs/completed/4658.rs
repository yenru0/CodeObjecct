use std::cmp::max;
use std::io::{read_to_string, stdin};
enum Oprt {
    Unload(usize, usize),
    Load(usize, usize, usize),
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    loop {
        let tries = (0..6)
            .map(|_| {
                vec![
                    iter.next().unwrap().parse::<u64>().unwrap(),
                    iter.next().unwrap().parse::<u64>().unwrap(),
                    iter.next().unwrap().parse::<u64>().unwrap(),
                ]
            })
            .collect::<Vec<Vec<u64>>>();

        let mut maxima = 0u64;
        let mut used = [false; 6];
        let mut side = [0u64; 6];
        let mut outer = [0u64; 6];
        let mut deq: Vec<Oprt> = vec![];
        deq.push(Oprt::Unload(0, 0));
        deq.push(Oprt::Load(0, 0, 0));
        deq.push(Oprt::Unload(0, 0));
        deq.push(Oprt::Load(0, 0, 1));
        deq.push(Oprt::Unload(0, 2));
        deq.push(Oprt::Load(0, 0, 2));

        while !deq.is_empty() {
            let oprt = deq.pop().unwrap();

            match oprt {
                Oprt::Load(depth, idx, face) => {
                    if depth == 5 {
                        let next = tries[idx][face];
                        outer[depth] = tries[idx][(face + 1) % 3];
                        if next == side[0] {
                            maxima = max(maxima, outer.iter().sum());
                        }
                    } else {
                        let next = if depth == 0 {
                            let before = tries[idx][(face + 2) % 3];
                            let next = tries[idx][face];
                            used[idx] = true;
                            side[depth] = before;
                            side[depth + 1] = next;
                            outer[depth] = tries[idx][(face + 1) % 3];
                            next
                        } else {
                            let next = tries[idx][face];
                            used[idx] = true;
                            side[depth + 1] = next;
                            outer[depth] = tries[idx][(face + 1) % 3];
                            next
                        };

                        for i in 0..6 {
                            if !used[i] {
                                for j in 0..3 {
                                    if tries[i][j] == next {
                                        deq.push(Oprt::Unload(depth + 1, i));
                                        deq.push(Oprt::Load(depth + 1, i, (j + 1) % 3));
                                    }
                                }
                            }
                        }
                    }
                }
                Oprt::Unload(depth, idx) => {
                    used[idx] = false;
                }
            }
        }
        if maxima != 0 {
            println!("{}", maxima);
        } else {
            println!("none");
        }

        if iter.next().unwrap().starts_with('$') {
            break;
        }
    }
}
