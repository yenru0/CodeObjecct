use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let arr = {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        line.split(' ')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    };

    /*
       0
       a1                   // ds = a1                   // dds = a1
       4 * a1 + a2          // ds = 3 * a1 + a2          // dds = 2 * a1 + a2
       9 * a1 + 4 * a2 + a3 // ds = 5 * a1 + 3 * a2 + a3 // dds = 2 * a1 + 2 * a2 + a3 // ds <- dds + ds
    */

    let mut ds = 0i64;
    let mut dds = 0i64;

    let mut sum = 0i64;
    let square_weighted_arr = (0..n)
        .map(|i| {
            dds = if i != 0 {
                dds + arr[i] + arr[i - 1]
            } else {
                arr[i]
            };
            ds = ds + dds;
            sum += ds;
            sum
        })
        .collect::<Vec<i64>>();

    println!(
        "{}",
        square_weighted_arr
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
