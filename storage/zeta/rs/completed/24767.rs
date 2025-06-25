use std::io::stdin;

fn get_sour_sweet(d: f64, poses: Vec<(f64, f64)>) -> (usize, usize) {
    let mut is_fight = vec![false; poses.len()];
    let d_pow = d * d;
    for (i, p1) in poses.iter().enumerate() {
        for (j, p2) in poses.iter().enumerate().skip(i + 1) {
            if (p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0) <= d_pow {
                is_fight[i] = true;
                is_fight[j] = true;
            }
        }
    }

    let fight_cnt = is_fight.iter().filter(|&&x| x).count();

    (fight_cnt, poses.len() - fight_cnt)
}

fn main() {
    while true {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        let d = iter.next().unwrap().parse::<f64>().unwrap();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        line.clear();

        if (n == 0) {
            break;
        }

        let poses: Vec<(f64, f64)> = (0..n)
            .map(|_| {
                stdin().read_line(&mut line).unwrap();
                let mut iter = line.split_ascii_whitespace();
                let t = (
                    iter.next().unwrap().parse::<f64>().unwrap(),
                    iter.next().unwrap().parse::<f64>().unwrap(),
                );
                line.clear();
                t
            })
            .collect();
        let res = get_sour_sweet(d, poses);

        println!("{} sour, {} sweet", res.0, res.1);
    }
}
