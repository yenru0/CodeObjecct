use core::panic;
use std::{
    io::stdin,
    ops::{Add, Div, Mul, Neg, Sub},
};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 && b == 0 {
        return 0;
    }
    let cd = gcd(a, b);

    (a / cd) * b
}

pub struct Rational {
    sign: bool,
    p: u64,
    q: u64,
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        let (res_sign, res_p, res_q) =
            add_rational_component(self.sign, self.p, self.q, other.sign, other.p, other.q);
        Rational {
            sign: res_sign,
            p: res_p,
            q: res_q,
        }
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        let (res_sign, res_p, res_q) =
            sub_rational_component(self.sign, self.p, self.q, other.sign, other.p, other.q);
        Rational {
            sign: res_sign,
            p: res_p,
            q: res_q,
        }
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        let (res_sign, res_p, res_q) =
            mul_rational_component(self.sign, self.p, self.q, other.sign, other.p, other.q);
        Rational {
            sign: res_sign,
            p: res_p,
            q: res_q,
        }
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        let (res_sign, res_p, res_q) =
            div_rational_component(self.sign, self.p, self.q, other.sign, other.p, other.q);
        Rational {
            sign: res_sign,
            p: res_p,
            q: res_q,
        }
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational {
            sign: !self.sign,
            p: self.p,
            q: self.q,
        }
    }
}

fn regular_rational_component(p: u64, q: u64) -> (u64, u64) {
    let cd = gcd(p, q);
    (p / cd, q / cd)
}

fn add_rational_component(
    sign_a: bool,
    p_a: u64,
    q_a: u64,
    sign_b: bool,
    p_b: u64,
    q_b: u64,
) -> (bool, u64, u64) {
    if p_a == 0 {
        return (sign_b, p_b, q_b);
    }
    if p_b == 0 {
        return (sign_a, p_a, q_a);
    }

    let common_q = lcm(q_a, q_b);

    let val_a = (p_a) * (common_q / q_a);
    let val_b = (p_b) * (common_q / q_b);

    let mut res_p: u64;
    let mut res_q: u64 = common_q;
    let res_sign: bool;
    if sign_a == sign_b {
        res_p = val_a + val_b;
        res_sign = sign_a;
    } else {
        if val_a > val_b {
            res_p = val_a - val_b;
            res_sign = sign_a;
        } else if val_b > val_a {
            res_p = val_b - val_a;
            res_sign = sign_b;
        } else {
            res_p = 0;
            res_q = 1;
            res_sign = false;
            return (res_sign, res_p, res_q);
        }
    }

    (res_p, res_q) = regular_rational_component(res_p, res_q);
    (res_sign, res_p, res_q)
}

fn sub_rational_component(
    /* a - b */
    sign_a: bool,
    p_a: u64,
    q_a: u64,
    sign_b: bool,
    p_b: u64,
    q_b: u64,
) -> (bool, u64, u64) {
    add_rational_component(sign_a, p_a, q_a, !sign_b, p_b, q_b)
}

fn mul_rational_component(
    sign_a: bool,
    p_a: u64,
    q_a: u64,
    sign_b: bool,
    p_b: u64,
    q_b: u64,
) -> (bool, u64, u64) {
    if p_a == 0 || p_b == 0 {
        return (false, 0, 1);
    }

    let res_sign = sign_a ^ sign_b;

    let mut res_p = p_a * p_b;
    let mut res_q = q_a * q_b;

    (res_p, res_q) = regular_rational_component(res_p, res_q);

    (res_sign, res_p, res_q)
}

