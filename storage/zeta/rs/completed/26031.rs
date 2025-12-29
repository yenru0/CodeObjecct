use std::cmp::max;
use std::{io::stdin, usize};

#[allow(unused_assignments)]
fn get_highest_peak(heights: &Vec<usize>) -> usize {
    let n = heights.len();
    let mut left_min = vec![0; n];
    let mut right_min = vec![0; n];

    left_min[0] = heights[0];
    right_min[n - 1] = heights[n - 1];

    for i in 1..n {
        if heights[i] < heights[i - 1] {
            left_min[i] = heights[i];
        } else {
            left_min[i] = left_min[i - 1];
        }
    }

    for j in (0..(n - 1)).rev() {
        if heights[j] < heights[j + 1] {
            right_min[j] = heights[j];
        } else {
            right_min[j] = right_min[j + 1];
        }
    }

    let mut max_peak = 0;

    for i in 1..n - 1 {
        if heights[i] >= heights[i - 1] && heights[i] >= heights[i + 1] {
            let peak = heights[i] - max(left_min[i - 1], right_min[i + 1]);
            max_peak = max(max_peak, peak);
        }
    }
    max_peak
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let _: usize = line.trim().parse().unwrap();

    line.clear();
    stdin().read_line(&mut line).unwrap();
    let heights: Vec<usize> = line
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", get_highest_peak(&heights));
}
