fn split_by_3(line: &mut String) -> (i32, i32, i32) {
    let mut iter = line.split_ascii_whitespace();
    let p = (
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap(),
    );

    line.clear();

    p
}

fn get_minimum_boundary_access(
    start: (i32, i32),
    end: (i32, i32),
    circles: Vec<(i32, i32, i32)>,
) -> usize {
    circles
        .iter()
        .map(|c| {
            let (x, y, r) = c;

            let dis_s_sq = (start.0 - x).pow(2) + (start.1 - y).pow(2);
            let dis_e_sq = (end.0 - x).pow(2) + (end.1 - y).pow(2);

            let flag1 = r.pow(2) > dis_s_sq;
            let flag2 = r.pow(2) > dis_e_sq;

            if (flag1 && !flag2) || (flag2 && !flag1) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let t = line.trim().parse::<usize>().unwrap();
    line.clear();

    for _ in 0..t {
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace();
        let pos_start = (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        );
        let pos_end = (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        );
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let n = line.trim().parse::<usize>().unwrap();
        line.clear();
        let circles = (0..n)
            .map(|_| {
                std::io::stdin().read_line(&mut line).unwrap();
                split_by_3(&mut line)
            })
            .collect::<Vec<_>>();

        println!(
            "{}",
            get_minimum_boundary_access(pos_start, pos_end, circles)
        );
    }
}
