mod question11 {
    use crate::common::machine::Instr;
    use crate::common::machine::Machine;
    use std::collections::HashSet;
    pub fn solve(data: Vec<&str>) -> String {
        use Direction::*;

        // Parse input
        let mem: Vec<_> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1: Run and count painted tiles
        let mut machine = Machine::new(&mem);
        let mut painted_once = HashSet::new();
        let mut white_panels = HashSet::new();
        let mut pos = (0, 0);
        let mut facing = Up;
        loop {
            // Get current color
            let is_white = white_panels.contains(&pos);
            let input = if is_white { 1 } else { 0 };

            // Run until paint output
            let mut output = -1;
            let not_halted = machine.run_until(Instr::Output, || input, |x| output = x);
            if !not_halted {
                break;
            }
            // Update painted tiles
            match output {
                0 => white_panels.remove(&pos),
                1 => white_panels.insert(pos),
                _ => unreachable!(),
            };
            painted_once.insert(pos);
            // Run until turn output
            let mut turn = 0;
            let not_halted = machine.run_until(Instr::Output, || input, |x| turn = x);
            if !not_halted {
                unimplemented!();
            }
            // Move
            match (facing, turn) {
                (Up, 0) => facing = Left,
                (Up, 1) => facing = Right,
                (Left, 0) => facing = Down,
                (Left, 1) => facing = Up,
                (Down, 0) => facing = Right,
                (Down, 1) => facing = Left,
                (Right, 0) => facing = Up,
                (Right, 1) => facing = Down,
                _ => unreachable!(),
            };
            match facing {
                Up => pos = (pos.0, pos.1 + 1),
                Left => pos = (pos.0 - 1, pos.1),
                Down => pos = (pos.0, pos.1 - 1),
                Right => pos = (pos.0 + 1, pos.1),
            }
        }

        // Part 2: Start on a white tile and display output
        // Copied from part 1
        let mut machine = Machine::new(&mem);
        let mut painted_once = HashSet::new();
        let mut white_panels = HashSet::new();
        white_panels.insert((0, 0));
        let mut pos = (0, 0);
        let mut facing = Up;
        loop {
            // Get current color
            let is_white = white_panels.contains(&pos);
            let input = if is_white { 1 } else { 0 };

            // Run until paint output
            let mut output = -1;
            let not_halted = machine.run_until(Instr::Output, || input, |x| output = x);
            if !not_halted {
                break;
            }
            // Update painted tiles
            match output {
                0 => white_panels.remove(&pos),
                1 => white_panels.insert(pos),
                _ => unreachable!(),
            };
            painted_once.insert(pos);
            // Run until turn output
            let mut turn = 0;
            let not_halted = machine.run_until(Instr::Output, || input, |x| turn = x);
            if !not_halted {
                unimplemented!();
            }
            // Move
            match (facing, turn) {
                (Up, 0) => facing = Left,
                (Up, 1) => facing = Right,
                (Left, 0) => facing = Down,
                (Left, 1) => facing = Up,
                (Down, 0) => facing = Right,
                (Down, 1) => facing = Left,
                (Right, 0) => facing = Up,
                (Right, 1) => facing = Down,
                _ => unreachable!(),
            };
            match facing {
                Up => pos = (pos.0, pos.1 + 1),
                Left => pos = (pos.0 - 1, pos.1),
                Down => pos = (pos.0, pos.1 - 1),
                Right => pos = (pos.0 + 1, pos.1),
            }
        }
        // Find min/max positions
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        for (x, y) in &white_panels {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        let capacity = (max_x - min_x).abs() * (max_y - min_y + 1).abs();
        let mut part2 = String::with_capacity(capacity as usize);
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let pos = (x, y);
                let is_white = white_panels.contains(&pos);
                if is_white {
                    part2.push('█');
                } else {
                    part2.push(' ');
                }
            }
            part2.push('\n');
        }

        // Return solutions
        format!("{}\n{}", painted_once.len(), part2)
    }

    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
}
