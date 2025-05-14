use std::cmp::min;

fn find_type_of_palindrome(s: &String) -> i32 {
    let s_chars = s.chars().collect::<Vec<char>>();
    let mut dex: Vec<(i32, usize, usize)> = Vec::new();

    dex.push((0, 0, s_chars.len() - 1));

    let mut last_rm = 4;

    while !dex.is_empty() {
        let (rm, start, end) = dex.pop().unwrap();
        if rm >= 4 || start >= end {
            last_rm = min(last_rm, rm);
            continue;
        }
        if s_chars[start] == s_chars[end] {
            dex.push((rm, start + 1, end - 1));
        } else {
            dex.push((rm + 1, start + 1, end));
            dex.push((rm + 1, start, end - 1));
        }
    }

    return if (last_rm <= 3) { last_rm } else { -1 };
}

fn main() {
    let mut s = String::new();

    match std::io::stdin().read_line(&mut s) {
        Ok(_) => s = s.trim().to_string(),
        Err(_) => return,
    };

    println!("{}", find_type_of_palindrome(&s));
}
