use std::{
    cmp::Ordering,
    io::{read_to_string, stdin},
};

fn gcd(a: u64, b: u64) -> u64 {
    let mut r = 0;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    a
}


#[derive(Debug)]
pub struct Rational {
    p: u64,
    q: u64,
}

impl Rational {
    fn new(p: u64, q: u64) -> Self {
        Rational {
            p: p / gcd(p, q),
            q: q / gcd(p, q),
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.q == other.q
    }

    fn ne(&self, other: &Self) -> bool {
        self.p != other.p && self.q != other.q
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.p * other.q).cmp(&(other.p * self.q)))
    }
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let n = iter.next().unwrap() as usize;

    let xs = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<_>>();
    let ys = (0..n).map(|_| iter.next().unwrap()).collect::<Vec<_>>();

    let rs = xs
        .iter()
        .zip(ys.iter())
        .map(|(&x, &y)| Rational::new(y, x))
        .collect::<Vec<_>>();

    let mut max_value = Rational { p: 0, q: 1 };
    let mut max_cons = 0;
    let mut curr_cons = 0;
    for i in rs {
        if i > max_value {
            curr_cons = 1;
            max_cons = 0;
            max_value = i;
        } else if i == max_value {
            if curr_cons > 0 {
                curr_cons += 1;
            } else {
                curr_cons += 1;
            }
        } else {
            max_cons = max_cons.max(curr_cons);
            curr_cons = 0;
        }
    }

    if curr_cons > 0 {
        max_cons = max_cons.max(curr_cons);
    }
    println!("{} {}", max_value.p, max_value.q);
    println!("{}", max_cons);
}
