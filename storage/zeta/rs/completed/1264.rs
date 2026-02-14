use std::io::stdin;

fn main() {
    let mut line = String::new();

    loop {
        line.clear();
        stdin().read_line(&mut line).unwrap();

        let s = line.chars().collect::<Vec<char>>();
        if s[0] == '#' {
            break;
        }
        let cnt = s
            .iter()
            .map(|x| x.to_ascii_lowercase())
            .filter(|x| match x {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            })
            .count();

        println!("{}", cnt);
    }
}
