use std::{collections::VecDeque, io::stdin};

enum PlaceState {
    Start,
    Safe,
    SafeFromUp,
    SafeFromDown,
    SafeFromLeft,
    SafeFromRight,
    Unsafe,
}

fn minimum_steps_to_get_out(places: &Vec<Vec<PlaceState>>, starting_point: (usize, usize)) -> i64 {
    let n = places.len();
    let m = places[0].len();

    let mut vis = vec![vec![false; m]; n];

    let mut deq: VecDeque<((usize, usize), i64)> = VecDeque::new();
    deq.push_back((starting_point, 0));

    while !deq.is_empty() {
        let ((r, c), step) = deq.pop_front().unwrap();

        if r == n - 1 || r == 0 || c == 0 || c == m - 1 {
            return step;
        }

        if r >= 1 {
            match &places[r - 1][c] {
                PlaceState::Safe | PlaceState::SafeFromDown => {
                    if !vis[r - 1][c] {
                        if r - 1 == 0 {
                            return step + 1;
                        }
                        vis[r - 1][c] = true;
                        deq.push_back(((r - 1, c), step + 1));
                    }
                }
                _ => {}
            }
        }
        if r + 1 < n {
            match &places[r + 1][c] {
                PlaceState::Safe | PlaceState::SafeFromUp => {
                    if !vis[r + 1][c] {
                        if r + 1 == n - 1 {
                            return step + 1;
                        }
                        vis[r + 1][c] = true;
                        deq.push_back(((r + 1, c), step + 1));
                    }
                }
                _ => {}
            }
        }
        if c >= 1 {
            match &places[r][c - 1] {
                PlaceState::Safe | PlaceState::SafeFromRight => {
                    if !vis[r][c - 1] {
                        if c - 1 == 0 {
                            return step + 1;
                        }
                        vis[r][c - 1] = true;
                        deq.push_back(((r, c - 1), step + 1));
                    }
                }
                _ => {}
            }
        }
        if c + 1 < m {
            match &places[r][c + 1] {
                PlaceState::Safe | PlaceState::SafeFromLeft => {
                    if !vis[r][c + 1] {
                        if c + 1 == m - 1 {
                            return step + 1;
                        }
                        vis[r][c + 1] = true;
                        deq.push_back(((r, c + 1), step + 1));
                    }
                }
                _ => {}
            }
        }
    }

    -1
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line.split(' ').map(|x| x.trim().parse::<usize>().unwrap());

    let t = iter.next().unwrap();
    let n = iter.next().unwrap();
    let m = iter.next().unwrap(); // suppress unused variable warning
    let mut starting_point = (0, 0);
    let places = (0..n)
        .map(|i| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            line.trim()
                .chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        starting_point = (i, j);
                        PlaceState::Start
                    }
                    '0' => PlaceState::Safe,
                    '1' => PlaceState::Unsafe,
                    'U' => PlaceState::SafeFromUp,
                    'D' => PlaceState::SafeFromDown,
                    'L' => PlaceState::SafeFromLeft,
                    'R' => PlaceState::SafeFromRight,
                    _ => unreachable!(),
                })
                .collect::<Vec<PlaceState>>()
        })
        .collect::<Vec<Vec<PlaceState>>>();
    let result = minimum_steps_to_get_out(&places, starting_point);
    if result < 0 || result > t as i64 {
        println!("NOT POSSIBLE");
    } else {
        println!("{}", result);
    }
}
