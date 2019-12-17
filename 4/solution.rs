mod question4 {
    use crate::common::get_digits;

    pub fn solve(data: Vec<&str>) -> String {
        // Parse input
        let range_ints: Vec<_> = data[0].split('-').map(|x| x.parse().unwrap()).collect();
        let min = range_ints[0];
        let max = range_ints[1];

        // Part 1: Count number matching initial conditions
        let mut part1_count = 0;
        let mut part2_count = 0;
        for num in min..max {
            // Get digits
            let digits = get_digits(num);
            // Check conditions
            let mut two_same = false;
            let mut only_two_same = false;
            let mut increasing = true;
            let mut cur_same = 0;
            for (d1, d2) in digits[0..digits.len() - 1].iter().zip(digits[1..].iter()) {
                if d2 < d1 {
                    increasing = false;
                    break;
                }
                if d1 == d2 {
                    two_same = true;
                    if cur_same == 0 {
                        cur_same = 2;
                    } else {
                        cur_same += 1;
                    }
                } else {
                    if cur_same == 2 {
                        only_two_same = true;
                    }
                    cur_same = 0;
                }
            }
            if cur_same == 2 {
                only_two_same = true;
            }

            if two_same && increasing {
                part1_count += 1;
                if only_two_same {
                    part2_count += 1;
                }
            }
        }

        // Return solutions
        return format!("{} {}", part1_count, part2_count);
    }
}
