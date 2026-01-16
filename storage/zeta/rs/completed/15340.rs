use std::io::stdin;

fn min_fee(call: u64, data: u64) -> u64 {
    [
        call * 30 + data * 40,
        call * 35 + data * 30,
        call * 40 + data * 20,
    ]
    .into_iter()
    .min()
    .unwrap()
}

fn main() {
    let mut line = String::new();
    loop {
        line.clear();

        stdin().read_line(&mut line).unwrap();

        let mut iter = line
            .trim_ascii_end()
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap());

        let (call, data) = (iter.next().unwrap(), iter.next().unwrap());

        if call == 0 && data == 0 {
            break;
        } else {
            println!("{}", min_fee(call, data));
        }
    }
}
