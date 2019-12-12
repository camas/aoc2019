mod question10 {
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;
    pub fn solve(data: Vec<&str>) -> String {
        // Parse data
        let mut asteroids = vec![];
        for (y, line) in data.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.push((x as i32, y as i32));
                }
            }
        }

        // Part 1: Find the position that can see the most asteroids
        let mut highest_count = 0;
        let mut station_position = (-1, -1);
        for asteroid in &asteroids {
            let mut dirs = HashSet::new();
            for other in &asteroids {
                if asteroid == other {
                    continue;
                }
                let diff = (other.0 - asteroid.0, other.1 - asteroid.1);
                let divisor = gcd(diff.0, diff.1);
                let dir = (diff.0 / divisor, diff.1 / divisor);
                dirs.insert(dir);
            }
            if dirs.len() > highest_count {
                highest_count = dirs.len();
                station_position = *asteroid;
            }
        }

        // Part 2: What are the coords of the 200th destroyed asteroid
        // Return NA if < 201 asteroids, including station
        if asteroids.len() < 201 {
            return format!("{} NA", highest_count);
        }
        // Convert to direction, multiplier
        let mut by_direction = HashMap::new();
        for other in &asteroids {
            if other == &station_position {
                continue;
            }
            let diff = (other.0 - station_position.0, other.1 - station_position.1);
            let divisor = gcd(diff.0, diff.1);
            let dir = (diff.0 / divisor, diff.1 / divisor);
            let list = by_direction.entry(dir).or_insert_with(|| vec![]);
            list.push(divisor);
        }
        // Sort by angle and dist
        for dist_list in by_direction.values_mut() {
            dist_list.sort();
        }
        let mut sorted = Vec::from_iter(&by_direction);
        sorted.sort_by(|(a, _), (b, _)| {
            let angle_a = (a.0 as f64).atan2(a.1 as f64);
            let angle_b = (b.0 as f64).atan2(b.1 as f64);
            angle_b.partial_cmp(&angle_a).unwrap()
        });
        // Find 200th
        let mut cur_dist = 0;
        // Find starting index by assuming there is an asteroid directly above
        let start_index = sorted.iter().position(|(x, _)| **x == (0, -1)).unwrap();
        let mut cur_index = start_index;
        let mut cur_count = 0;
        let part2;
        loop {
            if cur_index == start_index {
                cur_dist += 1;
            }

            let cur_list = sorted[cur_index].1;
            if cur_list.len() >= cur_dist {
                cur_count += 1;
                if cur_count == 200 {
                    // Found. Calc answer
                    let rel_dist = cur_list[cur_dist - 1];
                    let dir = sorted[cur_index].0;
                    let rel_pos = (dir.0 * rel_dist, dir.1 * rel_dist);
                    let x_part = (station_position.0 + rel_pos.0) * 100;
                    let y_part = station_position.1 + rel_pos.1;
                    part2 = x_part + y_part;
                    break;
                }
            }

            cur_index = (cur_index + 1) % sorted.len();
        }

        // Return solutions
        format!("{} {:?}", highest_count, part2)
    }

    // https://rosettacode.org/wiki/Greatest_common_divisor#Rust
    fn gcd(mut m: i32, mut n: i32) -> i32 {
        while m != 0 {
            let old_m = m;
            m = n % m;
            n = old_m;
        }
        n.abs()
    }

    #[cfg(test)]
    mod tests {
        use super::{gcd, solve};
        #[test]
        fn test_known_answers() {
            let input = vec![".#..#", ".....", "#####", "....#", "...##"];
            assert_eq!(solve(input), "8 NA");
            let input = vec![
                "......#.#.",
                "#..#.#....",
                "..#######.",
                ".#.#.###..",
                ".#..#.....",
                "..#....#.#",
                "#..#....#.",
                ".##.#..###",
                "##...#..#.",
                ".#....####",
            ];
            assert_eq!(solve(input), "33 NA");
            let input = vec![
                "#.#...#.#.",
                ".###....#.",
                ".#....#...",
                "##.#.#.#.#",
                "....#.#.#.",
                ".##..###.#",
                "..#...##..",
                "..##....##",
                "......#...",
                ".####.###.",
            ];
            assert_eq!(solve(input), "35 NA");
            let input = vec![
                ".#..#..###",
                "####.###.#",
                "....###.#.",
                "..###.##.#",
                "##.##.#.#.",
                "....###..#",
                "..#.#..#.#",
                "#..#.#.###",
                ".##...##.#",
                ".....#.#..",
            ];
            assert_eq!(solve(input), "41 NA");
            let input = vec![
                ".#..##.###...#######",
                "##.############..##.",
                ".#.######.########.#",
                ".###.#######.####.#.",
                "#####.##.#.##.###.##",
                "..#####..#.#########",
                "####################",
                "#.####....###.#.#.##",
                "##.#################",
                "#####.##.###..####..",
                "..######..##.#######",
                "####.##.####...##..#",
                ".#####..#.######.###",
                "##...#.##########...",
                "#.##########.#######",
                ".####.#.###.###.#.##",
                "....##.##.###..#####",
                ".#.#.###########.###",
                "#.#.#.#####.####.###",
                "###.##.####.##.#..##",
            ];
            assert_eq!(solve(input), "210 802");
        }

        #[test]
        fn test_gcd() {
            assert_eq!(gcd(10, 15), 5);
            assert_eq!(gcd(-10, 15), 5);
            assert_eq!(gcd(1, 1_000_000), 1);
            assert_eq!(gcd(18, 81), 9);
            assert_eq!(gcd(0, 10), 10);
            assert_eq!(gcd(10, 0), 10);
        }
    }
}
