use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    println!(
        "{}",
        (0..n)
            .map(|_| {
                line.clear();
                stdin().read_line(&mut line).unwrap();
                line.trim_end().parse::<i32>().unwrap()
            })
            .sum::<i32>()
    );
}
