use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut iter = line.split(' ');
    let s = iter.next().unwrap().trim().to_string();
    let n = s.len();
    let mut represent = 0u64;
    for (i, c) in s.chars().enumerate() {
        if c == 'B' {
            represent |= 1 << i;
        }
    }

    let day = iter.next().unwrap().trim().parse::<u64>().unwrap();

    represent += day;
    represent %= 1 << n;

    for i in 0..n {
        if (represent & (1 << i)) != 0 {
            print!("B");
        } else {
            print!("A");
        }
    }
}
