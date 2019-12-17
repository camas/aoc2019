mod question16 {
    use std::char;

    pub fn solve(data: Vec<&str>) -> String {
        // Parse input
        let digits: Vec<_> = data[0].chars().map(|x| x.to_digit(10).unwrap()).collect();

        // Part 1: Apply FFT
        let result = fft(&digits, 100);
        let part1: String = result[0..8]
            .iter()
            .map(|&x| char::from_digit(x, 10).unwrap())
            .collect();

        // Part 2: Apply FFT to the same input * 10000. Get 8 from offset
        let result = optimised_fft(&digits, 100);
        let part2: String = result
            .iter()
            .map(|&x| char::from_digit(x, 10).unwrap())
            .collect();

        format!("{} {}", part1, part2)
    }

    fn optimised_fft(digits: &[u32], phases: usize) -> Vec<u32> {
        // Variable setup
        let digits_len = digits.len() * 10_000;
        let mut temp_offset: usize = 0;
        for (i, val) in digits[0..7].iter().rev().enumerate() {
            temp_offset += 10usize.pow(i as u32) * *val as usize;
        }
        let offset = temp_offset;
        if offset as usize <= digits_len / 2 {
            panic!("Can't optimise when offset too low!");
        }

        // Get used digits past offset
        let mut cur_digits = Vec::with_capacity(digits_len - offset);
        for i in temp_offset..digits_len {
            cur_digits.push(digits[i % digits.len()]);
        }

        // Repeat for given number of phases
        for _ in 0..phases {
            let mut new_digits = Vec::with_capacity(digits_len - offset);
            let mut cur_val = 0;
            for val in cur_digits.iter().rev() {
                cur_val = (cur_val + val) % 10;
                new_digits.push(cur_val);
            }
            new_digits.reverse();
            cur_digits = new_digits;
        }

        cur_digits[0..8].to_vec()
    }

    fn fft(digits: &[u32], phases: usize) -> Vec<u32> {
        let mut curr_digits: Vec<i64> = digits.iter().map(|&x| x as i64).collect();
        let pattern = vec![0, 1, 0, -1];
        // Do n phases
        for _ in 0..phases {
            let mut new_digits: Vec<i64> = Vec::with_capacity(curr_digits.len());
            // Calculate new value for each digit
            for pattern_mult in 1..=curr_digits.len() {
                let mut new_digit = 0;
                for (i, old_digit) in curr_digits.iter().enumerate() {
                    let pattern_i = ((i + 1) / pattern_mult) % pattern.len();
                    new_digit += old_digit * pattern[pattern_i];
                }
                new_digits.push(new_digit.abs() % 10);
            }
            curr_digits = new_digits;
        }

        curr_digits.iter().map(|&x| x as u32).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_known_answers() {
            let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
            assert_eq!(fft(&input, 4), vec![0, 1, 0, 2, 9, 4, 9, 8]);
            let input = vec![
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4,
                5, 5, 9, 5,
            ];
            assert_eq!(fft(&input, 100)[0..8], [2, 4, 1, 7, 6, 1, 7, 6]);
            let input = vec![
                1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4, 1, 8,
                9, 9, 1, 7,
            ];
            assert_eq!(fft(&input, 100)[0..8], [7, 3, 7, 4, 5, 4, 1, 8]);
            let input = vec![
                6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4, 3, 1,
                9, 8, 7, 3,
            ];
            assert_eq!(fft(&input, 100)[0..8], [5, 2, 4, 3, 2, 1, 3, 3]);
            let input = vec![
                0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7,
                4, 6, 6, 4,
            ];
            assert_eq!(optimised_fft(&input, 100), [8, 4, 4, 6, 2, 0, 2, 6]);
            let input = vec![
                0, 2, 9, 3, 5, 1, 0, 9, 6, 9, 9, 9, 4, 0, 8, 0, 7, 4, 0, 7, 5, 8, 5, 4, 4, 7, 0, 3,
                4, 3, 2, 3,
            ];
            assert_eq!(optimised_fft(&input, 100), [7, 8, 7, 2, 5, 2, 7, 0]);
            let input = vec![
                0, 3, 0, 8, 1, 7, 7, 0, 8, 8, 4, 9, 2, 1, 9, 5, 9, 7, 3, 1, 1, 6, 5, 4, 4, 6, 8, 5,
                0, 5, 1, 7,
            ];
            assert_eq!(optimised_fft(&input, 100), [5, 3, 5, 5, 3, 7, 3, 1]);
        }
    }
}
