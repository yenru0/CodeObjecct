use std::io::stdin;

fn optimized_pos(poses: &Vec<(f64, f64)>) -> (f64, f64) {
    let n = poses.len();
    if n == 0 {
        return (0.0, 0.0);
    }
    if n == 1 {
        return poses[0];
    }

    let mut sorted_poses = poses.clone();
    sorted_poses.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    });

    fn cross_product(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    }

    let mut hull = Vec::new();
    for &p in &sorted_poses {
        while hull.len() >= 2 && cross_product(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0
        {
            hull.pop();
        }
        hull.push(p);
    }
    let lower_len = hull.len();
    for i in (0..sorted_poses.len() - 1).rev() {
        let p = sorted_poses[i];
        while hull.len() > lower_len
            && cross_product(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0
        {
            hull.pop();
        }
        hull.push(p);
    }
    hull.pop();

    let dist_sq = |p1: (f64, f64), p2: (f64, f64)| (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2);

    let mut min_max_dist_sq = f64::MAX;
    let mut best_pos = poses[0];

    for &p in poses {
        let mut max_d_sq = 0.0;
        for &h in &hull {
            let d_sq = dist_sq(p, h);
            if d_sq > max_d_sq {
                max_d_sq = d_sq;
            }
        }

        if max_d_sq < min_max_dist_sq {
            min_max_dist_sq = max_d_sq;
            best_pos = p;
        }
    }

    best_pos
}
fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let n = line.trim().parse::<usize>().unwrap();

    let mut poses_infra = (0..n)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let mut iter = line.split(' ').map(|s| s.trim().parse::<f64>().unwrap());

            let (a, b) = (iter.next().unwrap(), iter.next().unwrap());
            (a, b)
        })
        .collect::<Vec<(f64, f64)>>();

    let res = optimized_pos(&mut poses_infra);
    println!("{:.7} {:.7}", res.0, res.1);
}
