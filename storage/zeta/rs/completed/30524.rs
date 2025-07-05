use std::io::{stdin, stdout, BufWriter, Write};

fn get_able_pivots(arr: &Vec<u32>) -> Vec<u32> {
    if arr.len() == 1 {
        return vec![arr[0]];
    }

    let mut before_max: u32 = 0;
    let mut before_min: u32 = u32::MAX;

    let ortho_cumulative_max: Vec<u32> = arr
        .iter()
        .map(|&x| {
            if x > before_max {
                before_max = x;
            }
            before_max
        })
        .collect();

    let reverse_cumulative_min: Vec<u32> = arr
        .iter()
        .rev()
        .map(|&x| {
            if x < before_min {
                before_min = x;
            }
            before_min
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .rev()
        .collect();

    let mut pivots: Vec<u32> = Vec::new();

    if arr[0] < reverse_cumulative_min[1] {
        pivots.push(arr[0]);
    }

    for i in 1..arr.len() - 1 {
        if ortho_cumulative_max[i - 1] < arr[i] && arr[i] < reverse_cumulative_min[i + 1] {
            pivots.push(arr[i]);
        }
    }

    if arr[arr.len() - 1] > ortho_cumulative_max[arr.len() - 2] {
        pivots.push(arr[arr.len() - 1]);
    }
    pivots
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().trim_end().parse().unwrap();

    let arr: Vec<u32> = (0..n)
        .map(|_| iter.next().unwrap().trim_end().parse().unwrap())
        .collect();

    (|x: Vec<u32>| {
        write!(out, "{} ", x.len()).unwrap();
        x
    })(get_able_pivots(&arr))
    .into_iter()
    .take(100)
    .for_each(|x| {
        write!(out, "{} ", x).unwrap();
    });
}
