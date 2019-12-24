mod question24 {
    use std::collections::HashSet;

    pub fn solve(data: Vec<&str>) -> String {
        // Parse data
        let mut initial_state = [[false; 5]; 5];
        for (y, line) in data.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '#' => initial_state[y][i] = true,
                    '.' => initial_state[y][i] = false,
                    _ => unreachable!(),
                };
            }
        }

        // Part 1: Find repeating states and return biodiversity
        let mut seen = HashSet::new();
        let mut cur = initial_state;
        let part1;
        loop {
            let biodiv = calc_biodiv(&cur);
            let is_new = seen.insert(biodiv);
            if !is_new {
                part1 = biodiv;
                break;
            }
            cur = next(&cur);
        }

        format!("{}", part1)
    }

    fn calc_biodiv(state: &[[bool; 5]; 5]) -> u32 {
        let mut total = 0;
        let outer = state.len();
        let inner = state.len();
        let count = outer * inner;
        for i in 0..count {
            if state[i / outer][i % inner] {
                total += 2_u32.pow(i as u32);
            }
        }

        total
    }

    fn next(state: &[[bool; 5]; 5]) -> [[bool; 5]; 5] {
        let mut next_state = [[false; 5]; 5];
        let outer = state.len() as i32;
        let inner = state[0].len() as i32;
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for y in 0..outer {
            for x in 0..inner {
                // Count adjacent
                let mut adjacent = 0;
                for dir in &dirs {
                    let check_x = x + dir.0;
                    let check_y = y + dir.1;
                    // Ignore self
                    if check_x == x && check_y == y {
                        continue;
                    }
                    // Check bounds
                    if check_x < 0 || check_y < 0 || check_x >= inner || check_y >= outer {
                        continue;
                    }
                    if state[check_y as usize][check_x as usize] {
                        adjacent += 1;
                    }
                }
                // Update new state
                let is_bug = state[y as usize][x as usize];
                let new_value;
                if is_bug {
                    if adjacent == 1 {
                        new_value = true;
                    } else {
                        new_value = false;
                    }
                } else if adjacent == 1 || adjacent == 2 {
                    new_value = true;
                } else {
                    new_value = false;
                }
                next_state[y as usize][x as usize] = new_value;
            }
        }

        next_state
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_biodiv() {
            let state = [
                [true, false, false, false, true],
                [true, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
            ];
            assert_eq!(calc_biodiv(&state), 49);

            let state = [
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [true, false, false, false, false],
                [false, true, false, false, false],
            ];
            assert_eq!(calc_biodiv(&state), 2_129_920);
        }
    }
}
