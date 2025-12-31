use std::{cmp::max, io::stdin};

#[allow(unused_assignments)]
fn second_order_tanghuru(n: usize, fruits: &Vec<usize>) -> usize {
    let mut left = 0;
    let mut right = 0;

    let mut max_second_order_tanghuru = 0;

    'main: loop {
        let left_fruit = fruits[left];
        let mut right_fruit = fruits[right];
        while right_fruit == left_fruit {
            right += 1;
            if right < n {
                right_fruit = fruits[right];
            } else {
                break 'main;
            }
        }

        let panel = right;
        let panel_fruit = fruits[panel];
        let mut last_changed = panel;
        let mut before_fruit = panel_fruit;
        while right_fruit == panel_fruit || right_fruit == left_fruit {
            if right_fruit != before_fruit {
                last_changed = right;
            }
            right += 1;
            if right < n {
                before_fruit = right_fruit;
                right_fruit = fruits[right];
            } else {
                break 'main;
            }
        }
        max_second_order_tanghuru = max(max_second_order_tanghuru, right - left);
        left = last_changed;
    }

    max_second_order_tanghuru = max(max_second_order_tanghuru, right - left);

    max_second_order_tanghuru
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    line.clear();
    stdin().read_line(&mut line).unwrap();
    let fruits = line
        .trim_end()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("{}", second_order_tanghuru(n, &fruits));
}
