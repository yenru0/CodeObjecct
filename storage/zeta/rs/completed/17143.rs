use std::cmp::PartialEq;
use std::io::stdin;
use std::str::FromStr;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 3,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Up),
            2 => Ok(Self::Down),
            3 => Ok(Self::Right),
            4 => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Direction::try_from(value.parse::<u8>().unwrap())
    }
}

struct Shark {
    pos: usize,
    speed: u32,
    direction: Direction,
    size: u32,
}

impl Shark {
    fn new(size: &(usize, usize), r: usize, c: usize, s: u32, d: Direction, z: u32) -> Self {
        Self {
            pos: r * size.1 + c,
            speed: s,
            direction: if d.is_horizontal() {
                if (c == 0 && d == Direction::Left) || c == size.1 - 1 && d == Direction::Right {
                    d.reverse()
                } else {
                    d
                }
            } else {
                if (r == 0 && d == Direction::Up) || r == size.0 - 1 && d == Direction::Down {
                    d.reverse()
                } else {
                    d
                }
            },
            size: z,
        }
    }
}

fn move_result(size: (usize, usize), shark: &Shark) -> (usize, Direction) {
    let mut r = shark.pos / size.1;
    let mut c = shark.pos % size.1;

    let mut dir: Direction = shark.direction;

    match dir {
        Direction::Up => {
            let remain_range = (size.0 - 1) * 2;

            let remain = shark.speed as usize % remain_range;

            let c1delta = r;
            let c2delta = r + size.0 - 1;

            match remain {
                rx if rx < c1delta => {
                    r -= remain;
                }
                rx if rx < c2delta => {
                    r = remain - c1delta;
                    dir = dir.reverse();
                }

                rx if rx < remain_range => {
                    r = size.0 - (remain - c2delta) - 1;
                }
                _ => {}
            }

            (r * size.1 + c, dir)
        }

        Direction::Down => {
            let remain_range = (size.0 - 1) * 2;

            let remain = shark.speed as usize % remain_range;

            let c1delta = size.0 - r - 1;
            let c2delta = 2 * size.0 - r - 2;

            match remain {
                rx if rx < c1delta => {
                    r += remain;
                }
                rx if rx < c2delta => {
                    r = size.0 - (remain - c1delta) - 1;
                    dir = dir.reverse();
                }
                rx if rx < remain_range => r = remain - c2delta,
                _ => {}
            }

            (r * size.1 + c, dir)
        }

        Direction::Left => {
            let remain_range = (size.1 - 1) * 2;

            let remain = shark.speed as usize % remain_range;

            let c1delta = c;
            let c2delta = c + size.1 - 1;

            match remain {
                rx if rx < c1delta => {
                    c -= remain;
                }
                rx if rx < c2delta => {
                    c = remain - c1delta;
                    dir = dir.reverse();
                }

                rx if rx < remain_range => {
                    c = size.1 - (remain - c2delta) - 1;
                }
                _ => {}
            }

            (r * size.1 + c, dir)
        }
        Direction::Right => {
            let remain_range = (size.1 - 1) * 2;

            let remain = shark.speed as usize % remain_range;

            let c1delta = size.1 - c - 1;
            let c2delta = 2 * size.1 - c - 2;

            match remain {
                rx if rx < c1delta => {
                    c += remain;
                }
                rx if rx < c2delta => {
                    c = size.1 - (remain - c1delta) - 1;
                    dir = dir.reverse();
                }
                rx if rx < remain_range => c = remain - c2delta,
                _ => {}
            }

            (r * size.1 + c, dir)
        }
    }
}

fn get_total_size_sharks_caught(size: (usize, usize), mut sharks: Vec<Shark>) -> u32 {
    let mut map: Vec<usize> = vec![usize::MAX; size.0 * size.1];

    let mut exists = vec![true; sharks.len()];
    for (i, shark) in sharks.iter().enumerate() {
        map[shark.pos] = i;
    }

    fn update(
        size: (usize, usize),
        map: &mut Vec<usize>,
        sharks: &mut Vec<Shark>,
        exists: &mut Vec<bool>,
    ) {
        // 이동
        let move_record = sharks
            .iter()
            .enumerate()
            .filter(|&(i, _shark)| exists[i])
            .map(|(i, shark)| (i, shark.pos, move_result(size, shark)))
            .collect::<Vec<_>>();

        let mut is_moved = vec![false; size.0 * size.1];
        for (i, pos, (new_pos, new_dir)) in move_record {
            if !is_moved[pos] && pos != new_pos {
                map[pos] = usize::MAX;
            }
            if is_moved[new_pos] == true {
                if &sharks[map[new_pos]].size > &sharks[i].size {
                    exists[i] = false;
                } else {
                    exists[map[new_pos]] = false;
                    map[new_pos] = i;
                }
            } else {
                map[new_pos] = i;
                is_moved[new_pos] = true;
            }

            sharks[i].pos = new_pos;
            sharks[i].direction = new_dir;
        }
    }

    let mut s: u32 = 0;

    for fisher_loc in 0..size.1 {
        match (0..size.0)
            .map(|i| map[i * size.1 + fisher_loc])
            .find(|&x| x != usize::MAX && exists[x])
        {
            None => {}
            Some(x) => {
                s += sharks[x].size;
                exists[x] = false;
            }
        };
        update(size, &mut map, &mut sharks, &mut exists);
    }

    s
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_ascii_whitespace();
    let size: (usize, usize) = (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );
    let num_sharks: usize = iter.next().unwrap().parse().unwrap();

    let sharks = (0..num_sharks)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_ascii_whitespace();
            Shark::new(
                &size,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<u32>().unwrap(), // speed
                iter.next().unwrap().parse::<Direction>().unwrap(), // dir
                iter.next().unwrap().parse::<u32>().unwrap(), // size
            )
        })
        .collect::<Vec<_>>();

    println!("{}", get_total_size_sharks_caught(size, sharks));
}
