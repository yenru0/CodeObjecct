use std::io::{stdin, BufRead};

const R: usize = 1_000_000_007;

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    pub fn next(&mut self) -> Option<String> {
        loop {
            if let Some(token) = self.buffer.pop() {
                return Some(token);
            }

            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => return None,
                Ok(_) => {
                    self.buffer = line.split_whitespace().rev().map(String::from).collect();
                }
                Err(_) => return None,
            }
        }
    }

    pub fn next_usize(&mut self) -> Option<usize> {
        self.next()?.parse().ok()
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.next()?.chars().next()
    }
}

fn get_partial_sent(
    partial_sent_count: &mut Vec<usize>,
    cnt_words_by_syllables: &Vec<usize>,
    curr: usize,
) -> usize {
    if partial_sent_count[curr] != usize::MAX {
        partial_sent_count[curr]
    } else {
        let mut s = 0;

        for i in 1..=curr {
            let cnt_words_at = cnt_words_by_syllables[i];
            s += cnt_words_at
                * get_partial_sent(partial_sent_count, cnt_words_by_syllables, curr - i);
            s %= R;
        }

        partial_sent_count[curr] = s;

        s
    }
}

fn get_all_poems(
    n: usize,
    m: usize,
    k: usize,
    words: &Vec<(usize, usize)>,
    rhyme_types: &[usize; 26],
) -> usize {
    let mut total_sent_by_cat = vec![0; n];
    let mut partial_sent_count = vec![usize::MAX; k];
    partial_sent_count[0] = 1;
    let mut cnt_words_by_syllables = vec![0; k + 1];

    for &(s, _) in words.iter() {
        cnt_words_by_syllables[s] += 1;
    }

    for &(s, c) in words.iter() {
        let curr = k - s;
        let s = get_partial_sent(&mut partial_sent_count, &cnt_words_by_syllables, curr);
        total_sent_by_cat[c - 1] += s;
        total_sent_by_cat[c - 1] %= R;
    }
    // Devide Powering
    let mut pow2_memo = vec![vec![usize::MAX; 20]; n];

    for i in 0..n {
        let sent = total_sent_by_cat[i];
        if sent == 0 {
            continue;
        }

        let mut curr = sent;
        pow2_memo[i][0] = 1;
        for j in 1..20 {
            pow2_memo[i][j] = curr;
            curr = curr * curr;
            curr %= R;
        }
    }

    let fast_power = |idx: usize, n: usize| {
        let x = total_sent_by_cat[idx];
        if x == 0 {
            return 0;
        }
        let mut curr = n;
        let mut s = 1;
        let mut i = 1;
        while curr > 0 {
            if curr & 1 == 1 {
                s *= pow2_memo[idx][i];
                s %= R;
            }
            curr >>= 1;
            i += 1;
        }

        s
    };

    // Logic
    let mut res = 1;
    for &rhyme in rhyme_types.iter() {
        if rhyme == 0 {
            continue;
        }

        let s: usize = (0..n).map(|idx| fast_power(idx, rhyme)).sum::<usize>() % R;

        res *= s;
        res %= R;
    }
    res
}

fn main() {
    let mut scan = Scanner::new(stdin().lock());

    let (n, m, k) = (
        scan.next_usize().unwrap(),
        scan.next_usize().unwrap(),
        scan.next_usize().unwrap(),
    );

    let words = (0..n)
        .map(|_| (scan.next_usize().unwrap(), scan.next_usize().unwrap()))
        .collect::<Vec<(usize, usize)>>();

    let poem_structure = (0..m)
        .map(|_| (scan.next_char().unwrap() as u8 - 'A' as u8) as usize)
        .collect::<Vec<usize>>();

    let rhyme_types = {
        let mut rhyme_types = [0usize; 26];

        for t in poem_structure {
            rhyme_types[t] += 1;
        }
        rhyme_types
    };

    println!("{}", get_all_poems(n, m, k, &words, &rhyme_types))
}
