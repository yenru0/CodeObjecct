use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Binary,
    io::{read_to_string, stdin},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Water = 0,
    Lily = 1,
    Rock = 2,
    StartLily = 3,
    DestLily = 4,
}

impl From<usize> for Tile {
    fn from(num: usize) -> Self {
        match num {
            0 => Tile::Water,
            1 => Tile::Lily,
            2 => Tile::Rock,
            3 => Tile::StartLily,
            4 => Tile::DestLily,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    installs: usize,
    moves: usize,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .installs
            .cmp(&self.installs)
            .then_with(|| other.moves.cmp(&self.moves))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DELTAS: [(isize, isize); 8] = [
    (-2, 1),
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, -1),
    (2, -1),
    (1, -2),
    (-1, -2),
];

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let rows = iter.next().unwrap();
    let cols = iter.next().unwrap();

    let field: Vec<Vec<Tile>> = (0..rows)
        .map(|_| {
            (0..cols)
                .map(|_| Tile::from(iter.next().unwrap()))
                .collect()
        })
        .collect();

    let (start_pos, dest_pos) = {
        let mut sp = (0, 0);
        let mut dp = (0, 0);
        for i in (0..rows) {
            for j in (0..cols) {
                match field[i][j] {
                    Tile::StartLily => {
                        sp = (i, j);
                    }
                    Tile::DestLily => {
                        dp = (i, j);
                    }
                    _ => {}
                }
            }
        }
        (sp, dp)
    };

    let mut field_install_cnt: Vec<Vec<usize>> = vec![vec![usize::MAX; cols]; rows];
    let mut field_move_cnt: Vec<Vec<usize>> = vec![vec![usize::MAX; cols]; rows];
    let mut field_path_cnt: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    let mut deq = BinaryHeap::new();
    deq.push(State {
        installs: 0,
        moves: 0,
        pos: start_pos,
    });

    field_path_cnt[start_pos.0][start_pos.1] = 1;

    while !deq.is_empty() {
        let state = deq.pop().unwrap();
        let (curr_installs, curr_moves, curr_pos) = (state.installs, state.moves, state.pos);

        if curr_installs > field_install_cnt[curr_pos.0][curr_pos.1]
            || (curr_installs == field_install_cnt[curr_pos.0][curr_pos.1]
                && curr_moves > field_move_cnt[curr_pos.0][curr_pos.1])
        {
            continue;
        }

        for (dr, dc) in DELTAS {
            let next_pos = (curr_pos.0 as isize + dr, curr_pos.1 as isize + dc);
            if next_pos.0 < 0
                || next_pos.0 >= rows as isize
                || next_pos.1 < 0
                || next_pos.1 >= cols as isize
            {
                continue;
            }
            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
            if field[next_pos.0][next_pos.1] == Tile::Rock {
                continue;
            }

            let next_installs = if field[next_pos.0][next_pos.1] == Tile::Water {
                curr_installs + 1
            } else {
                curr_installs
            };

            let next_moves = curr_moves + 1;

            // calc first
            if next_installs < field_install_cnt[next_pos.0][next_pos.1] {
                field_install_cnt[next_pos.0][next_pos.1] = next_installs;
                field_move_cnt[next_pos.0][next_pos.1] = next_moves;
                field_path_cnt[next_pos.0][next_pos.1] = field_path_cnt[curr_pos.0][curr_pos.1];
                deq.push(State {
                    installs: next_installs,
                    moves: next_moves,
                    pos: next_pos,
                });
            } else if next_installs == field_install_cnt[next_pos.0][next_pos.1] {
                if next_moves < field_move_cnt[next_pos.0][next_pos.1] {
                    field_move_cnt[next_pos.0][next_pos.1] = next_moves;
                    field_path_cnt[next_pos.0][next_pos.1] = field_path_cnt[curr_pos.0][curr_pos.1];
                    deq.push(State {
                        installs: next_installs,
                        moves: next_moves,
                        pos: next_pos,
                    });
                } else if next_moves == field_move_cnt[next_pos.0][next_pos.1] {
                    field_path_cnt[next_pos.0][next_pos.1] +=
                        field_path_cnt[curr_pos.0][curr_pos.1];
                }
            }
        }
    }

    if field_install_cnt[dest_pos.0][dest_pos.1] != usize::MAX {
        println!("{}", field_install_cnt[dest_pos.0][dest_pos.1]);
        println!("{}", field_move_cnt[dest_pos.0][dest_pos.1]);
        println!("{}", field_path_cnt[dest_pos.0][dest_pos.1]);
    } else {
        println!("{}", -1);
    }
}
