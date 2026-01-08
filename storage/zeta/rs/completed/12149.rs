use std::io::{read_to_string, stdin};

// maxima: 2
// 1/4 * 2 + 1/4 * 1 + 1/4
/*

    GO
*/

fn get_max_overlap(target: &String) -> usize {
    let l = target.len();
    let chs: Vec<char> = target.chars().collect();

    for i in 1..l {
        if chs[i..] == chs[..l - i] {
            return l - i;
        }
    }
    0
}

fn expected_my_banana(keyboard: String, target: String, s: usize) -> f64 {
    let k = keyboard.len();
    let l = target.len();
    let key_distribution = keyboard.chars().fold([0usize; 26], |mut acc, x| {
        acc[x as usize - 'A' as usize] += 1;
        accQ
    });

    let key_proba = key_distribution.map(|x| x as f64 / k as f64);
    let proba_de_target = target
        .chars()
        .fold(1.0, |acc, x| acc * key_proba[x as usize - 'A' as usize]);
    let overlap = get_max_overlap(&target);

    let maxima_banana: f64 = {
        let mut flag = false;
        for key in target.chars() {
            if key_distribution[key as usize - 'A' as usize] == 0 {
                flag = true;
                break;
            }
        }
        if flag {
            return 0.0;
        } else {
            1.0 + ((s - l) / (l - overlap)) as f64
        }
    };

    let expected_give = proba_de_target * (s - l + 1) as f64;
    maxima_banana - expected_give
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    (0..n).for_each(|case| {
        iter.next();
        iter.next();
        let s = iter.next().unwrap().parse().unwrap();

        let keyboard = iter.next().unwrap().trim().to_string();
        let target = iter.next().unwrap().trim().to_string();

        let res = expected_my_banana(keyboard, target, s);
        println!("Case #{}: {:0.8}", case + 1, res);
    });
}
