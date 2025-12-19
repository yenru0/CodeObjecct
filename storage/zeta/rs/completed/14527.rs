use std::io::stdin;

fn minimum_milking_time(mut cows: Vec<(i64, i64)>) -> i64 {
    cows.sort_by_key(|x| x.1 );

    let mut left = 0;
    let mut right = cows.len() - 1;

    let mut time_maxima = 0;

    while left <= right {
        let (left_count, left_milk) = cows[left];
        let (right_count, right_milk) = cows[right];

        if (left_count <= 0) {
            left += 1;
            continue;
        }
        if (right_count <= 0) {
            right -= 1;
            continue;
        }

        time_maxima = time_maxima.max(left_milk + right_milk);

        cows[left].0 -= 1;
        cows[right].0 -= 1;
    } 
    
    time_maxima
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let n: usize = line.trim().parse().unwrap();
    let cows = (0..n).map(
        |_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let mut iter = line
                .trim()
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap());
            
            (iter.next().unwrap(), iter.next().unwrap())
        }
    ).collect::<Vec<(i64, i64)>>();

    println!("{}", minimum_milking_time(cows));
}