mod question23 {
    use crate::common::machine::Machine;
    use std::collections::{HashMap, HashSet};
    use std::sync::mpsc;
    use std::thread;

    pub fn solve(data: Vec<&str>) -> String {
        // Parse data
        let mem: Vec<_> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1: Run all machines and return first Y value sent to NAT
        // Part 2: Return first value NAT sends to 0 twice
        let (main_tx, main_rx) = mpsc::channel::<(i64, i64, i64)>();
        let mut channels = Vec::new();
        let mut threads = Vec::new();
        for i in 0..50 {
            let (tx, rx) = mpsc::channel::<(i64, i64)>();
            let main_tx_clone = mpsc::Sender::clone(&main_tx);
            let machine = Machine::new(&mem);
            let handle = thread::spawn(move || {
                let mut machine = machine;
                let mut input = vec![i];
                let input_func = || {
                    if input.is_empty() {
                        match rx.try_recv() {
                            Ok((x, y)) => {
                                input.push(y);
                                input.push(x);
                                main_tx_clone.send((-1, i, 0)).unwrap();
                            }
                            Err(_) => {
                                input.push(-1);
                                main_tx_clone.send((-1, i, 1)).unwrap();
                            }
                        };
                    }
                    input.pop().unwrap()
                };
                let mut output = Vec::new();
                let output_func = |x| {
                    output.push(x);
                    if output.len() == 3 {
                        let y = output.pop().unwrap();
                        let x = output.pop().unwrap();
                        let other_i = output.pop().unwrap();
                        main_tx_clone.send((other_i, x, y)).unwrap();
                        main_tx_clone.send((-1, i, 0)).unwrap();
                    }
                };
                machine.run(input_func, output_func);
            });
            channels.push(tx);
            threads.push(handle);
        }
        // Handle messages
        let mut found_part1 = false;
        let mut part1 = -1;
        let mut last_nat = (0, 0);
        let mut idles = [0; 50];
        let mut nat_sent: HashSet<i64> = HashSet::new();
        let mut part2 = -1;
        for (i, x, y) in main_rx {
            // Capture value for part 1
            if i == 255 && !found_part1 {
                part1 = y;
                found_part1 = true;
            }
            // Handle message to NAT
            if i == 255 {
                last_nat = (x, y);
                continue;
            }

            // Handle idle messages
            if i == -1 {
                let new_count = if y == 0 { 0 } else { idles[x as usize] + 1 };
                idles[x as usize] = new_count;
                let mut all_idle = true;
                for value in idles.iter() {
                    if *value < 100 {
                        all_idle = false;
                        break;
                    }
                }
                if all_idle {
                    if !nat_sent.insert(last_nat.1) {
                        part2 = last_nat.1;
                        panic!("TODO: Close threads cleanly");
                        break;
                    }
                    idles = [0; 50];
                    channels[0].send(last_nat).unwrap();
                }
                continue;
            }

            // Send message to channel
            channels[i as usize].send((x, y)).unwrap();
        }

        // Return solutions
        format!("{} {}", part1, part2)
    }
}
