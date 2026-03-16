use std::io::{read_to_string, stdin};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let t = iter.next().unwrap();
    (0..t).for_each(|_| {
        let (a, b, c, m) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        let l_ab = lcm(a, b);
        let l_bc = lcm(b, c);
        let l_ac = lcm(a, c);
        let l_abc = lcm(l_ab, c);
        // density of a

        let cnt_a = {
            let tot = m / a;
            if tot == 0 {
                tot
            } else {
                let with_b = l_ab / a;
                let with_c = l_ac / a;
                let with_bc = l_abc / a;
                let tot_with_b = tot / with_b;
                let tot_with_c = tot / with_c;
                let tot_with_bc = tot / with_bc;

                let res = (tot + tot_with_bc - tot_with_b - tot_with_c) * 6
                    + (tot_with_b - tot_with_bc) * 3
                    + (tot_with_c - tot_with_bc) * 3
                    + (tot_with_bc) * 2;
                res
            }
        };

        let cnt_b = {
            let tot = m / b;
            if tot == 0 {
                tot
            } else {
                let with_a = l_ab / b;
                let with_c = l_bc / b;
                let with_ac = l_abc / b;
                let tot_with_a = tot / with_a;
                let tot_with_c = tot / with_c;
                let tot_with_ac = tot / with_ac;

                let res = (tot + tot_with_ac - tot_with_a - tot_with_c) * 6
                    + (tot_with_a - tot_with_ac) * 3
                    + (tot_with_c - tot_with_ac) * 3
                    + (tot_with_ac) * 2;
                res
            }
        };

        let cnt_c = {
            let tot = m / c;
            if tot == 0 {
                tot
            } else {
                let with_a = l_ac / c;
                let with_b = l_bc / c;
                let with_ab = l_abc / c;
                let tot_with_a = tot / with_a;
                let tot_with_b = tot / with_b;
                let tot_with_ab = tot / with_ab;

                let res = (tot + tot_with_ab - tot_with_a - tot_with_b) * 6
                    + (tot_with_a - tot_with_ab) * 3
                    + (tot_with_b - tot_with_ab) * 3
                    + (tot_with_ab) * 2;
                res
            }
        };

        println!("{} {} {}", cnt_a, cnt_b, cnt_c);
    })
}
