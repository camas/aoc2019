mod question20 {
    use std::collections::{HashMap, HashSet, VecDeque};

    pub fn solve(data: Vec<&str>) -> String {
        // Parse data
        let mut portal_locations = HashMap::new();
        let mut map = HashSet::new();
        let max_x = data[0].len() as i32;
        let max_y = data.len() as i32;
        // Read map
        for (y, &line) in data.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    map.insert((x as i32, y as i32));
                }
                if c == ' ' || c == '#' {
                    continue;
                }
                if c >= 'A' && c <= 'Z' {
                    portal_locations.insert((x as i32, y as i32), c);
                }
            }
        }
        // Find portal locations and names
        let mut portal_inter = HashMap::new();
        for ((x, y), c) in &portal_locations {
            let left_pos = (x - 1, *y);
            let right_pos = (x + 1, *y);
            let up_pos = (*x, y - 1);
            let down_pos = (*x, y + 1);
            if portal_locations.contains_key(&left_pos) && map.contains(&right_pos) {
                let first_char = portal_locations[&left_pos];
                let portal_name = format!("{}{}", first_char, c);
                portal_inter.insert(right_pos, portal_name);
            }
            if portal_locations.contains_key(&right_pos) && map.contains(&left_pos) {
                let last_char = portal_locations[&right_pos];
                let portal_name = format!("{}{}", c, last_char);
                portal_inter.insert(left_pos, portal_name);
            }
            if portal_locations.contains_key(&up_pos) && map.contains(&down_pos) {
                let first_char = portal_locations[&up_pos];
                let portal_name = format!("{}{}", first_char, c);
                portal_inter.insert(down_pos, portal_name);
            }
            if portal_locations.contains_key(&down_pos) && map.contains(&up_pos) {
                let last_char = portal_locations[&down_pos];
                let portal_name = format!("{}{}", c, last_char);
                portal_inter.insert(up_pos, portal_name);
            }
        }
        // Match related portals
        let mut start = (0, 0, 0);
        let mut end = (0, 0, 0);
        let mut portals: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        for portal in &portal_inter {
            if portal.1 == "AA" {
                let portal_pos = portal.0;
                start = (portal_pos.0, portal_pos.1, 0);
                continue;
            }
            if portal.1 == "ZZ" {
                let portal_pos = portal.0;
                end = (portal_pos.0, portal_pos.1, 0);
                continue;
            }
            for other in &portal_inter {
                if portal.0 != other.0 && portal.1 == other.1 {
                    portals.insert(*portal.0, *other.0);
                }
            }
        }

        // Part 1: Find shortest route
        let part1 = find_shortest_path(start, end, &map, &portals, false, max_x, max_y);

        // Part 2: Find shortest route with recursive map
        let part2 = find_shortest_path(start, end, &map, &portals, true, max_x, max_y);

        // Return data
        format!("{} {}", part1, part2)
    }

    fn find_shortest_path(
        start: (i32, i32, i32),
        end: (i32, i32, i32),
        map: &HashSet<(i32, i32)>,
        portals: &HashMap<(i32, i32), (i32, i32)>,
        recursive: bool,
        max_x: i32,
        max_y: i32,
    ) -> u32 {
        let mut states = VecDeque::new();
        let first_state = State {
            x: start.0,
            y: start.1,
            z: start.2,
            steps: 0,
        };
        let mut seen = HashMap::new();
        states.push_back(first_state);
        // Search loop
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        loop {
            let state = states.pop_front().unwrap();
            let pos = (state.x, state.y, state.z);
            let simple_pos = (state.x, state.y);
            // Check state valid
            if !map.contains(&simple_pos) {
                continue;
            }
            if seen.contains_key(&pos) {
                continue;
            }
            // Add to seen
            seen.insert(pos, state.steps);
            // Check if reached end
            if pos == end {
                break;
            }
            // Add possible moves to state queue
            let new_steps = state.steps + 1;
            for dir in &directions {
                let new_x = state.x + dir.0;
                let new_y = state.y + dir.1;
                let new_state = State {
                    x: new_x,
                    y: new_y,
                    z: state.z,
                    steps: new_steps,
                };
                states.push_back(new_state);
            }
            if portals.contains_key(&simple_pos) {
                let portal_dest = portals[&simple_pos];
                let new_state = if recursive {
                    let on_edge = state.x == 2
                        || state.y == 2
                        || state.x == max_x - 3
                        || state.y == max_y - 3;
                    if on_edge && state.z == 0 {
                        continue;
                    }
                    let new_z = if on_edge { state.z + 1 } else { state.z - 1 };
                    State {
                        x: portal_dest.0,
                        y: portal_dest.1,
                        z: new_z,
                        steps: new_steps,
                    }
                } else {
                    State {
                        x: portal_dest.0,
                        y: portal_dest.1,
                        z: state.z,
                        steps: new_steps,
                    }
                };
                states.push_back(new_state);
            }
        }
        seen[&end]
    }

    struct State {
        x: i32,
        y: i32,
        z: i32,
        steps: u32,
    }

    mod tests {
        use super::*;
        use crate::common::read_file;

        #[test]
        fn test_known_answers() {
            let input = read_file("20/test3.txt");
            let input_ref = input.iter().map(AsRef::as_ref).collect();
            assert_eq!(solve(input_ref), "77 396");

            let input = read_file("20/test1.txt");
            let input_ref = input.iter().map(AsRef::as_ref).collect();
            assert_eq!(solve(input_ref), "23 26");

            // let input = read_file("20/test2.txt");
            // let input_ref = input.iter().map(AsRef::as_ref).collect();
            // assert_eq!(solve(input_ref), "58 294");
        }
    }
}
