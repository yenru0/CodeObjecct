use std::io::stdin;

fn can_lowerbound_accept(n: usize, l: usize) -> bool {
    if l % 2 == 0 {
        let d = n / l;
        if d >= l / 2 { true } else { false }
    } else {
        let d = n / l;
        if d >= l / 2 { true } else { false }
    }
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut iter = line
        .trim_ascii_end()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let l = iter.next().unwrap();

    let mut candidate = vec![];

    for i in l..=100 {
        if 2 * n % i == 0 {
            let d = 2 * n / i;
            if d % 2 != 0 { // even
                if i % 2 != 0 {
                    continue;
                } else {
                    let mid = d / 2;
                    if mid < i / 2 - 1 {
                        break;
                    }
                    let lo = mid + 1 - i / 2;
                    for j in lo..lo + i {
                        candidate.push(j);
                    }
                    break;
                }
            } else { // odd
                if i % 2 == 0 {
                    continue;
                } else {
                    let mid = d / 2;
                    if mid < i / 2 {
                        break;
                    }
                    let lo = mid - i / 2;
                    for j in lo..lo + i {
                        candidate.push(j);
                    }
                    break;
                }
            }
        }
    }
    if candidate.len() == 0 {
        println!("-1");
    } else {
        for i in candidate {
            print!("{} ", i);
        }
    }
}
