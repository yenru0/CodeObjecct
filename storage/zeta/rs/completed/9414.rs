use std::io::{read_to_string, stdin};

fn main() {
    let temp = read_to_string(stdin()).unwrap();

    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let tc = iter.next().unwrap();

    const MONEY_LIMIT_U64: u64 = 2_500_000;
    const MONEY_LIMIT_F64: f64 = MONEY_LIMIT_U64 as f64;

    for _ in 0..tc {
        let prices = {
            let mut prices = vec![];

            let mut value = iter.next().unwrap() as u64;

            while value != 0 {
                prices.push(value);
                value = iter.next().unwrap() as u64;
            }

            prices.sort();
            prices.reverse();
            prices
        };

        let mut flag_expensive = false;
        let mut total_price = 0;
        for (t, &price) in prices.iter().enumerate() {
            let t = t + 1;
            let critea = MONEY_LIMIT_F64.powf(1.0 / (t as f64)).ceil() as u64;
            if price > critea {
                flag_expensive = true;
                break;
            }

            total_price += price.pow(t as u32);
        }
        if total_price > MONEY_LIMIT_U64 {
            flag_expensive = true;
        }

        if flag_expensive {
            println!("Too expensive");
        } else {
            println!("{}", 2 * total_price);
        }
    }
}
