use std::{io::stdin, iter::zip};

pub struct Road {
    length: usize,
    max_speed: usize,
}

fn get_road_speed_by_distance(road: &Vec<Road>) -> Vec<usize> {
    let mut speed_by_distance: Vec<usize> = Vec::new();

    for road_segment in road.iter() {
        for _ in 0..road_segment.length {
            speed_by_distance.push(road_segment.max_speed);
        }
    }

    return speed_by_distance;
}

fn get_violation_count(road: &Vec<Road>, road_run: &Vec<Road>) -> usize {
    let speed_limit = get_road_speed_by_distance(road);
    let run_speed = get_road_speed_by_distance(road_run);

    if (speed_limit.len() != run_speed.len()) {
        panic!("Road and run lengths do not match!");
    }

    return 
        zip(speed_limit, run_speed)
            .map(|(limit, speed)| speed.saturating_sub(limit) )
            .max().unwrap();
}

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();
    let (n, m) = {
        let mut iter = line.split(' ').map(|x| x.trim().parse::<usize>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let road: Vec<Road> = {
        (0..n).map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let mut iter = line.split(' ').map(|x| x.trim().parse::<usize>().unwrap());

            Road {
                length: iter.next().unwrap(),
                max_speed: iter.next().unwrap(),
            }
        })
    }
    .collect::<Vec<Road>>();

    let road_run = {
        (0..m).map(|_| {
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let mut iter = line.split(' ').map(|x| x.trim().parse::<usize>().unwrap());

            Road {
                length: iter.next().unwrap(),
                max_speed: iter.next().unwrap(),
            }
        })
    }
    .collect::<Vec<Road>>();

    println!("{}", get_violation_count(&road, &road_run));
}
