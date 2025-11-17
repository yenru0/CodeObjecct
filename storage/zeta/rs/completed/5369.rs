use std::{collections::VecDeque, io::stdin};

enum Location {
    Empty,
    Asteroid,
    Start,
    Destination,
}

struct Point {
    x: usize,
    y: usize,
}

fn find_minimum_steps(field: &Vec<Vec<Location>>, start: &Point, destination: &Point) -> i32 {
    let mut deq: VecDeque<(Point, i32)> = VecDeque::new();
    let (n, m) = (field.len(), field[0].len());
    let mut visited = vec![vec![false; field[0].len()]; field.len()];
    deq.push_back((
        Point {
            x: start.x,
            y: start.y,
        },
        0,
    ));

    while deq.is_empty() == false {
        let (current_point, steps) = deq.pop_front().unwrap();
        if current_point.x == destination.x && current_point.y == destination.y {
            return steps;
        }

        let directions = vec![
            (0isize, 1isize),
            (1isize, 0isize),
            (0isize, -1isize),
            (-1isize, 0isize),
        ];

        for direction in directions {
            let new_x = current_point.x as isize + direction.0;
            let new_y = current_point.y as isize + direction.1;

            if new_x >= 0 && new_x < m as isize && new_y >= 0 && new_y < n as isize {
                let new_x_u = new_x as usize;
                let new_y_u = new_y as usize;

                if visited[new_y_u][new_x_u] {
                    continue;
                } else {
                    match &field[new_y_u][new_x_u] {
                        Location::Asteroid => continue,
                        _ => {
                            visited[new_y_u][new_x_u] = true;
                            deq.push_back((
                                Point {
                                    x: new_x_u,
                                    y: new_y_u,
                                },
                                steps + 1,
                            ));
                        }
                    }
                }
            } else {
                continue;
            }
        }
    }

    return -1;
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t = line.trim().parse::<usize>().unwrap();
    (0..t).for_each(|_| {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let n: usize = line.trim().parse().unwrap();
        let mut starting_point = Point { x: 0, y: 0 };
        let mut destination_point = Point { x: 0, y: 0 };
        let field = (0..n)
            .map(|i| {
                line.clear();
                stdin().read_line(&mut line).unwrap();
                line.trim()
                    .char_indices()
                    .map(|item| {
                        let (j, c) = item;
                        if c == 's' {
                            starting_point = Point { x: j, y: i };
                            return Location::Start;
                        } else if c == 'd' {
                            destination_point = Point { x: j, y: i };
                            return Location::Destination;
                        } else if c == '*' {
                            return Location::Asteroid;
                        } else {
                            return Location::Empty;
                        }
                    })
                    .collect::<Vec<Location>>()
            })
            .collect::<Vec<Vec<Location>>>();
        let result = find_minimum_steps(&field, &starting_point, &destination_point);
        println!("{}", result);
    });
}
