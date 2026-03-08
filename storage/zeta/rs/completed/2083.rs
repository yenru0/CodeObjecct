use std::io::stdin;

pub struct Person {
    name: String,
    age: u32,
    weight: u32,
}

impl Person {
    fn is_adult(&self) -> bool {
        self.age >= 18 || self.weight >= 80
    }

    fn is_null(&self) -> bool {
        self.age == 0 && self.weight == 0
    }
}

fn main() {
    let mut line = String::new();

    loop {
        line.clear();
        stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim_ascii_end().split(' ');
        let p = Person {
            name: iter.next().unwrap().to_string(),
            age: iter.next().unwrap().parse().unwrap(),
            weight: iter.next().unwrap().parse().unwrap(),
        };
        if p.is_null() {
            break;
        } else if p.is_adult() {
            println!("{} Senior", p.name);
        } else {
            println!("{} Junior", p.name);
        }
    }
}