fn div_rational_component(
    /* a / b */
    sign_a: bool,
    p_a: u64,
    q_a: u64,
    sign_b: bool,
    p_b: u64,
    q_b: u64,
) -> (bool, u64, u64) {
    mul_rational_component(sign_a, p_a, q_a, sign_b, q_b, p_b)
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expr {
    Rational(Rational),
    Var,
    Op {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub enum SolutionSignal {
    No,
    Multiple,
    Unique(Rational),
    Undef,
}

fn solve(ops: Vec<&str>) -> SolutionSignal {
    let mut solution = Rational {
        sign: false,
        p: 0,
        q: 1,
    };

    let mut stack: Vec<Expr> = vec![];

    let mut signal = SolutionSignal::Undef;

    ops.iter().for_each(|&x| match x.parse::<u64>() {
        Ok(u) => {
            stack.push(Expr::Rational(Rational {
                sign: false,
                p: u,
                q: 1,
            }));
        }
        Err(_) => match x.chars().next().unwrap() {
            'X' | 'x' => stack.push(Expr::Var),
            '+' => {
                let right: Expr = stack.pop().unwrap();
                let left: Expr = stack.pop().unwrap();
                match (left, right) {
                    (Expr::Rational(l_val), Expr::Rational(r_val)) => {
                        stack.push(Expr::Rational(l_val + r_val))
                    }
                    (l, r) => stack.push(Expr::Op {
                        op: Operator::Add,
                        left: Box::new(l),
                        right: Box::new(r),
                    }),
                }
            }
            '*' => {
                let right: Expr = stack.pop().unwrap();
                let left: Expr = stack.pop().unwrap();
                match (left, right) {
                    (Expr::Rational(l_val), Expr::Rational(r_val)) => {
                        stack.push(Expr::Rational(l_val * r_val))
                    }
                    (l, r) => stack.push(Expr::Op {
                        op: Operator::Mul,
                        left: Box::new(l),
                        right: Box::new(r),
                    }),
                }
            }
            '/' => {
                let right: Expr = stack.pop().unwrap();
                let left: Expr = stack.pop().unwrap();
                match (left, right) {
                    (Expr::Rational(l_val), Expr::Rational(r_val)) => {
                        stack.push(Expr::Rational(l_val / r_val))
                    }
                    (l, r) => stack.push(Expr::Op {
                        op: Operator::Div,
                        left: Box::new(l),
                        right: Box::new(r),
                    }),
                }
            }
            '-' => {
                let right: Expr = stack.pop().unwrap();
                let left: Expr = stack.pop().unwrap();
                match (left, right) {
                    (Expr::Rational(l_val), Expr::Rational(r_val)) => {
                        stack.push(Expr::Rational(l_val - r_val))
                    }
                    (l, r) => stack.push(Expr::Op {
                        op: Operator::Sub,
                        left: Box::new(l),
                        right: Box::new(r),
                    }),
                }
            }
            _ => {
                panic!("Invalid operator");
            }
        },
    });

    let mut root = stack.pop().unwrap();

    if let Expr::Rational(r) = root {
        if r.p == 0 {
            signal = SolutionSignal::Multiple;
        } else {
            signal = SolutionSignal::No;
        }
    } else {
        while let Expr::Op { op, left, right } = root {
            match (*left, *right) {
                (Expr::Rational(r), right_expr) => {
                    match op {
                        Operator::Add => {
                            solution = solution - r;
                        }
                        Operator::Sub => {
                            solution = -solution + r;
                        }
                        Operator::Mul => {
                            // a * f = s  => f = s / a
                            if r.p == 0 {
                                // 0 * f = s
                                if (solution.p == 0) {
                                    signal = SolutionSignal::Multiple;
                                } else {
                                    signal = SolutionSignal::No;
                                }
                                break;
                            } else {
                                solution = solution / r;
                            }
                        } // a / (a / f) = 0
                        Operator::Div => {
                            // a / f = s => f = a / s
                            if r.p == 0 && solution.p != 0 {
                                signal = SolutionSignal::No;
                                break;
                            } else if r.p == 0 && solution.p == 0 {
                                signal = SolutionSignal::Multiple;
                                break;
                            } else if r.p != 0 && solution.p == 0 {
                                signal = SolutionSignal::No;
                                break;
                            } else {
                                solution = r / solution;
                            }
                        }
                    }
                    root = right_expr;
                }
                (left_expr, Expr::Rational(r)) => {
                    // Right is Rational, move it to solution based on operator
                    match op {
                        Operator::Add => {
                            solution = solution - r;
                        }
                        Operator::Sub => {
                            solution = solution + r;
                        }
                        Operator::Mul => {
                            // f * a = s => f = s / a
                            if r.p == 0 {
                                // 0 * f = s
                                if (solution.p == 0) {
                                    signal = SolutionSignal::Multiple;
                                } else {
                                    signal = SolutionSignal::No;
                                }
                                break;
                            } else {
                                solution = solution / r;
                            }
                        }
                        Operator::Div => {
                            if r.p == 0 {
                                signal = SolutionSignal::No;
                                break;
                            } else {
                                // f / a = s => f = s * a
                                solution = solution * r;
                            }
                        }
                    }
                    root = left_expr;
                }
                _ => panic!("One side must be Rational"),
            }
        }
    }

    return match signal {
        SolutionSignal::No => SolutionSignal::No,
        SolutionSignal::Multiple => SolutionSignal::Multiple,
        SolutionSignal::Undef => SolutionSignal::Unique(solution),
        SolutionSignal::Unique(_) => panic!("Unreachable"),
    };
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let ops = line.split(' ').map(|x| x).collect::<Vec<&str>>();

    match solve(ops) {
        SolutionSignal::No => println!("NONE"),
        SolutionSignal::Multiple => println!("MULTIPLE"),
        SolutionSignal::Unique(rational) => {
            print!("X = ");
            if rational.sign {
                print!("-");
            }
            println!("{}/{}", rational.p, rational.q);
        }
        SolutionSignal::Undef => panic!("Unreachable"),
    }
}
