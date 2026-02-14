use std::{cmp::min, collections::VecDeque, io::stdin};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Wall,
    Detective,
    Robber,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'D' => Cell::Detective,
            'R' => Cell::Robber,
            _ => Cell::Empty,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum JailState {
    OpenBelow,
    OpenNorth,
    OpenSouth,
    OpenWest,
    OpenEast,
    OpenAbove,
}

impl JailState {
    pub fn to_index(&self) -> usize {
        match self {
            Self::OpenBelow => 0,
            Self::OpenNorth => 1,
            Self::OpenSouth => 2,
            Self::OpenWest => 3,
            Self::OpenEast => 4,
            Self::OpenAbove => 5,
        }
    }
}
#[derive(Clone, Copy)]
pub enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    pub fn value(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::West => (0, -1),
            Self::East => (0, 1),
            Self::South => (1, 0),
        }
    }
}

pub struct Jail {
    state: JailState,
    pos: (usize, usize),
}

impl Jail {
    pub fn roll(&self, dir: Direction) -> Self {
        let next_pos = (
            (self.pos.0 as isize + dir.value().0) as usize,
            (self.pos.1 as isize + dir.value().1) as usize,
        );
        let next_state = match self.state {
            JailState::OpenBelow => match dir {
                Direction::East => JailState::OpenWest,
                Direction::North => JailState::OpenSouth,
                Direction::South => JailState::OpenNorth,
                Direction::West => JailState::OpenEast,
            },
            JailState::OpenAbove => match dir {
                Direction::East => JailState::OpenEast,
                Direction::North => JailState::OpenNorth,
                Direction::South => JailState::OpenSouth,
                Direction::West => JailState::OpenWest,
            },
            JailState::OpenEast => match dir {
                Direction::East => JailState::OpenBelow,
                Direction::West => JailState::OpenAbove,
                Direction::North | Direction::South => JailState::OpenEast,
            },
            JailState::OpenNorth => match dir {
                Direction::North => JailState::OpenBelow,
                Direction::South => JailState::OpenAbove,
                Direction::East | Direction::West => JailState::OpenNorth,
            },
            JailState::OpenSouth => match dir {
                Direction::South => JailState::OpenBelow,
                Direction::North => JailState::OpenAbove,
                Direction::East | Direction::West => JailState::OpenSouth,
            },
            JailState::OpenWest => match dir {
                Direction::West => JailState::OpenBelow,
                Direction::East => JailState::OpenAbove,
                _ => JailState::OpenWest,
            },
        };
        Self {
            state: next_state,
            pos: next_pos,
        }
    }
}

fn find_minimum_rolls_to_catch_robber(n: usize, m: usize, field: &Vec<Vec<Cell>>) -> isize {
    let (d_pos, r_pos) = {
        let mut d = (0, 0);
        let mut r = (0, 0);
        for i in 0..n {
            for j in 0..m {
                if field[i][j] == Cell::Detective {
                    d = (i, j);
                } else if field[i][j] == Cell::Robber {
                    r = (i, j);
                }
            }
        }
        (d, r)
    };

    let mut deq: VecDeque<(usize, Jail)> = VecDeque::new();
    let mut vis: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 6]; m]; n];
    deq.push_back((
        0,
        Jail {
            state: JailState::OpenBelow,
            pos: d_pos,
        },
    ));

    let mut minima = usize::MAX;
    while !deq.is_empty() {
        let (step, jail) = deq.pop_front().unwrap();
        let pos = jail.pos;

        if field[pos.0][pos.1] == Cell::Robber {
            if jail.state == JailState::OpenBelow {
                minima = min(minima, step);
            } else {
                continue;
            }
        }

        if vis[pos.0][pos.1][jail.state.to_index()] {
            continue;
        } else {
            vis[pos.0][pos.1][jail.state.to_index()] = true;
        }

        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let next_pos = (
                (pos.0 as isize + dir.value().0),
                (pos.1 as isize + dir.value().1),
            );

            if next_pos.0 < 0 || next_pos.1 < 0 {
                continue;
            }
            if next_pos.0 >= n as isize || next_pos.1 >= m as isize {
                continue;
            }
            if field[next_pos.0 as usize][next_pos.1 as usize] == Cell::Wall {
                continue;
            }

            let next_jail = jail.roll(dir);
            deq.push_back((step + 1, next_jail));
        }
    }

    if minima == usize::MAX {
        -1
    } else {
        minima as isize
    }
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut iter = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (n, m) = (iter.next().unwrap(), iter.next().unwrap());

    let field = (0..n)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            line.trim()
                .chars()
                .map(|c| Cell::from_char(c))
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let res = find_minimum_rolls_to_catch_robber(n, m, &field);

    println!("{}", res);
}
