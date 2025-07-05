use std::io::stdin;

const X_MAX: usize = 1_000_001;

/**
get divisor of n excepts n
*/
fn get_divisors(n: u32) -> Vec<u32> {
    let mut divisors: Vec<u32> = Vec::new();
    divisors.push(1);
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors
}

fn number_partition_game_result(participants: &Vec<u32>) -> Vec<i32> {
    let mut sorted_participants = participants.clone();
    sorted_participants.sort();

    let mut scores = vec![0i32; X_MAX];
    let mut exists = vec![false; X_MAX];
    for &p in sorted_participants.iter() {
        // N
        exists[p as usize] = true;
        let divisors = get_divisors(p); // sqrt(X_MAX)
        for &d in divisors.iter() {
            if exists[d as usize] {
                // if p == 1; d == 1; then plus 1 minus 1 consequently zero
                scores[d as usize] += 1;
                scores[p as usize] -= 1;
            }
        }
    }

    participants.iter().map(|&p| scores[p as usize]).collect()
}

fn main() {
    let mut lines = stdin().lines();
    lines.next();
    let participants: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    println!(
        "{}",
        number_partition_game_result(&participants)
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
