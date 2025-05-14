use std::io;

fn beacon_interval_count(n: i32, hs: Vec<i32>) -> i64 {
    let mut before: i32 = 0;

    let mut edge: Vec<i32> = Vec::new();

    for (i, &h) in hs.iter().enumerate() {
        if before < h {
            before = h;
            edge.push(i as i32);
        }
    }

    let mut cnt: i64 = 1;
    let mut before_edge: i32 = 0;
    for &ev in edge.iter() {
        cnt = (cnt * ((ev - before_edge + 1) as i64)) % 1_000_000_007;
        before_edge = ev;
    }

    cnt
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: i32;
    n = line.trim().parse().unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let hs: Vec<i32> = line.trim().split(' ').map(|x| x.parse().unwrap()).collect();

    println!("{}", beacon_interval_count(n, hs));
}
