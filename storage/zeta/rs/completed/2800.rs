use std::{collections::HashSet, io::stdin, iter};

fn get_all_parenthesis_pairs(s: &String) -> Vec<(usize, usize)> {
    let mut stack: Vec<usize> = vec![];

    let mut parenthesis_pairs = vec![];

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            let counter = stack.pop().unwrap();
            parenthesis_pairs.push((counter, i));
        }
    }

    parenthesis_pairs
}

fn filter_out(s: &Vec<char>, mask: &Vec<usize>) -> String {
    // mask is sorted
    if mask.len() == 0 {
        return s.iter().collect();
    } else {
        let mut new_s = String::new();
        let n = s.len();
        let m = mask.len();
        let mut i = 0;
        let mut j = 0;
        while i < n {
            let c = s[i];
            if j != m {
                if i < mask[j] {
                    new_s.push(c);
                } else if i == mask[j] {
                    j += 1;
                }
            } else {
                new_s.push(c);
            }

            i += 1;
        }
        new_s
    }
}

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut filtered_strings: HashSet<String> = HashSet::new();

    let pars = get_all_parenthesis_pairs(&s);
    let n = pars.len();
    let vectored_s = s.chars().collect::<Vec<char>>();

    for i in 1..(1 << n) {
        let mut mask = vec![];
        for (j, &(l, r)) in pars.iter().enumerate() {
            if i & (1 << j) != 0 {
                mask.push(l);
                mask.push(r);
            }
        }
        mask.sort();
        let new_s = filter_out(&vectored_s, &mask);
        filtered_strings.insert(new_s);
    }
    let mut filtered_strings = filtered_strings.into_iter().collect::<Vec<String>>();
    filtered_strings.sort();
    for s in  filtered_strings {
        println!("{}", s);
    }
    
}
