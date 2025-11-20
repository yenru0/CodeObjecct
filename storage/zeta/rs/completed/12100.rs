use std::io::stdin;

#[derive(PartialEq, Eq)]
enum InstructionType {
    Move,
    Restore,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    instruction_type: InstructionType,
    direction: Direction,
}

impl Instruction {
    fn new(instruction_type: InstructionType, direction: Direction) -> Self {
        Instruction {
            instruction_type,
            direction,
        }
    }
}

fn move_board(board: &Vec<Vec<u64>>, direction: &Direction) -> Vec<Vec<u64>> {
    let n = board.len();
    let mut new_board = vec![vec![0; n]; n];

    match direction {
        Direction::Up => {
            for col in 0..n {
                let mut position = 0;
                let mut last_merged = false;
                for row in 0..n {
                    if board[row][col] != 0 {
                        if position > 0
                            && new_board[position - 1][col] == board[row][col]
                            && !last_merged
                        {
                            new_board[position - 1][col] *= 2;
                            last_merged = true;
                        } else {
                            new_board[position][col] = board[row][col];
                            position += 1;
                            last_merged = false;
                        }
                    }
                }
            }
        }
        Direction::Down => {
            for col in 0..n {
                let mut position = n - 1;
                let mut last_merged = false;
                for row in (0..n).rev() {
                    if board[row][col] != 0 {
                        if position < n - 1
                            && new_board[position + 1][col] == board[row][col]
                            && !last_merged
                        {
                            new_board[position + 1][col] *= 2;
                            last_merged = true;
                        } else {
                            new_board[position][col] = board[row][col];
                            if position > 0 {
                                position -= 1;
                            }
                            last_merged = false;
                        }
                    }
                }
            }
        }
        Direction::Left => {
            for row in 0..n {
                let mut position = 0;
                let mut last_merged = false;
                for col in 0..n {
                    if board[row][col] != 0 {
                        if position > 0
                            && new_board[row][position - 1] == board[row][col]
                            && !last_merged
                        {
                            new_board[row][position - 1] *= 2;
                            last_merged = true;
                        } else {
                            new_board[row][position] = board[row][col];
                            position += 1;
                            last_merged = false;
                        }
                    }
                }
            }
        }
        Direction::Right => {
            for row in 0..n {
                let mut position = n - 1;
                let mut last_merged = false;
                for col in (0..n).rev() {
                    if board[row][col] != 0 {
                        if position < n - 1
                            && new_board[row][position + 1] == board[row][col]
                            && !last_merged
                        {
                            new_board[row][position + 1] *= 2;
                            last_merged = true;
                        } else {
                            new_board[row][position] = board[row][col];
                            if position > 0 {
                                position -= 1;
                            }
                            last_merged = false;
                        }
                    }
                }
            }
        }
    }

    new_board
}

fn max_element_of(board: &Vec<Vec<u64>>) -> u64 {
    let mut max_val: u64 = 0;
    for row in board {
        for &val in row {
            if val > max_val {
                max_val = val;
            }
        }
    }
    max_val
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<u64>>) {
    for row in board {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

const MAX_DEPTH: u64 = 4;

fn get_maximum_2048_value(original_board: &Vec<Vec<u64>>) -> u64 {
    let mut history: Vec<Vec<Vec<u64>>> = vec![];
    let mut maxima: u64 = 0;
    let mut stack: Vec<(Instruction, u64)> = vec![];

    history.push(original_board.clone());
    stack.push((Instruction::new(InstructionType::Restore, Direction::Up), 0));
    stack.push((Instruction::new(InstructionType::Move, Direction::Up), 0));
    stack.push((Instruction::new(InstructionType::Move, Direction::Down), 0));
    stack.push((Instruction::new(InstructionType::Move, Direction::Left), 0));
    stack.push((Instruction::new(InstructionType::Move, Direction::Right), 0));

    while !stack.is_empty() {
        let (inst, depth) = stack.pop().unwrap();

        if inst.instruction_type == InstructionType::Restore {
            history.pop().unwrap();
            continue;
        } else {
            let current_board = history.last().unwrap();
            let new_board = move_board(current_board, &inst.direction);
            if depth == MAX_DEPTH {
                let local_max = max_element_of(&new_board);

                if local_max > maxima {
                    maxima = local_max;
                }
            } else if depth < MAX_DEPTH {
                history.push(new_board);
                stack.push((
                    Instruction::new(InstructionType::Restore, Direction::Up),
                    depth,
                ));
                stack.push((
                    Instruction::new(InstructionType::Move, Direction::Up),
                    depth + 1,
                ));
                stack.push((
                    Instruction::new(InstructionType::Move, Direction::Down),
                    depth + 1,
                ));
                stack.push((
                    Instruction::new(InstructionType::Move, Direction::Left),
                    depth + 1,
                ));
                stack.push((
                    Instruction::new(InstructionType::Move, Direction::Right),
                    depth + 1,
                ));
            }
        }
    }

    maxima
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let n: usize = line.trim().parse().unwrap();

    let board = (0..n)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            line.split(' ')
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let result = get_maximum_2048_value(&board);
    println!("{}", result);
}
