use std::io::stdin;

const YOKOHAMA: &[char] = &['Y', 'O', 'K', 'O', 'H', 'A', 'M', 'A'];

fn get_yokohama_count(size: (usize, usize), field: &Vec<Vec<char>>) -> usize {
    let (n, m) = size;
    let mut count = 0;

    let mut startings: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        for j in 0..m {
            if field[i][j] == 'Y' {
                startings.push((i, j));
            }
        }
    }

    let mut deq: Vec<((usize, usize), usize)> = vec![];
    for starting in startings {
        deq.push((starting, 0));
    }

    while !deq.is_empty() {
        let (loc, curr) = deq.pop().unwrap();
        let next_char = YOKOHAMA[curr + 1];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = loc.0 as isize + dx;
            let new_y = loc.1 as isize + dy;
            if new_x < 0 || new_x >= n as isize || new_y < 0 || new_y >= m as isize {
                continue;
            }
            let new_loc = (new_x as usize, new_y as usize);

            if field[new_loc.0][new_loc.1] == next_char {
                if curr + 1 == YOKOHAMA.len() - 1 {
                    count += 1;
                } else {
                    deq.push((new_loc, curr + 1));
                }
            }
        }
    }

    count
}

fn main() {
    let mut line = String::new();

    let (n, m) = {
        stdin().read_line(&mut line).unwrap();
        let mut iter = line.split(' ').map(|x| x.trim().parse::<usize>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let field = (0..n)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            line.trim_ascii().chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    println!("{}", get_yokohama_count((n, m), &field));
}
