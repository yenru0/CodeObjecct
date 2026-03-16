use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let t = line.trim().parse::<usize>().unwrap();

    (0..t).for_each(|_| {
        line.clear();
        stdin().read_line(&mut line).unwrap();

        let n = line.trim().parse::<usize>().unwrap();
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let arr = line.trim().chars().collect::<Vec<_>>();

        let mut res = 0;
        for (i, &st) in arr.iter().enumerate() {
            if st == 'L' {
                res = i + 1;
                break;
            }
        }

        println!("{}", res);
    });
}
