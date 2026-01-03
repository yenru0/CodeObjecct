use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.clear();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let n = s.len();
    let mut sentence_map = vec![0; n];

    let mut curr_sentence = 0;
    for (i, c) in s.char_indices() {
        if c == '.' {
            sentence_map[i] = curr_sentence;
            curr_sentence += 1;
        } else {
            sentence_map[i] = curr_sentence;
        }
    }
    if n <= 25 {
        println!("{}", s);
    } else if sentence_map[11] == sentence_map[n - 12] {
        let first = &s[0..11];
        let last = &s[(n - 11)..n];
        let mut res = String::new();
        res.push_str(first);
        res.push_str("...");
        res.push_str(last);
        println!("{}", res);
    } else {
        let first = &s[0..9];
        let last = &s[(n - 10)..n];
        let mut res = String::new();
        res.push_str(first);
        res.push_str("......");
        res.push_str(last);
        println!("{}", res);
    }
}
