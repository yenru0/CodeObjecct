use std::io::stdin;

fn get_clap(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }
    let str_n = n.to_string();
    let x = str_n
        .chars()
        .filter(|c| *c == '3' || *c == '6' || *c == '9')
        .count();

    return (x as i32) + get_clap(n - 1);
}

fn main() {
    let mut str_n = String::new();
    stdin().read_line(&mut str_n).expect("err");

    let n: i32 = str_n.trim().parse::<i32>().unwrap();

    println!("{}", get_clap(n));
}
