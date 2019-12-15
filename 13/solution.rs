mod question13 {
    use crate::common::machine::{Instr, Machine};
    use std::collections::{HashMap, VecDeque};
    use std::fmt;

    struct PinballMachine {
        machine: Machine,
        tiles: HashMap<(i64, i64), i64>,
        score: i64,
    }

    impl fmt::Debug for PinballMachine {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.tiles.is_empty() {
                return write!(f, "Pinball:    Score: 0");
            }
            let min_x = self.tiles.keys().min_by_key(|a| a.0).unwrap().0;
            let min_y = self.tiles.keys().min_by_key(|a| a.1).unwrap().1;
            let max_x = self.tiles.keys().max_by_key(|a| a.0).unwrap().0;
            let max_y = self.tiles.keys().max_by_key(|a| a.1).unwrap().1;

            let capacity = (max_x - min_x).abs() * (max_y - min_y + 1).abs();
            let mut output = String::with_capacity(capacity as usize);

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    match self.tiles.get(&(x, y)) {
                        None => output.push(' '),
                        Some(0) => output.push(' '),
                        Some(1) => output.push('█'),
                        Some(2) => output.push('▒'),
                        Some(3) => output.push('▔'),
                        Some(4) => output.push('o'),
                        _ => unreachable!(),
                    }
                }
                output.push('\n');
            }

            write!(f, "Pinball:    Score: {}\n{}", self.score, output)
        }
    }

    impl PinballMachine {
        fn new(mem: &[i64]) -> PinballMachine {
            PinballMachine {
                machine: Machine::new(&mem),
                tiles: HashMap::new(),
                score: 0,
            }
        }

        fn run(&mut self) {
            let input = || 0;
            let mut outputs = VecDeque::new();
            let output = |x| {
                outputs.push_back(x);
            };
            self.machine.run(input, output);
            while outputs.len() >= 3 {
                self.tiles.insert(
                    (outputs.pop_front().unwrap(), outputs.pop_front().unwrap()),
                    outputs.pop_front().unwrap(),
                );
            }
        }

        fn insert_coins(&mut self) {
            self.machine.set_mem(0, 2);
        }

        fn win(&mut self) {
            let mut current_dir = 0;
            let mut output = VecDeque::new();
            let mut last_paddle_x = 0;
            loop {
                //println!("{:?}", self);
                let input_func = || current_dir;
                let output_func = |x| output.push_back(x);
                let ran_to_instruction =
                    self.machine
                        .run_until(Instr::Output, input_func, output_func);
                if !ran_to_instruction {
                    break;
                }

                if output.len() < 3 {
                    continue;
                }

                let x = output.pop_front().unwrap();
                let y = output.pop_front().unwrap();
                let value = output.pop_front().unwrap();
                if x == -1 && y == 0 {
                    self.score = value;
                    continue;
                }
                self.tiles.insert((x, y), value);

                if value == 4 {
                    // Ball
                    if x < last_paddle_x {
                        current_dir = -1;
                    } else if x > last_paddle_x {
                        current_dir = 1;
                    } else {
                        current_dir = 0;
                    }
                } else if value == 3 {
                    last_paddle_x = x;
                }
            }
        }
    }

    pub fn solve(data: Vec<&str>) -> String {
        // Parse input
        let mem: Vec<_> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1: Run game and count block tiles
        let mut pinball = PinballMachine::new(&mem);
        pinball.run();
        let part1 = pinball.tiles.iter().filter(|(_, &v)| v == 2).count();

        // Part 2: Run and win game
        let mut pinball = PinballMachine::new(&mem);
        pinball.insert_coins();
        pinball.win();
        let part2 = pinball.score;

        // Return solutions
        format!("{} {}", part1, part2)
    }
}
