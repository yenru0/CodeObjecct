use std::io::stdin;

struct InquireManager {
    line: String,
}

impl InquireManager {
    fn new() -> Self {
        InquireManager {
            line: String::new(),
        }
    }

    fn inquire_to_a(&mut self, x: usize) -> usize {
        println!("? A {}", x);
        self.get()
    }

    fn inquire_to_b(&mut self, x: usize) -> usize {
        println!("? B {}", x);
        self.get()
    }

    fn get(&mut self) -> usize {
        stdin().read_line(&mut self.line).unwrap();
        let r = self.line.trim().parse::<usize>().unwrap();
        self.line.clear();
        r
    }
}

fn main() {
    let mut inq = InquireManager::new();
    let n = inq.get();

    let res = if n <= 4 {
        let mut v = vec![];
        for i in 1..=n {
            v.push(inq.inquire_to_a(i));
            v.push(inq.inquire_to_b(i));
        }
        v.sort();
        v[n - 1]
    } else {
        let mut midpoint = 0;
        let mut lo = 1;
        let mut hi = n;
        let mut a_of = 0;
        let mut b_of = 0;
        let mut a_hi = inq.inquire_to_a(n);
        let mut b_hi = inq.inquire_to_b(n);
        while lo < hi {
            let mid = (hi + lo) / 2;
            a_of = inq.inquire_to_a(mid);
            b_of = inq.inquire_to_b(n - mid);
            if a_of == b_of {
                midpoint = a_of;
                break;
            } else if a_of > b_of {
                hi = mid;
                a_hi = a_of;
            } else {
                // a_of < b_of
                lo = mid + 1;
                b_hi = b_of;
            }
        }

        if midpoint != 0 {
            midpoint
        } else {
            if a_hi < b_hi {
                a_hi
            } else {
                b_hi
            }
        }
    };

    println!("! {}", res);
}
