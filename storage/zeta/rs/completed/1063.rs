use std::io::{read_to_string, stdin};

enum Movement {
    R,
    L,
    B,
    T,
    RT,
    LT,
    RB,
    LB,
}

impl Movement {
    fn from_str(s: &str) -> Self {
        match s {
            "R" => Self::R,
            "L" => Self::L,
            "B" => Self::B,
            "T" => Self::T,
            "RT" => Self::RT,
            "LT" => Self::LT,
            "RB" => Self::RB,
            "LB" => Self::LB,
            _ => panic!("Invalid movement"),
        }
    }

    fn to_delta(&self) -> (i8, i8) {
        match self {
            Self::R => (1, 0),
            Self::L => (-1, 0),
            Self::B => (0, -1),
            Self::T => (0, 1),
            Self::RT => (1, 1),
            Self::LT => (-1, 1),
            Self::RB => (1, -1),
            Self::LB => (-1, -1),
        }
    }
}

fn to_pos(s: &str) -> (u8, u8) {
    (s.as_bytes()[0] - b'A', s.as_bytes()[1] - b'1')
}

fn to_code(pos: (u8, u8)) -> String {
    let col = (pos.0 + b'A') as char;
    let row = (pos.1 + b'1') as char;
    format!("{}{}", col, row)
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    let str_king = iter.next().unwrap();
    let mut pos_king = to_pos(str_king);

    let str_stone = iter.next().unwrap();
    let mut pos_stone = to_pos(str_stone);

    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let movements = iter
        .take(n)
        .map(Movement::from_str)
        .collect::<Vec<Movement>>();

    for mv in movements {
        let (dx, dy) = mv.to_delta();
        let new_pos_king = (pos_king.0 as i8 + dx, pos_king.1 as i8 + dy);
        if new_pos_king.0 < 0 || new_pos_king.0 >= 8 || new_pos_king.1 < 0 || new_pos_king.1 >= 8 {
            continue;
        }

        let new_pos_king = (new_pos_king.0 as u8, new_pos_king.1 as u8);

        if new_pos_king == pos_stone {
            let new_pos_stone = (pos_stone.0 as i8 + dx, pos_stone.1 as i8 + dy);

            if new_pos_stone.0 < 0
                || new_pos_stone.0 >= 8
                || new_pos_stone.1 < 0
                || new_pos_stone.1 >= 8
            {
                continue;
            }
            let new_pos_stone = (new_pos_stone.0 as u8, new_pos_stone.1 as u8);

            pos_stone = new_pos_stone;
            pos_king = new_pos_king;
        } else {
            pos_king = new_pos_king;
        }
    }

    println!("{}", to_code(pos_king));
    println!("{}", to_code(pos_stone));
}
