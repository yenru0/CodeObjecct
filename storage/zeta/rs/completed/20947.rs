use std::io::{read_to_string, stdin, stdout, BufWriter, Write};

#[derive(Debug, Clone, Copy)]
pub enum Land {
    Building,
    Debris,
    Bomb,
    Empty,
}

impl Land {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            'O' => Self::Building,
            'X' => Self::Debris,
            'B' => Self::Bomb,
            _ => Self::Empty,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Land::Building => 'O',
            Land::Debris => 'X',
            Land::Bomb => 'B',
            Land::Empty => '.',
        }
    }
}

fn find_bombs(n: usize, mut city: Vec<Vec<Land>>) -> Vec<Vec<Land>> {
    let mut ables: Vec<Vec<bool>> = city
        .iter()
        .map(|row| {
            row.iter()
                .map(|land| match land {
                    Land::Empty => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    for i in 0..n {
        for j in 0..n {
            let land = city[i][j];
            match land {
                Land::Empty => {}
                Land::Debris => {}
                Land::Bomb => {}
                Land::Building => {
                    for iu in (0..i).rev() {
                        let search = city[iu][j];
                        match search {
                            Land::Empty => {
                                ables[iu][j] = false;
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    for iu in (i + 1..n) {
                        let search = city[iu][j];
                        match search {
                            Land::Empty => {
                                ables[iu][j] = false;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    for ju in (0..j).rev() {
                        let search = city[i][ju];
                        match search {
                            Land::Empty => {
                                ables[i][ju] = false;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    for ju in (j + 1..n) {
                        let search = city[i][ju];
                        match search {
                            Land::Empty => {
                                ables[i][ju] = false;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            let able = ables[i][j];
            if able {
                city[i][j] = Land::Bomb;
            }
        }
    }

    city
}

fn main() {
    let temp = read_to_string(stdin()).unwrap();
    let mut iter = temp.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let city: Vec<Vec<Land>> = (0..n)
        .map(|_| {
            iter.next()
                .unwrap()
                .trim()
                .chars()
                .map(|ch| Land::from_char(ch))
                .collect()
        })
        .collect();

    let city_with_bomb = find_bombs(n, city);
    let mut stdout = BufWriter::new(stdout().lock());
    city_with_bomb.iter().for_each(|x| {
        x.iter().for_each(|land| {
            stdout.write_all(&[land.to_char() as u8]).unwrap();
        });
        stdout.write_all(&['\n' as u8]).unwrap();
    });
}
