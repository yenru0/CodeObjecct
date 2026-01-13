use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split(' ').map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let words = (0..n)
        .map(|_| lines.next().unwrap().unwrap().trim().chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut cnts = vec![vec![0; 26]; m];
    for i in 0..n {
        for (j, &ch) in words[i].iter().enumerate() {
            let idx = (ch as u8 - 'a' as u8) as usize;
            cnts[j][idx] += 1;
        }
    }

    let mut total_penalty = 0;
    for i in 0..m {
        let mut max_value = 0;
        for j in 0..26 {
            let value = cnts[i][j];
            if value > max_value {
                max_value = value;
            }
        }
        total_penalty += n - max_value;
    }

    println!("{}", total_penalty);
}
