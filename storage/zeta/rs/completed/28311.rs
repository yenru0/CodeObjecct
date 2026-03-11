use std::{collections::VecDeque, io::stdin, iter::zip, usize};

fn next_line(line: &mut String) {
    line.clear();
    stdin().read_line(line).unwrap();
}

fn line_to_iter(line: &String) -> impl Iterator<Item = usize> + '_ {
    line.trim_ascii_end()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
}

const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_cost_field(
    r: usize,
    c: usize,
    pos: (usize, usize),
    field: &Vec<Vec<char>>,
) -> Vec<Vec<usize>> {
    let mut deq = VecDeque::new();

    deq.push_back(((pos.0, pos.1), 0));

    let mut cost_field = vec![vec![usize::MAX; c]; r];

    while !deq.is_empty() {
        let ((curr_r, curr_c), curr_cost) = deq.pop_front().unwrap();

        if cost_field[curr_r][curr_c] == usize::MAX {
            cost_field[curr_r][curr_c] = curr_cost;
        } else {
            continue;
        }
        for &(dr, dc) in DELTAS.iter() {
            let (nxt_r, nxt_c) = (curr_r as isize + dr, curr_c as isize + dc);
            if nxt_r < 0 || nxt_r >= r as isize {
                continue;
            }
            if nxt_c < 0 || nxt_c >= c as isize {
                continue;
            }
            let (nxt_r, nxt_c) = (nxt_r as usize, nxt_c as usize);
            if field[nxt_r][nxt_c] == 'W' {
                continue;
            }
            deq.push_back(((nxt_r, nxt_c), curr_cost + 1));
        }
    }
    cost_field
}

fn solve(
    r: usize,
    c: usize,
    dest_r: usize,
    dest_c: usize,
    field: &Vec<Vec<char>>,
    poses: &Vec<(usize, usize)>,
) -> (usize, usize) {
    let dst_cost_field = get_cost_field(r, c, (dest_r, dest_c), field);
    let pos_cost_fields = poses
        .iter()
        .map(|&pos| get_cost_field(r, c, pos, field))
        .collect::<Vec<_>>();

    let orig_values = poses
        .iter()
        .map(|&(r, c)| dst_cost_field[r][c])
        .collect::<Vec<_>>();
    let orig_res = orig_values.iter().sum::<usize>();
    let mut value_sum = 0;

    for i in 0..r {
        for j in 0..c {
            if field[i][j] == 'W' {
                let cands = DELTAS
                    .iter()
                    .map(|&(dr, dc)| (i as isize + dr, j as isize + dc))
                    .filter(|&(nxt_r, nxt_c)| {
                        nxt_r >= 0
                            && nxt_r < r as isize
                            && nxt_c >= 0
                            && nxt_c < c as isize
                            && field[nxt_r as usize][nxt_c as usize] != 'W'
                    })
                    .map(|x| (x.0 as usize, x.1 as usize))
                    .collect::<Vec<_>>();
                if cands.len() <= 1 {
                    continue;
                }

                //

                let min_from_dst = cands
                    .iter()
                    .map(|&(nr, nc)| dst_cost_field[nr][nc])
                    .min()
                    .unwrap();

                if min_from_dst == usize::MAX {
                    continue;
                }
                let min_from_poses = pos_cost_fields
                    .iter()
                    .map(|f| cands.iter().map(|&(nr, nc)| f[nr][nc]).min().unwrap());

                let rep_value = zip(&orig_values, min_from_poses)
                    .into_iter()
                    .map(|(&ov, mv)| ov.min(mv + min_from_dst + 2))
                    .sum::<usize>();

                if rep_value < orig_res {
                    let rep_delta = orig_res - rep_value;
                    value_sum += rep_delta;
                }
            }
        }
    }
    (orig_res, value_sum)
}

fn main() {
    let mut line = String::new();
    next_line(&mut line);
    let t = line.trim_ascii_end().parse::<usize>().unwrap();
    (0..t).for_each(|_| {
        next_line(&mut line);
        let (r, c, n, dest_r, dest_c) = {
            let mut iter = line_to_iter(&line);
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
            )
        };

        let poses = (0..n)
            .map(|_| {
                next_line(&mut line);
                let mut iter = line_to_iter(&line);
                (iter.next().unwrap() - 1, iter.next().unwrap() - 1)
            })
            .collect::<Vec<_>>();

        let field = (0..r)
            .map(|_| {
                next_line(&mut line);
                line.trim_ascii_end().chars().collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let (orig_res, value_sum) = solve(r, c, dest_r, dest_c, &field, &poses);
        println!("{} {}", orig_res, value_sum);
    });
}
