use std::io::stdin;

fn convert(text: Vec<char>) -> Vec<u64> {
    text.into_iter()
        .map(|x| match x {
            '_' => 0,
            'a'..='z' => (x as u8 - 'a' as u8 + 1) as u64,
            '.' => 27,
            _ => u64::MAX,
        })
        .collect()
}

fn revert(code: Vec<u64>) -> Vec<char> {
    code.into_iter()
        .map(|x| match x {
            0 => '_',
            1..=26 => (b'a' + x as u8 - 1) as char,
            27 => '.',
            _ => '*',
        })
        .collect()
}

fn decrypt(k: u64, ciphertext: Vec<char>) -> Vec<char> {
    let n = ciphertext.len();
    let ciphercode = convert(ciphertext);
    let mut plaincode = vec![0u64; n];

    for i in 0..n {
        let j = ((k * i as u64) % n as u64) as usize;
        plaincode[j] = ((ciphercode[i] as u64 + i as u64) % 28) as u64;
    }
    revert(plaincode)
}

fn main() {
    let mut line = String::new();

    loop {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        if line.starts_with("0") {
            break;
        }
        let mut iter = line.split(' ');
        let k = iter.next().unwrap().trim().parse::<u64>().unwrap();
        let msg = iter.next().unwrap().trim().chars().collect();
        println!("{}", decrypt(k, msg).into_iter().collect::<String>());
    }
}
