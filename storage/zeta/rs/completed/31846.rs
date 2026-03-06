use std::io::stdin;

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    let _ = line.trim().parse::<usize>().unwrap();

    line.clear();
    stdin().read_line(&mut line).unwrap();
    let s = line.trim().chars().collect::<Vec<char>>();

    line.clear();
    stdin().read_line(&mut line).unwrap();
    let q = line.trim().parse::<usize>().unwrap();

    let queries = (0..q).map(|_| {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let l = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let r = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        (l, r)
    });

    for (l, r) in queries {
        let len = r - l + 1;
        let sub = &s[l..=r];

        let mut max_score = 0;
        for k in 1..len {
            let mut score = 0;
            for i in 0..k.min(len - k) {
                if sub[k - 1 - i] == sub[k + i] {
                    score += 1;
                }
            }
            max_score = max_score.max(score);
        }
        println!("{}", max_score);
    }
}
