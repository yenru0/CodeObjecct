const PKR_RANKS: [u64; 12] = [
    6_612_900, // TOP
    9_730_740, // ONE_PAIR
    2_532_816, // TWO_PAIR
    0_732_160, // TRIPLE
    0_282_060, // ST
    0_039_780, // BST
    0_039_780, // MT
    0_205_976, // F
    0_165_984, // FH
    0_014_664, // FC
    0_001_472, // SF
    0_000_188, // RSF
];

const PKR_TOT: u64 = {
    let mut s = 0;
    let mut i = 0;
    while i < PKR_RANKS.len() {
        s += PKR_RANKS[i];
        i += 1;
    }
    s
};

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    for &cnt in PKR_RANKS.iter() {
        let g = gcd(PKR_TOT, cnt);
        println!("{}/{}", cnt / g, PKR_TOT / g);
    }
}
