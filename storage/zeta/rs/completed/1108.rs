use std::{collections::HashMap, io::stdin};

fn score(graph: &Vec<Vec<usize>>, guess: usize) -> usize {
    let n = graph.len();

    let mut reach = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == 1 {
                reach[i][j] = true;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if reach[i][k] && reach[k][j] {
                    reach[i][j] = true;
                }
            }
        }
    }

    let mut memo = vec![usize::MAX; n];

    fn get_score(
        curr: usize,
        n: usize,
        graph: &Vec<Vec<usize>>,
        reachable: &Vec<Vec<bool>>,
        memo: &mut Vec<usize>,
    ) -> usize {
        if memo[curr] != usize::MAX {
            return memo[curr];
        }

        let mut total = 1;

        for next in 0..n {
            if graph[curr][next] == 1 && !reachable[next][curr] {
                total += get_score(next, n, graph, reachable, memo);
            }
        }

        memo[curr] = total;
        total
    }

    get_score(guess, n, graph, &reach, &mut memo)
}
fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    let n = line.trim().parse::<usize>().unwrap();

    let mut name_map = HashMap::new();

    let mut idx: usize = 0;
    let edges_info = (0..n)
        .map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let mut iter = line.trim().split(' ');

            let start = iter.next().unwrap().trim().to_string();
            if !name_map.contains_key(&start) {
                name_map.insert(start.clone(), idx);
                idx += 1;
            }
            let cnt = iter.next().unwrap().parse::<usize>().unwrap();
            let dests = (0..cnt)
                .map(|_| {
                    let t = iter.next().unwrap().trim().to_string();
                    if !name_map.contains_key(&t) {
                        name_map.insert(t.clone(), idx);
                        idx += 1;
                    }
                    t
                })
                .collect::<Vec<String>>();

            (start, dests)
        })
        .collect::<Vec<(String, Vec<String>)>>();

    let mut graph = vec![vec![0; idx]; idx];

    for (start, dests) in edges_info {
        let start_idx = name_map[&start];
        for dest in dests {
            let dest_idx = name_map[&dest];
            graph[start_idx][dest_idx] = 1;
        }
    }

    line.clear();
    stdin().read_line(&mut line).unwrap();
    let guess = line.trim().to_string();
    let guess_idx = name_map[&guess];

    println!("{}", score(&graph, guess_idx));
}
