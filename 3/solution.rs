pub mod question3 {
    use std::collections::HashMap;

    pub fn solve(data: Vec<&str>) -> String {
        let path1 = travel(&data[0]);
        let path2 = travel(&data[1]);

        let mut closest_man = 9000000000;
        let mut closest_walk = 9000000000;
        for (key, walk1) in &path1 {
            if !path2.contains_key(key) {
                continue;
            }
            let walk2 = path2[key];
            let manhat_dist = key.0.abs() + key.1.abs();
            let walk_dist = walk1 + walk2;
            if manhat_dist < closest_man {
                closest_man = manhat_dist;
            }
            if walk_dist < closest_walk {
                closest_walk = walk_dist;
            }
        }

        return format!("{} {}", closest_man, closest_walk);
    }

    fn travel(input: &str) -> HashMap<(i64, i64), i64> {
        let instructions: Vec<_> = input.split(',').collect();
        let mut output: HashMap<_, _> = HashMap::new();

        let mut curx: i64 = 0;
        let mut cury: i64 = 0;
        let mut walked: i64 = 1;
        for instr in &instructions {
            let dist: i64 = instr[1..].parse().unwrap();
            match instr.as_bytes()[0] {
                b'U' => {
                    for i in (cury + 1)..(cury + dist + 1) {
                        output.insert((curx, i), walked);
                        walked += 1;
                    }
                    cury += dist;
                }
                b'D' => {
                    for i in ((cury - dist - 1)..(cury - 1)).rev() {
                        output.insert((curx, i), walked);
                        walked += 1;
                    }
                    cury -= dist;
                }
                b'R' => {
                    for i in (curx + 1)..(curx + dist + 1) {
                        output.insert((i, cury), walked);
                        walked += 1;
                    }
                    curx += dist;
                }
                b'L' => {
                    for i in ((curx - dist - 1)..(curx - 1)).rev() {
                        output.insert((i, cury), walked);
                        walked += 1;
                    }
                    curx -= dist;
                }
                _ => unreachable!(),
            }
        }

        return output;
    }

    #[cfg(test)]
    mod tests {
        use super::solve;

        #[test]
        fn test_known_answers() {
            let answer = solve(vec!["R8,U5,L5,D3", "U7,R6,D4,L4"]);
            assert_eq!(answer, "6 30");
            let answer = solve(vec![
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
            ]);
            assert_eq!(answer, "159 610");
            let answer = solve(vec![
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            ]);
            assert_eq!(answer, "135 410");
        }
    }
}
