use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut iter = line.trim().split(' ').map(|x| x.parse::<usize>().unwrap());

    let (n, m, k) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let mut comb_mem = [[0u64; 9]; 9];

    for i in 0..=n {
        comb_mem[i][0] = 1; // nC0 = 1
        for j in 1..=i {
            comb_mem[i][j] = comb_mem[i - 1][j - 1] + comb_mem[i - 1][j];
        }
    }

    let total_cases = comb_mem[n][m];
    let mut winning_cases = 0u64;

    for i in k..=m {
        if n >= m && (n - m) >= (m - i) {
            let ways_to_match = comb_mem[m][i];
            let ways_to_pick_rest = comb_mem[n - m][m - i];
            winning_cases += ways_to_match * ways_to_pick_rest;
        }
    }

    let probability = winning_cases as f64 / total_cases as f64;

    println!("{:.10}", probability);
}
