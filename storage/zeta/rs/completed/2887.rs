use std::io::{stdin, BufRead};

fn planets_find(mut parents: &Vec<usize>, i: usize) -> usize {
    let mut node = parents[i];
    while node != parents[node] {
        node = parents[node];
    }
    node
}

fn planets_union(parents: &mut Vec<usize>, i: usize, j: usize) -> bool {
    let ir = planets_find(parents, i);
    let jr = planets_find(parents, j);
    if ir == jr {
        return false;
    }
    if ir < jr {
        parents[jr] = ir;
    } else {
        parents[ir] = jr;
    }
    true
}

fn get_minimum_cost_planet_tunnel(v: &Vec<(i32, i32, i32)>) -> usize {
    let mut x_axis_find_base = v
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x.0, i))
        .collect::<Vec<(i32, usize)>>();
    let mut y_axis_find_base = v
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x.1, i))
        .collect::<Vec<(i32, usize)>>();
    let mut z_axis_find_base = v
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x.2, i))
        .collect::<Vec<(i32, usize)>>();

    x_axis_find_base.sort();
    y_axis_find_base.sort();
    z_axis_find_base.sort();

    let mut parents = (0..v.len()).map(|i| i).collect::<Vec<usize>>();

    let mut candidate_edges: Vec<(usize, (usize, usize))> = Vec::new();
    for i in 0..v.len() - 1 {
        let ahead = x_axis_find_base[i + 1];
        let now = x_axis_find_base[i];
        candidate_edges.push(((ahead.0 - now.0) as usize, (now.1, ahead.1)));
    }
    for i in 0..v.len() - 1 {
        let ahead = y_axis_find_base[i + 1];
        let now = y_axis_find_base[i];
        candidate_edges.push(((ahead.0 - now.0) as usize, (now.1, ahead.1)));
    }
    for i in 0..v.len() - 1 {
        let ahead = z_axis_find_base[i + 1];
        let now = z_axis_find_base[i];
        candidate_edges.push(((ahead.0 - now.0) as usize, (now.1, ahead.1)));
    }

    candidate_edges.sort();
    let mut total_cost = 0;
    for (cost, (i, j)) in candidate_edges {
        if planets_union(&mut parents, i, j) {
            total_cost += cost;
        }
    }

    total_cost
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim_end().parse::<usize>().unwrap();
    line.clear();

    let v: Vec<(i32, i32, i32)> = (0..n)
        .map(|_| {
            stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_ascii_whitespace();

            let t: (i32, i32, i32) = (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            );

            line.clear();

            t
        })
        .collect();

    println!("{}", get_minimum_cost_planet_tunnel(&v));
}
