use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let n = iter.next().unwrap() as usize;

    let movies = (0..n)
        .map(|i| {
            (
                i,
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut res: i64 = movies.iter().map(|x| x.1).sum();
    let mut secondary_movies = movies.iter().map(|x| (x.0, x.2 - x.1)).collect::<Vec<_>>();
    secondary_movies.sort_by_key(|x| x.1);
    let mut third_movies = movies.iter().map(|x| (x.0, x.3 - x.1)).collect::<Vec<_>>();
    third_movies.sort_by_key(|x| x.1);
    let s1 = secondary_movies[secondary_movies.len() - 1];
    let s2 = secondary_movies[secondary_movies.len() - 2];
    let t1 = third_movies[third_movies.len() - 1];
    let t2 = third_movies[third_movies.len() - 2];

    let cand = if s1.0 == t1.0 {
        if s1.1 + t2.1 > s2.1 + t1.1 {
            res += s1.1 + t2.1;
            (s1.0, t2.0)
        } else {
            res += s2.1 + t1.1;
            (s2.0, t1.0)
        }
    } else {
        res += s1.1 + t1.1;
        (s1.0, t1.0)
    };
    println!("{}", res);
    println!("{} {}", cand.0 + 1, cand.1 + 1);
}
