mod question15 {

    use crate::common::machine::{Instr, Machine};
    use std::collections::{HashMap, VecDeque};
    use std::fmt;

    pub fn solve(data: Vec<&str>) -> String {
        // Parse input
        let mem: Vec<_> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1: Find shortest path to oxygen system
        let mut map = RepairMap::new(&mem);
        map.explore();
        let path = map.find_path_to_oxygen();
        let part1 = path.len();

        // Part 2: Calculate time to fill
        let part2 = map.calc_fill_time();

        format!("{} {}", part1, part2)
    }

    struct RepairMap {
        map: HashMap<Point, i64>,
        program: Vec<i64>,
    }

    impl fmt::Debug for RepairMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut output = String::new();
            let min_x = self.map.keys().min_by_key(|a| a.x).unwrap().x;
            let min_y = self.map.keys().min_by_key(|a| a.y).unwrap().y;
            let max_x = self.map.keys().max_by_key(|a| a.x).unwrap().x;
            let max_y = self.map.keys().max_by_key(|a| a.y).unwrap().y;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let point = Point::new(x, y);
                    match self.map.get(&point) {
                        None => output.push('?'),
                        Some(0) => output.push('#'),
                        Some(1) => output.push(' '),
                        Some(2) => output.push('X'),
                        _ => unreachable!(),
                    }
                }
                output.push('\n');
            }
            write!(f, "{}", output)
        }
    }

    impl RepairMap {
        // tile types: 0 - wall, 1 - empty, 2 - oxygen

        fn new(program: &[i64]) -> RepairMap {
            let mut repair = RepairMap {
                map: HashMap::new(),
                program: Vec::from(program),
            };
            repair.map.insert(Point::new(0, 0), 1);
            repair
        }

        fn calc_fill_time(&self) -> u64 {
            // Find unknown point using simple grid search
            // point: distance from start
            let mut seen: HashMap<Point, i64> = HashMap::new();
            let mut queue = VecDeque::new();
            let oxygen_pos = self.map.iter().find(|(_, &v)| v == 2).unwrap().0;
            let mut max_time = 0;
            queue.push_back((*oxygen_pos, 0));
            while !queue.is_empty() {
                let (cur_pos, cur_dist) = queue.pop_front().unwrap();
                if seen.contains_key(&cur_pos) {
                    continue;
                }
                if self.map.contains_key(&cur_pos) {
                    let cur_value = self.map[&cur_pos];
                    if cur_value == 0 {
                        // If wall
                        continue;
                    }
                }

                seen.insert(cur_pos, cur_dist);
                if cur_dist > max_time {
                    max_time = cur_dist;
                }
                let new_dist = cur_dist + 1;
                queue.push_back((Point::new(cur_pos.x + 1, cur_pos.y), new_dist));
                queue.push_back((Point::new(cur_pos.x - 1, cur_pos.y), new_dist));
                queue.push_back((Point::new(cur_pos.x, cur_pos.y + 1), new_dist));
                queue.push_back((Point::new(cur_pos.x, cur_pos.y - 1), new_dist));
            }
            max_time as u64
        }

        fn explore(&mut self) {
            let mut machine = Machine::new(&self.program);
            let mut curr_pos = Point::new(0, 0);

            loop {
                let result = self.find_path_to_unknown(&curr_pos);
                let path;
                match result {
                    Some(x) => path = x,
                    None => break,
                }
                if path.len() > 1 {
                    // Move along known parts of path
                    let result = self.move_path(&path[0..path.len() - 1], &mut machine, &curr_pos);
                    assert_ne!(result, 0); // Check didn't hit wall
                    curr_pos = path[path.len() - 2];
                }
                let test_pos = path[path.len() - 1];
                let result = self.move_step(&mut machine, &curr_pos, &test_pos);
                if result == 1 || result == 2 {
                    curr_pos = test_pos;
                }
            }
        }

        fn find_path_to_oxygen(&mut self) -> Vec<Point> {
            let oxygen_pos = self.map.iter().find(|(_, &v)| v == 2).unwrap().0;
            let start_pos = Point::new(0, 0);
            self.find_shortest_path(&start_pos, &oxygen_pos)
        }

        fn move_step(&mut self, machine: &mut Machine, from: &Point, to: &Point) -> i64 {
            // Find dir to move
            let dir;
            if to.x == from.x + 1 {
                dir = 4;
            } else if to.x == from.x - 1 {
                dir = 3;
            } else if to.y == from.y + 1 {
                dir = 1;
            } else if to.y == from.y - 1 {
                dir = 2;
            } else {
                unreachable!();
            }
            // Pass input to machine
            let input_func = || dir;
            let mut outputs = Vec::new();
            let output_func = |x| outputs.push(x);
            machine.run_until(Instr::Output, input_func, output_func);
            assert_eq!(outputs.len(), 1);
            self.map.insert(*to, outputs[0]);
            // Return machine output
            outputs[0]
        }

        // Returns result of last step
        fn move_path(&mut self, path: &[Point], machine: &mut Machine, robot_pos: &Point) -> i64 {
            let mut cur_pos = robot_pos;
            let mut last_result = -1;
            for next_pos in path {
                last_result = self.move_step(machine, cur_pos, next_pos);
                if last_result == 0 {
                    return 0;
                } // Check move succeeded
                cur_pos = next_pos;
            }
            last_result
        }

        // Finds shortest path treating unknowns as empty
        fn find_shortest_path(&self, from: &Point, to: &Point) -> Vec<Point> {
            // Find unknown point using simple grid search
            // point: distance from start
            let mut seen: HashMap<Point, i64> = HashMap::new();
            let mut queue = VecDeque::new();
            let to_dist;
            queue.push_back((*from, 0));
            loop {
                let (cur_pos, cur_dist) = queue.pop_front().unwrap();
                if &cur_pos == to {
                    to_dist = cur_dist;
                    break;
                }
                if seen.contains_key(&cur_pos) {
                    continue;
                }
                if self.map.contains_key(&cur_pos) {
                    let cur_value = self.map[&cur_pos];
                    if cur_value != 1 {
                        // If not empty tile
                        continue;
                    }
                }

                seen.insert(cur_pos, cur_dist);
                let new_dist = cur_dist + 1;
                queue.push_back((Point::new(cur_pos.x + 1, cur_pos.y), new_dist));
                queue.push_back((Point::new(cur_pos.x - 1, cur_pos.y), new_dist));
                queue.push_back((Point::new(cur_pos.x, cur_pos.y + 1), new_dist));
                queue.push_back((Point::new(cur_pos.x, cur_pos.y - 1), new_dist));
            }

            // Trace back unknown pos to start
            let mut path = Vec::new();
            let mut cur_pos = *to;
            let mut cur_dist = to_dist;
            while &cur_pos != from {
                path.push(cur_pos);
                let to_try = vec![
                    Point::new(cur_pos.x + 1, cur_pos.y),
                    Point::new(cur_pos.x - 1, cur_pos.y),
                    Point::new(cur_pos.x, cur_pos.y + 1),
                    Point::new(cur_pos.x, cur_pos.y - 1),
                ];
                for other in to_try {
                    if !seen.contains_key(&other) {
                        continue;
                    }
                    if seen[&other] == cur_dist - 1 {
                        cur_pos = other;
                        cur_dist -= 1;
                        break;
                    }
                }
            }

            // Return path from start to end
            path.reverse();
            path
        }

        // Finds a path to the closest unknown tile
        fn find_path_to_unknown(&self, robot_pos: &Point) -> Option<Vec<Point>> {
            // Find unknown point using simple grid search
            // point: distance from start
            let mut seen: HashMap<Point, i64> = HashMap::new();
            let mut queue = VecDeque::new();
            let unknown_pos;
            let unknown_dist;
            queue.push_back((*robot_pos, 0));
            loop {
                if queue.is_empty() {
                    return None;
                }
                let (cur_pos, cur_dist) = queue.pop_front().unwrap();
                if seen.contains_key(&cur_pos) {
                    continue;
                }
                if !self.map.contains_key(&cur_pos) {
                    unknown_pos = cur_pos;
                    unknown_dist = cur_dist;
                    break;
                }
                let cur_value = self.map[&cur_pos];
                if cur_value == 0 {
                    // If wall
                    continue;
                }
                seen.insert(cur_pos, cur_dist);
                let new_dist = cur_dist + 1;
                queue.push_back((Point::new(cur_pos.x + 1, cur_pos.y), new_dist));
                queue.push_back((Point::new(cur_pos.x - 1, cur_pos.y), new_dist));
                queue.push_back((Point::new(cur_pos.x, cur_pos.y + 1), new_dist));
                queue.push_back((Point::new(cur_pos.x, cur_pos.y - 1), new_dist));
            }

            // Trace back unknown pos to start
            let mut path = Vec::new();
            let mut cur_pos = unknown_pos;
            let mut cur_dist = unknown_dist;
            while &cur_pos != robot_pos {
                path.push(cur_pos);
                let to_try = vec![
                    Point::new(cur_pos.x + 1, cur_pos.y),
                    Point::new(cur_pos.x - 1, cur_pos.y),
                    Point::new(cur_pos.x, cur_pos.y + 1),
                    Point::new(cur_pos.x, cur_pos.y - 1),
                ];
                for other in to_try {
                    if !seen.contains_key(&other) {
                        continue;
                    }
                    if seen[&other] == cur_dist - 1 {
                        cur_pos = other;
                        cur_dist -= 1;
                        break;
                    }
                }
            }

            // Return path from start to end
            path.reverse();
            Some(path)
        }
    }

    #[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
    struct Point {
        x: i64,
        y: i64,
    }

    impl Point {
        fn new(x: i64, y: i64) -> Point {
            Point { x: x, y: y }
        }
    }
}
