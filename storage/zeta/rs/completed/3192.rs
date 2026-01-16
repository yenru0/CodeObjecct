use std::io::{read_to_string, stdin};

const SIZE: usize = 3;

fn get_horizontal_with_cnt_zeros(magic_square: &[u64; 9], row: usize) -> (u64, usize) {
    let mut sum = 0;
    let mut cnt_zeros = 0;

    for j in 0..SIZE {
        let v = magic_square[row * SIZE + j];
        if v == 0 {
            cnt_zeros += 1;
        }
        sum += v;
    }

    (sum, cnt_zeros)
}

fn get_vertical_with_cnt_zeros(magic_square: &[u64; 9], col: usize) -> (u64, usize) {
    let mut sum = 0;
    let mut cnt_zeros = 0;

    for i in 0..SIZE {
        let v = magic_square[i * SIZE + col];
        if v == 0 {
            cnt_zeros += 1;
        }
        sum += v;
    }

    (sum, cnt_zeros)
}

fn get_diagonal1_with_cnt_zeros(magic_square: &[u64; 9]) -> (u64, usize) {
    let mut sum = 0;
    let mut cnt_zeros = 0;

    for i in 0..SIZE {
        let v = magic_square[i * SIZE + i];
        if v == 0 {
            cnt_zeros += 1;
        }
        sum += v;
    }

    (sum, cnt_zeros)
}

fn get_diagonal2_with_cnt_zeros(magic_square: &[u64; 9]) -> (u64, usize) {
    let mut sum = 0;
    let mut cnt_zeros = 0;

    for i in 0..SIZE {
        let v = magic_square[i * SIZE + (SIZE - 1 - i)];
        if v == 0 {
            cnt_zeros += 1;
        }
        sum += v;
    }

    (sum, cnt_zeros)
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let mut magic_square = [0; 9];

    let mut missings = vec![];

    for i in 0..SIZE * SIZE {
        let value = iter.next().unwrap();
        if value == 0 {
            missings.push(i);
        }
        magic_square[i] = value;
    }

    let mut res = 0u64;

    let mut zero_flag = false;
    for i in 0..SIZE {
        let mut s1 = 0;
        let mut s2 = 0;
        let mut z1 = 0;
        let mut z2 = 0;
        for j in 0..SIZE {
            let v1 = magic_square[i * SIZE + j];
            if v1 == 0 {
                z1 += 1;
            }
            s1 += v1;
            let v2 = magic_square[i + SIZE * j];
            if v2 == 0 {
                z2 += 1;
            }
            s2 += v2;
        }

        if s1 == 0 || s2 == 0 {
            zero_flag = true;
            break;
        }

        if z1 == 0 {
            res = s1;
            break;
        } else if z2 == 0 {
            res = s2;
            break;
        }
    }
    {
        let mut s1 = 0;
        let mut s2 = 0;
        let mut z1 = 0;
        let mut z2 = 0;
        for i in 0..SIZE {
            let v1 = magic_square[i * SIZE + i];
            let v2 = magic_square[i * SIZE + (SIZE - 1 - i)];
            if v1 == 0 {
                z1 += 1;
            }
            if v2 == 0 {
                z2 += 1;
            }
            s1 += v1;
            s2 += v2;
        }

        if s1 == 0 || s2 == 0 {
            zero_flag = true;
        }

        if z1 == 0 {
            res = s1;
        } else if z2 == 0 {
            res = s2;
        }
    }

    if zero_flag {
        res = magic_square.iter().sum::<u64>() / (SIZE as u64 - 1);
    }

    loop {
        let mut cnt_zero = 0;

        for i in 0..SIZE * SIZE {
            if magic_square[i] == 0 {
                cnt_zero += 1;
                // fill

                let (r, c) = (i / SIZE, i % SIZE);

                let (h_sum, h_cnt_zeros) = get_horizontal_with_cnt_zeros(&magic_square, r);
                let (v_sum, v_cnt_zeros) = get_vertical_with_cnt_zeros(&magic_square, c);
                if h_cnt_zeros == 1 {
                    magic_square[i] = res - h_sum;
                } else if v_cnt_zeros == 1 {
                    magic_square[i] = res - v_sum;
                } else {
                    if r == c {
                        let (d1_sum, d1_cnt_zeros) = get_diagonal1_with_cnt_zeros(&magic_square);
                        if d1_cnt_zeros == 1 {
                            magic_square[i] = res - d1_sum;
                            continue;
                        }
                    }
                    if r + c == SIZE - 1 {
                        let (d2_sum, d2_cnt_zeros) = get_diagonal2_with_cnt_zeros(&magic_square);
                        if d2_cnt_zeros == 1 {
                            magic_square[i] = res - d2_sum;
                            continue;
                        }
                    }
                }
            }
        }

        if cnt_zero == 0 {
            break;
        }
    }

    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{} ", magic_square[i * SIZE + j]);
        }
        println!();
    }
}
