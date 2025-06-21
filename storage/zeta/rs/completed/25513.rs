use std::collections::VecDeque;
use std::io;

fn get_from_board(board: &Vec<Vec<i8>>, r: i8, c: i8) -> i8 {
    board[r as usize][c as usize]
}

fn get_min_search_cnt(board: &Vec<Vec<i8>>, pos: (i8, i8)) -> isize {
    let mut deq: VecDeque<((i8, i8), isize, i8)> = VecDeque::new();

    let init_state = if get_from_board(&board, pos.0, pos.1) == 1 {
        1
    } else {
        0
    };

    let mut gvis = vec![vec![false; 6]; 25];
    deq.push_back((pos, 0, init_state));

    while deq.is_empty() == false {
        let (now, cnt, state) = deq.pop_front().unwrap();

        gvis[(now.0 * 5 + now.1) as usize][state as usize] = true;

        for (i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let (r, c) = (now.0 + i, now.1 + j);
            if r < 0 || r >= 5 || c < 0 || c >= 5 {
                continue;
            }
            let ahead = get_from_board(&board, r, c);
            if ahead == -1 {
                continue;
            } else if gvis[(r * 5 + c) as usize][state as usize] == true {
                continue;
            }

            if ahead - state == 1 {
                if ahead == 6 {
                    return cnt + 1;
                } else {
                    deq.push_back(((r, c), cnt + 1, state + 1));
                }
                continue;
            } else {
                deq.push_back(((r, c), cnt + 1, state));
            }
        }
    }

    return -1;
}

fn main() {
    let mut board: Vec<Vec<i8>> = Vec::new();
    let mut line = String::new();

    for _ in 0..5 {
        io::stdin().read_line(&mut line).unwrap();

        board.push(
            line.split_ascii_whitespace()
                .map(|token| token.parse::<i8>().unwrap())
                .collect(),
        );
        line.clear();
    }
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_ascii_whitespace();

    let pos: (i8, i8) = (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );

    println!("{}", get_min_search_cnt(&board, pos));
}
