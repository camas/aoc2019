mod question22 {

    pub fn solve(data: Vec<&str>) -> String {
        // Part 1: Shuffle and get index of 2019
        let i = 2019;
        let m = 10007;
        let (a, b) = calc_fn(&data, m);
        let part1 = (a * i + b) % m;

        // Part 2: Same but bigger deck and repeated a few trillion times
        let i = 2020;
        let m = 119_315_717_514_047;
        let n = 101_741_582_076_661;
        let (a, b) = calc_fn_rev(&data, m);
        let i_part = mod_pow(a, n, m) * i;
        let other_part = mul_mod(
            mod_pow(a, n, m) - 1,
            mod_inv(a as i64 - 1, m as i64) as u64,
            m,
        ) as u128;
        let part2 = (i_part as u128 + other_part * b as u128) % m as u128;
        format!("{} {}", part1, part2)
    }

    fn calc_fn_rev(lines: &[&str], m: u64) -> (u64, u64) {
        // Explanation written down on a piece of paper somewhere
        // Hardest part is doing modular arithmetic with a mix of signed and unsigned integers
        let mut a = 1;
        let mut b = 0;
        for line in lines.iter().rev() {
            if line.starts_with("deal into new stack") {
                a = m - a;
                b = m - b - 1;
            } else if line.starts_with("cut") {
                let j: i64 = line[4..].parse().unwrap();
                let j_mod = if j < 0 {
                    (j + m as i64) as u64
                } else {
                    j as u64
                };
                b = (b + j_mod) % m;
            } else if line.starts_with("deal with increment") {
                let j: u64 = line[20..].parse().unwrap();
                let j_inv = mod_inv(j as i64, m as i64) as u64;
                a = mul_mod(a, j_inv, m);
                b = mul_mod(b, j_inv, m);
            } else {
                unreachable!();
            }
        }
        (a, b)
    }

    fn calc_fn(lines: &[&str], m: u64) -> (u64, u64) {
        // Explanation written down on a piece of paper somewhere
        // Hardest part is doing modular arithmetic with a mix of signed and unsigned integers
        let mut a = 1;
        let mut b = 0;
        for line in lines {
            if line.starts_with("deal into new stack") {
                a = m - a;
                b = m - b - 1;
            } else if line.starts_with("cut") {
                let j: i64 = line[4..].parse().unwrap();
                let j_mod = if j < 0 {
                    (j + m as i64) as u64
                } else {
                    j as u64
                };
                b = (m + b - j_mod) % m;
            } else if line.starts_with("deal with increment") {
                let j: u64 = line[20..].parse().unwrap();
                a = mul_mod(a, j, m);
                b = mul_mod(b, j, m);
            } else {
                unreachable!();
            }
        }
        (a, b)
    }

    // https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
    fn mod_pow(b: u64, e: u64, m: u64) -> u64 {
        let mut base = b as u128;
        let mut exp = e as u128;
        let modulus = m as u128;
        if modulus == 1 {
            return 0;
        }
        let mut result = 1;
        base %= modulus;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            exp >>= 1;
            base = base * base % modulus
        }
        result as u64
    }

    fn mul_mod(mut x: u64, mut y: u64, m: u64) -> u64 {
        let mut d = 0_u64;
        let mp2 = m >> 1;
        x %= m;
        y %= m;

        for _ in 0..64 {
            d = if d > mp2 { (d << 1) - m } else { d << 1 };
            if x & 0x8000_0000_0000_0000_u64 != 0 {
                d += y;
            }
            if d > m {
                d -= m;
            }
            x <<= 1;
        }
        d
    }

    // https://rosettacode.org/wiki/Modular_inverse#Rust
    fn mod_inv(a: i64, module: i64) -> i64 {
        let mut mn = (module, a);
        let mut xy = (0, 1);

        while mn.1 != 0 {
            xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
            mn = (mn.1, mn.0 % mn.1);
        }

        while xy.0 < 0 {
            xy.0 += module;
        }
        xy.0
    }
}
