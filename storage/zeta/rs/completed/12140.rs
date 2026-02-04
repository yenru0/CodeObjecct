use std::io::stdin;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Up,
    Right,
    Down,
    Left,
    Period,
}

fn cnt_minimum_changed_to_be_safe(r: usize, c: usize, field: &Vec<Vec<Cell>>) -> usize {
    let mut res = 0;

    let has_target = |row: usize, col: usize, dr: i32, dc: i32| -> bool {
        let mut curr_r = row as i32 + dr;
        let mut curr_c = col as i32 + dc;
        while curr_r >= 0 && curr_r < r as i32 && curr_c >= 0 && curr_c < c as i32 {
            if field[curr_r as usize][curr_c as usize] != Cell::Period {
                return true;
            }
            curr_r += dr;
            curr_c += dc;
        }
        false
    };

    for i in 0..r {
        for j in 0..c {
            let s = field[i][j];
            if let Cell::Period = s {
                continue;
            }

            let (dr, dc) = match s {
                Cell::Up => (-1, 0),
                Cell::Down => (1, 0),
                Cell::Left => (0, -1),
                Cell::Right => (0, 1),
                Cell::Period => unreachable!(),
            };

            if has_target(i, j, dr, dc) {
                continue;
            }

            let possible_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let can_be_saved = possible_dirs
                .iter()
                .any(|&(ndr, ndc)| has_target(i, j, ndr, ndc));

            if can_be_saved {
                res += 1;
            } else {
                return usize::MAX;
            }
        }
    }

    res
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t = line.trim().parse::<usize>().unwrap();
    (1..=t).for_each(|case| {
        line.clear();
        stdin().read_line(&mut line).unwrap();

        let (r, c) = {
            let mut iter = line.trim().split(' ').map(|x| x.parse::<usize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };

        let field = (0..r)
            .map(|_| {
                line.clear();
                stdin().read_line(&mut line).unwrap();
                let row = line
                    .trim()
                    .chars()
                    .map(|c| match c {
                        '.' => Cell::Period,
                        '>' => Cell::Right,
                        '<' => Cell::Left,
                        '^' => Cell::Up,
                        'v' => Cell::Down,
                        _ => Cell::Period,
                    })
                    .collect::<Vec<Cell>>();
                row
            })
            .collect::<Vec<Vec<Cell>>>();

        let res = cnt_minimum_changed_to_be_safe(r, c, &field);
        println!(
            "Case #{}: {}",
            case,
            if res != usize::MAX {
                res.to_string()
            } else {
                "IMPOSSIBLE".to_string()
            }
        )
    });
}
