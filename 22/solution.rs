mod question22 {
    use std::collections::HashMap;

    const DECK_SIZE: usize = 10007;

    pub fn solve(data: Vec<&str>) -> String {
        // Part 1: Do instructions
        // Init deck
        let mut deck: [u32; DECK_SIZE] = [0; DECK_SIZE];
        for (i, item) in deck.iter_mut().enumerate() {
            *item = i as u32;
        }
        // Parse instructions
        for line in data.iter() {
            if line.starts_with("deal into new stack") {
                deck = reverse_deck(&deck);
            } else if line.starts_with("cut") {
                let cut_val = line[4..].parse().unwrap();
                deck = cut_deck(&deck, cut_val);
            } else if line.starts_with("deal with increment") {
                let incr_val = line[20..].parse().unwrap();
                deck = incr_deck(&deck, incr_val);
            } else {
                unreachable!();
            }
        }
        let mut part1 = 0;
        for (i, item) in deck.iter().enumerate() {
            if *item == 2019 {
                part1 = i;
                break;
            }
        }

        format!("{}", part1)
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

    fn reverse_deck(deck: &[u32; DECK_SIZE]) -> [u32; DECK_SIZE] {
        let mut new = [0; DECK_SIZE];
        for (i, x) in deck.iter().rev().enumerate() {
            new[i as usize] = *x;
        }
        new
    }

    fn cut_deck(deck: &[u32; DECK_SIZE], cut_index: i32) -> [u32; DECK_SIZE] {
        let mut new = [0; DECK_SIZE];
        for (i, item) in new.iter_mut().enumerate() {
            let other_index = ((i as i32 + cut_index).rem_euclid(DECK_SIZE as i32)) as usize;
            *item = deck[other_index];
        }
        new
    }

    fn incr_deck(deck: &[u32; DECK_SIZE], incr_size: usize) -> [u32; DECK_SIZE] {
        let mut new = [0; DECK_SIZE];
        for (i, item) in deck.iter().enumerate() {
            let real_index = (i * incr_size) % DECK_SIZE;
            new[real_index] = *item;
        }
        new
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_incr_deck() {
            let mut data = [0; DECK_SIZE];
            for (i, item) in data.iter_mut().enumerate() {
                *item = i as u32;
            }
            let incr = incr_deck(&data, 3);
            assert_eq!(incr[0], 0);
            assert_eq!(incr[3], 1);
            assert_eq!(incr[6], 2);
            assert_eq!(incr[1], 3336);
        }

        #[test]
        fn test_cut_deck() {
            let mut data = [0; DECK_SIZE];
            for (i, item) in data.iter_mut().enumerate() {
                *item = i as u32;
            }
            let cut = cut_deck(&data, 3);
            assert_eq!(cut[0], 3);
            assert_eq!(cut[1], 4);
            assert_eq!(cut[2], 5);
            assert_eq!(cut[3], 6);
            assert_eq!(cut[4], 7);
            assert_eq!(cut[10006], 2);
            assert_eq!(cut[10005], 1);
            assert_eq!(cut[10004], 0);
            assert_eq!(cut[10003], 10006);
        }

        #[test]
        fn test_reverse_deck() {
            let mut data = [0; DECK_SIZE];
            for (i, item) in data.iter_mut().enumerate() {
                *item = i as u32;
            }
            let reversed = reverse_deck(&data);
            assert_eq!(reversed[0], 10006);
            assert_eq!(reversed[1], 10005);
            assert_eq!(reversed[2], 10004);
            assert_eq!(reversed[3], 10003);
            assert_eq!(reversed[10006], 0);
            assert_eq!(reversed[10005], 1);
            assert_eq!(reversed[10004], 2);
            assert_eq!(reversed[10003], 3);
        }
    }
}
