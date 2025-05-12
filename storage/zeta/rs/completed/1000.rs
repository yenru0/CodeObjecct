use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let arr: Vec<i32> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("{}", arr[0] + arr[1]);
}
