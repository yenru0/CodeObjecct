use std::cmp::min;
use std::i64;
use std::io::{read_to_string, stdin};
enum Room {
    Monster { atk: usize, hp: usize },
    Potion { atk: usize, hp: usize },
}

fn battle(atk: usize, mon_atk: usize, mon_hp: usize) -> usize {
    let d = mon_hp / atk;
    let r = mon_hp % atk;
    if d == 0 {
        0
    } else if r == 0 {
        (d - 1) * mon_atk
    } else {
        d * mon_atk
    }
}

fn simulate(hp_max: i64, hp_changes: &Vec<i64>) -> i64 {
    let mut hp = hp_max;

    for &dhp in hp_changes {
        hp += dhp;
        if hp <= 0 {
            return hp;
        }
        if hp > hp_max {
            hp = hp_max;
        }
    }
    hp
}

fn needed_hp_for_clearing(init_atk: usize, rooms: &Vec<Room>) -> i64 {
    let mut atk = init_atk;
    let hp_changes = rooms
        .iter()
        .map(|room| match room {
            Room::Monster {
                atk: mon_atk,
                hp: mon_hp,
            } => {
                let damage = battle(atk, *mon_atk, *mon_hp) as i64;
                -damage as i64
            }
            Room::Potion { atk: datk, hp: dhp } => {
                atk += *datk;
                *dhp as i64
            }
        })
        .collect::<Vec<i64>>();
    let mut lo = 1;
    let mut hi = i64::MAX - 1;

    let mut accept_min = hi;

    while lo <= hi {
        let mid = (lo + hi) / 2;
        let final_hp = simulate(mid, &hp_changes);

        if final_hp > 0 {
            hi = mid - 1;
            accept_min = min(accept_min, mid);
        } else {
            lo = mid + 1;
        }
    }
    accept_min
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = iter.next().unwrap();

    let init_atk = iter.next().unwrap();

    let rooms = (0..n)
        .map(|_| {
            let t = iter.next().unwrap();

            if t == 1 {
                Room::Monster {
                    atk: iter.next().unwrap(),
                    hp: iter.next().unwrap(),
                }
            } else {
                Room::Potion {
                    atk: iter.next().unwrap(),
                    hp: iter.next().unwrap(),
                }
            }
        })
        .collect::<Vec<Room>>();
    println!("{}", needed_hp_for_clearing(init_atk, &rooms));
}
