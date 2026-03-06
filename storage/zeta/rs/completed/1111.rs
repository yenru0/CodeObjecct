use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();

    line.clear();

    stdin().read_line(&mut line).unwrap();
    let arr = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if n <= 1 {
        println!("A");
    } else if n == 2 {
        if arr[0] == arr[1] {
            println!("{}", arr[0]);
        } else {
            println!("A");
        }
    } else {
        if arr[0] == arr[1] {
            let same_flag = arr.iter().all(|&x| x == arr[0]);

            if same_flag {
                println!("{}", arr[0]);
            } else {
                println!("B");
            }
        } else {
            if arr[1] == arr[2] {
                for i in 2..n - 1 {
                    if arr[i + 1] == arr[i] {
                        continue;
                    } else {
                        println!("B");
                        return;
                    }
                }
                println!("{}", arr[n - 1]);
            } else {
                let d_a = arr[2] - arr[1];
                let d_b = arr[1] - arr[0];

                if d_a % d_b == 0 {
                    let a = d_a / d_b;
                    let b = arr[1] - a * arr[0];
                    let mut flag = true;
                    for i in 1..n - 1 {
                        if arr[i + 1] == arr[i] * a + b {
                            continue;
                        } else {
                            flag = false;
                            break;
                        }
                    }

                    if flag {
                        println!("{}", arr[n - 1] * a + b);
                    } else {
                        println!("B");
                    }
                } else {
                    println!("B");
                }
            }
        }
    }
}
