use std::cmp::max;
use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();

    let mut schedules = (0..n)
        .map(|_| (iter.next().unwrap(), iter.next().unwrap()))
        .collect::<Vec<(usize, usize)>>();

    schedules.sort_by_key(|x| x.0);

    let mut calendar = vec![0usize; 366];
    for &(s, e) in schedules.iter() {
        for i in s..=e {
            calendar[i] += 1;
        }
    }

    let mut local_max_height = 0;
    let mut starting = 0;
    let mut sheet_area = 0;
    for (i, &c) in calendar.iter().enumerate() {
        if c == 0 {
            sheet_area += (i - starting) * local_max_height;
            local_max_height = 0;
        } else {
            if local_max_height == 0 {
                starting = i;
            }

            local_max_height = max(c, local_max_height);
        }
    }

    if local_max_height > 0 {
        sheet_area += (366 - starting) * local_max_height;
    }

    println!("{}", sheet_area);
}
