use std::fmt::{Display, Formatter, Result};
use std::io::stdin;
use std::ops::Add;

const MAX_UUINT_CNT: usize = 128;
#[derive(Clone, Copy, Debug, PartialEq)]
struct UltraUInt {
    data: [u64; MAX_UUINT_CNT],
}

impl UltraUInt {
    const fn zero() -> Self {
        UltraUInt {
            data: [0; MAX_UUINT_CNT],
        }
    }

    const fn from_u64(val: u64) -> Self {
        let mut data = [0; MAX_UUINT_CNT];
        data[0] = val;
        UltraUInt { data }
    }

    fn is_high_masked(&self) -> bool {
        if self.data[MAX_UUINT_CNT - 1] == u64::MAX {
            true
        } else {
            false
        }
    }

    const fn high_masked() -> Self {
        let mut data = [0; MAX_UUINT_CNT];
        data[MAX_UUINT_CNT - 1] = u64::MAX;
        UltraUInt { data }
    }

    fn div_rem_u64(&mut self, divisor: u64) -> u64 {
        let mut rem = 0u128;
        for i in (0..MAX_UUINT_CNT).rev() {
            let current = (self.data[i] as u128) + (rem << 64);
            self.data[i] = (current / divisor as u128) as u64;
            rem = current % divisor as u128;
        }
        rem as u64
    }

    fn is_zero(&self) -> bool {
        self.data.iter().all(|&x| x == 0)
    }
}

impl<'a, 'b> Add<&'b UltraUInt> for &'a UltraUInt {
    type Output = UltraUInt;
    fn add(self, rhs: &'b UltraUInt) -> Self::Output {
        let mut new = [0; MAX_UUINT_CNT];
        let mut carry = 0u128;
        for i in 0..MAX_UUINT_CNT {
            let s = (self.data[i] as u128) + (rhs.data[i] as u128) + carry;
            new[i] = s as u64;
            carry = s >> 64;
        }
        UltraUInt { data: new }
    }
}

impl Display for UltraUInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let mut temp = *self;
        let mut chunks = Vec::new();
        let base = 10_000_000_000_000_000_000u64;

        while !temp.is_zero() {
            chunks.push(temp.div_rem_u64(base));
        }

        if let Some(last) = chunks.pop() {
            write!(f, "{}", last)?;
        }

        for chunk in chunks.into_iter().rev() {
            write!(f, "{:019}", chunk)?;
        }

        Ok(())
    }
}
static mut F: [UltraUInt; 10001] = {
    let mut arr: [UltraUInt; 10001] = [UltraUInt::high_masked(); 10001];
    arr[0] = UltraUInt::from_u64(0);
    arr[1] = UltraUInt::from_u64(1);
    arr
};

fn fib(n: usize) -> UltraUInt {
    unsafe {
        for i in 2..=n {
            F[i] = &F[i - 1] + &F[i - 2];
        }
        F[n]
    }
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let n = line.trim().parse::<usize>().unwrap();

    println!("{}", fib(n));
}
