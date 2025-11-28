use std::io::stdin;

pub struct Field {
    n: usize,
    data: Vec<Vec<bool>>,
}

impl Field {
    pub fn new(n: usize, data: Vec<Vec<bool>>) -> Self {
        Self { n, data }
    }

    pub fn clone(&self) -> Self {
        Self {
            n: self.n,
            data: self.data.clone(),
        }
    }

    pub fn next_availables(&self, cur_ru: usize) -> Vec<(usize, usize)> {
        let mut availables = vec![];

        let mut ru = cur_ru;
        while availables.is_empty() && ru <= 2 * self.n - 1 {
            for i in (0..(ru + 1)).rev() {
                let j = ru - i;
                if i < self.n && j < self.n && self.data[i][j] {
                    availables.push((i, j));
                }
            }
            ru += 1;
        }

        availables
    }

    pub fn make_bishop(&self, point: (usize, usize)) -> Self {
        let mut new_data = self.data.clone();
        new_data[point.0][point.1] = false;

        let directions: [(isize, isize); 1] = [(1, 1)];
        for direction in directions.iter() {
            let mut x = point.0 as isize;
            let mut y = point.1 as isize;

            loop {
                x += direction.0;
                y += direction.1;

                if x < 0 || x >= self.n as isize || y < 0 || y >= self.n as isize {
                    break;
                }

                new_data[x as usize][y as usize] = false;
            }
        }

        Self {
            n: self.n,
            data: new_data,
        }
    }

    pub fn print(&self) {
        for i in 0..self.n {
            for j in 0..self.n {
                if self.data[i][j] {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!();
        }
    }
}

pub enum Instruction {
    Make(
        (usize, usize), // 착수 위치
    ), // 비숍을 착수하고 다음 스택프레임을 만듬
    Backtrack, // 현재 스택프레임을 파괴하고 이전 프레임으로 돌아감
}

fn solve(n: usize, init_field_data: Vec<Vec<bool>>) -> u64 {
    let mut history: Vec<Field> = vec![];
    let mut stack: Vec<Instruction> = vec![];
    let field = Field::new(n, init_field_data);

    let mut max_bishop_count = 0u64;
    history.push(field);

    stack.push(Instruction::Backtrack);
    for availables in history.last().unwrap().next_availables(0) {
        stack.push(Instruction::Make(availables));
    }

    while !stack.is_empty() {
        let instruction = stack.pop().unwrap();

        match instruction {
            Instruction::Make(point) => {
                let current_field = history.last().unwrap();
                let new_field = current_field.make_bishop(point);

                history.push(new_field);

                // 해지해야함
                stack.push(Instruction::Backtrack);
                let next_availables = history
                    .last()
                    .unwrap()
                    .next_availables(point.0 + point.1 + 1);
                if next_availables.is_empty() {
                    let bishop_count = history.len() as u64 - 1;
                    if bishop_count > max_bishop_count {
                        max_bishop_count = bishop_count;
                    }
                } else {
                    for next_point in next_availables {
                        stack.push(Instruction::Make(next_point));
                    }
                }
            }
            Instruction::Backtrack => {
                history.pop();
            }
        }
    }

    max_bishop_count
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let n: usize = line.trim().parse().unwrap();

    let mut available_point: Vec<(usize, usize)> = vec![];

    let field_data = (0..n)
        .map(|i| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let mut iter = line
                .split(' ')
                .map(|x| x.trim_ascii_end().parse::<u64>().unwrap() == 1);

            (0..n)
                .map(|j| {
                    available_point.push((i, j));
                    iter.next().unwrap()
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let res = solve(n, field_data);
    println!("{}", res);
}
