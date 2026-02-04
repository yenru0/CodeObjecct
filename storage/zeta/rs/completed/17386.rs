use std::io::{read_to_string, stdin};

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Line {
    p1: Point,
    p2: Point,
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let k = (p1.y - p2.y) * (p2.x - p3.x) - (p1.x - p2.x) * (p2.y - p3.y);

    if k > 0.0 {
        1
    } else if k < 0.0 {
        -1
    } else {
        0
    }
}

fn check_intersect(l1: &Line, l2: &Line) -> bool {
    let c1 = ccw(&l1.p1, &l1.p2, &l2.p1) * ccw(&l1.p1, &l1.p2, &l2.p2);
    let c2 = ccw(&l2.p1, &l2.p2, &l1.p1) * ccw(&l2.p1, &l2.p2, &l1.p2);

    if c1 <= 0 && c2 <= 0 {
        true
    } else {
        false
    }
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<f64>().unwrap());

    let l1 = {
        Line {
            p1: Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            },
            p2: Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            },
        }
    };
    let l2 = {
        Line {
            p1: Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            },
            p2: Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            },
        }
    };

    println!("{}", check_intersect(&l1, &l2) as u8);
}
