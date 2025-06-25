use std::cmp::min;
use std::io::stdin;

fn left_binary_search(arr: &Vec<i32>, target: i32) -> usize {
    let mut lo = 0;
    let mut hi = arr.len();

    while lo < hi {
        let mid = lo + ((hi - lo) >> 1);

        if arr[mid] >= target {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    lo
}

fn get_count_hightouch(s1: String, s2: String) -> i64 {
    let mut cnt: i64 = 0;

    let mut cumulative_s1: Vec<i32> = Vec::new();

    let mut before = 0;

    for c in s1.chars() {
        if c == '(' {
            before += 1;
        } else {
            before -= 1;
        }
        if before < 0 {
            break;
        }
        // ())) 이래놓으면 할 수가 없음
        cumulative_s1.push(before);
    }

    let mut cds: Vec<Vec<i32>> = vec![Vec::new(); 200001];

    before = 0;
    let mut cum_min = 0;

    for c in s2.chars() {
        if c == '(' {
            before += 1;
        } else {
            before -= 1;
        }

        cum_min = min(before, cum_min);

        if before > 0 {
            continue;
        } else {
            cds[(-before) as usize].push(cum_min);
        }
    }

    for v in cds.iter_mut() {
        v.sort();
    }

    for a in cumulative_s1.iter() {
        let mins = &cds[(*a) as usize];
        let append = (mins.len() - left_binary_search(mins, -a)) as i64;
        cnt += append;
    }

    cnt
}

fn main() {
    let (mut s1, mut s2) = (String::new(), String::new());

    stdin().read_line(&mut s1).unwrap();
    stdin().read_line(&mut s2).unwrap();

    s1 = s1.trim().to_string();
    s2 = s2.trim().to_string();

    println!("{}", get_count_hightouch(s1, s2));
}
