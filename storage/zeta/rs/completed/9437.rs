use std::io::stdin;

fn main() {
    let mut line = String::new();
    loop {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let mut iter = line
            .trim()
            .split(' ')
            .map(|x| x.trim().parse::<usize>().unwrap());

        let n = iter.next().unwrap();
        if n == 0 {
            break;
        }
        let p = iter.next().unwrap();

        let mut res = vec![];
        if p % 2 == 0 {
            res.push(p - 1);
            res.push(n + 1 - p + 1);
            res.push(n + 1 - p);
        } else {
            res.push(p + 1);
            res.push(n - p);
            res.push(n + 1 - p);
        }

        res.sort();

        println!("{} {} {}", res[0], res[1], res[2]);
    }
}
