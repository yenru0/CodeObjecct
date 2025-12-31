use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

const prefield: [[[char; 3]; 5]; 10] = [
    [
        ['*', '*', '*'],
        ['*', ' ', '*'],
        ['*', ' ', '*'],
        ['*', ' ', '*'],
        ['*', '*', '*'],
    ],
    [
        ['*', '*', ' '],
        [' ', '*', ' '],
        [' ', '*', ' '],
        [' ', '*', ' '],
        ['*', '*', '*'],
    ],
    [
        ['*', '*', ' '],
        [' ', ' ', '*'],
        [' ', '*', ' '],
        ['*', ' ', ' '],
        ['*', '*', '*'],
    ],
    [
        ['*', '*', '*'],
        [' ', ' ', '*'],
        [' ', '*', '*'],
        [' ', ' ', '*'],
        ['*', '*', '*'],
    ],
    [
        [' ', ' ', '*'],
        [' ', '*', '*'],
        ['*', ' ', '*'],
        ['*', '*', '*'],
        [' ', ' ', '*'],
    ],
    [
        ['*', '*', '*'],
        ['*', ' ', ' '],
        ['*', '*', ' '],
        [' ', ' ', '*'],
        ['*', '*', '*'],
    ],
    [
        ['*', ' ', ' '],
        ['*', ' ', ' '],
        ['*', '*', '*'],
        ['*', ' ', '*'],
        ['*', '*', '*'],
    ],
    [
        ['*', '*', '*'],
        [' ', ' ', '*'],
        [' ', '*', ' '],
        ['*', ' ', ' '],
        ['*', ' ', ' '],
    ],
    [
        ['*', '*', '*'],
        ['*', ' ', '*'],
        ['*', '*', '*'],
        ['*', ' ', '*'],
        ['*', '*', '*'],
    ],
    [
        ['*', '*', '*'],
        ['*', ' ', '*'],
        ['*', '*', '*'],
        [' ', ' ', '*'],
        [' ', ' ', '*'],
    ],
];

fn get_next_curr(n: usize, m: usize, curr: (usize, usize)) -> (usize, usize) {
    if curr.1 + 1 >= m {
        (curr.0 + 1, 0)
    } else {
        (curr.0, curr.1 + 1)
    }
}

fn is_fin_curr(n: usize, m: usize, curr: (usize, usize)) -> bool {
    return curr.0 >= n - 1 && curr.1 >= m - 1;
}

fn find_biggest_number(n: usize, m: usize, display: &Vec<Vec<char>>) -> usize {
    let mut curr: (usize, usize) = (0, 0);
    let mut display_boxed_region = vec![vec![false; m]; n];

    let deltas: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut largest: (usize, usize, usize, usize) = (usize::MAX, 0, usize::MAX, 0);

    let mut largest_size: usize = 0;

    'main: while !is_fin_curr(n, m, curr) {
        if !display_boxed_region[curr.0][curr.1] {
            let x = display[curr.0][curr.1];

            if x == '*' {
                let mut stack: VecDeque<(usize, usize)> = VecDeque::from(vec![curr]);
                let (mut leftmost, mut rightmost, mut upmost, mut downmost) =
                    (usize::MAX, 0usize, usize::MAX, 0usize);
                while !stack.is_empty() {
                    let now = stack.pop_front().unwrap();
                    if display_boxed_region[now.0][now.1] {
                        continue;
                    }
                    display_boxed_region[now.0][now.1] = true;
                    upmost = min(upmost, now.0);
                    downmost = max(upmost, now.1);
                    leftmost = min(leftmost, now.1);
                    rightmost = max(rightmost, now.1);
                    for &delta in deltas.iter() {
                        let next_r = delta.0 + now.0 as isize;
                        let next_c = delta.1 + now.1 as isize;

                        if next_r < 0 || next_c < 0 || next_r >= n as isize || next_c >= m as isize
                        {
                            continue;
                        }
                        let next_r = next_r as usize;
                        let next_c = next_c as usize;
                        if display_boxed_region[next_r][next_c] || display[next_r][next_c] != '*' {
                            continue;
                        }

                        stack.push_back((next_r, next_c));
                    }
                }
                let size = (rightmost - leftmost + 1) / 3;
                if size > largest_size {
                    largest_size = size;
                    largest = (upmost, downmost, leftmost, rightmost);
                }
            }
        }

        curr = get_next_curr(n, m, curr);
    }
    let largest_up_left = (largest.0, largest.2);
    let mut sample: [[char; 3]; 5] = [[' '; 3]; 5];
    for i in 0..5 {
        for j in 0..3 {
            sample[i][j] =
                display[largest_up_left.0 + i * largest_size][largest_up_left.1 + j * largest_size];
        }
    }
    let mut largest_num = 10;
    for k in (0..=9) {
        let mut flag = true;
        for i in 0..5 {
            for j in 0..3 {
                if prefield[k][i][j] != sample[i][j] {
                    flag = false;
                }
            }
        }
        if flag {
            largest_num = k;
        }
    }

    largest_num
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line
        .trim_ascii_end()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap());

    let (n, m) = (iter.next().unwrap(), iter.next().unwrap());

    let display: Vec<Vec<char>> = (0..n)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            line.trim_end_matches('\n').chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let res = find_biggest_number(n, m, &display);
    println!("{}", res);
}
