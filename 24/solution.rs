mod question24 {
    use std::collections::{HashMap, HashSet};

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

        // Part 2: Count bugs after 200 recursive steps
        let part2 = part2(&initial_state);

        format!("{} {}", part1, part2)
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

    fn part2(initial_state: &[[bool; 5]; 5]) -> u32 {
        // Setup
        // Higher index states contain lower index states
        let outer = initial_state.len() as i32;
        let inner = initial_state[0].len() as i32;
        let mut state_map: HashMap<i32, [[bool; 5]; 5]> = HashMap::new();
        state_map.insert(0, *initial_state);
        let mut min_index = 0;
        let mut max_index = 0;
        let mut adj_dict = HashMap::new();
        adj_dict.insert(1, vec![(2, 0), (6, 0), (8, 1), (12, 1)]);
        adj_dict.insert(2, vec![(1, 0), (3, 0), (7, 0), (8, 1)]);
        adj_dict.insert(3, vec![(2, 0), (4, 0), (8, 0), (8, 1)]);
        adj_dict.insert(4, vec![(3, 0), (5, 0), (9, 0), (8, 1)]);
        adj_dict.insert(5, vec![(4, 0), (10, 0), (8, 1), (14, 1)]);
        adj_dict.insert(6, vec![(1, 0), (7, 0), (11, 0), (12, 1)]);
        adj_dict.insert(7, vec![(2, 0), (6, 0), (8, 0), (12, 0)]);
        adj_dict.insert(
            8,
            vec![
                (3, 0),
                (7, 0),
                (9, 0),
                (1, -1),
                (2, -1),
                (3, -1),
                (4, -1),
                (5, -1),
            ],
        );
        adj_dict.insert(9, vec![(4, 0), (8, 0), (10, 0), (14, 0)]);
        adj_dict.insert(10, vec![(5, 0), (9, 0), (15, 0), (14, 1)]);
        adj_dict.insert(11, vec![(6, 0), (12, 0), (16, 0), (12, 1)]);
        adj_dict.insert(
            12,
            vec![
                (7, 0),
                (11, 0),
                (17, 0),
                (1, -1),
                (6, -1),
                (11, -1),
                (16, -1),
                (21, -1),
            ],
        );
        adj_dict.insert(
            14,
            vec![
                (9, 0),
                (15, 0),
                (19, 0),
                (5, -1),
                (10, -1),
                (15, -1),
                (20, -1),
                (25, -1),
            ],
        );
        adj_dict.insert(15, vec![(10, 0), (14, 0), (20, 0), (14, 1)]);
        adj_dict.insert(16, vec![(11, 0), (17, 0), (21, 0), (12, 1)]);
        adj_dict.insert(17, vec![(12, 0), (16, 0), (18, 0), (22, 0)]);
        adj_dict.insert(
            18,
            vec![
                (17, 0),
                (19, 0),
                (23, 0),
                (21, -1),
                (22, -1),
                (23, -1),
                (24, -1),
                (25, -1),
            ],
        );
        adj_dict.insert(19, vec![(14, 0), (18, 0), (20, 0), (24, 0)]);
        adj_dict.insert(20, vec![(15, 0), (19, 0), (25, 0), (14, 1)]);
        adj_dict.insert(21, vec![(16, 0), (22, 0), (12, 1), (18, 1)]);
        adj_dict.insert(22, vec![(17, 0), (21, 0), (23, 0), (18, 1)]);
        adj_dict.insert(23, vec![(18, 0), (22, 0), (24, 0), (18, 1)]);
        adj_dict.insert(24, vec![(23, 0), (19, 0), (25, 0), (18, 1)]);
        adj_dict.insert(25, vec![(20, 0), (24, 0), (14, 1), (18, 1)]);

        // Repeat for 200 steps
        for _ in 0..200 {
            // Add new blank states to either end
            state_map.insert(min_index - 1, [[false; 5]; 5]);
            state_map.insert(min_index - 2, [[false; 5]; 5]);
            state_map.insert(max_index + 1, [[false; 5]; 5]);
            state_map.insert(max_index + 2, [[false; 5]; 5]);

            let mut new_map = HashMap::new();
            for i in min_index - 1..=max_index + 1 {
                // Calculate new state for each index
                let mut new_state = [[false; 5]; 5];
                for j in 1..=25 {
                    if j == 13 {
                        continue;
                    }
                    let x = (j - 1) % inner;
                    let y = (j - 1) / outer;
                    // Count adjacent
                    let mut adjacent = 0;
                    for (other_j, rel_index) in &adj_dict[&j] {
                        let other_x = (other_j - 1) % inner;
                        let other_y = (other_j - 1) / outer;
                        let other_i = i + rel_index;
                        let is_bug = state_map[&other_i][other_y as usize][other_x as usize];
                        if is_bug {
                            adjacent += 1;
                        }
                    }
                    // Store new value
                    let is_bug = state_map[&i][y as usize][x as usize];
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
                    new_state[y as usize][x as usize] = new_value;
                }
                new_map.insert(i, new_state);
            }
            state_map = new_map;
            min_index -= 1;
            max_index += 1;
        }

        // Return bug count
        let mut total = 0;
        for state in state_map.values() {
            total += count(&state);
        }
        total
    }

    fn count(state: &[[bool; 5]; 5]) -> u32 {
        let outer = state.len();
        let inner = state[0].len();
        let mut total = 0;
        for line in state {
            for &is_bug in line {
                if is_bug {
                    total += 1;
                }
            }
        }
        total
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

        #[test]
        fn test_count() {
            let state = [
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [true, false, false, false, false],
                [false, true, false, false, false],
            ];
            assert_eq!(count(&state), 2);
        }
    }
}
