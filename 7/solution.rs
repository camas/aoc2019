mod question7 {
    use crate::common::machine::Instr;
    use crate::common::machine::Machine;
    use itertools::Itertools;

    pub fn solve(data: Vec<&str>) -> String {
        // Parse input
        let mem: Vec<i64> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1: Find max thrust
        // Runs an amplifier machine and returns it's first output
        fn run_amp(phase: i64, signal: i64, mem: &[i64]) -> i64 {
            let mut machine = Machine::new(&mem);
            let mut input = vec![signal, phase];
            let mut output = Vec::new();
            machine.run(|| input.pop().unwrap(), |x| output.push(x));
            output[0]
        }
        let mut highest_signal: i64 = 0;
        for phases in [0, 1, 2, 3, 4].iter().permutations(5) {
            let a_res = run_amp(*phases[0], 0, &mem);
            let b_res = run_amp(*phases[1], a_res, &mem);
            let c_res = run_amp(*phases[2], b_res, &mem);
            let d_res = run_amp(*phases[3], c_res, &mem);
            let e_res = run_amp(*phases[4], d_res, &mem);
            if e_res > highest_signal {
                highest_signal = e_res;
            }
        }

        // Part 2: Find max thrust using feedback loops

        // Test all possible phase combinations
        let mut highest_f_signal: i64 = 0;
        for phases in [5, 6, 7, 8, 9].iter().permutations(5) {
            // Create machines A-E
            let mut machines = Vec::new();
            for _ in 0..5 {
                let m = Machine::new(&mem);
                machines.push(m);
            }
            // Loops until E halts
            let mut first_loop = true;
            let mut pipe = vec![0]; // Stores values piped between machines
            loop {
                let mut res = true;
                for i in 0..machines.len() {
                    let m = &mut machines[i];
                    // If first loop, pass phase as first input by adding it to the pipe
                    if first_loop {
                        pipe.push(*phases[i]);
                    }
                    let in_func = || pipe.pop().unwrap();
                    // Create temp output storage
                    let mut outputs_tmp = Vec::new();
                    let out_func = |x| outputs_tmp.push(x);
                    // Run machine until it outputs
                    res = m.run_until(Instr::Output, in_func, out_func);

                    // Add outputs to pipe
                    pipe.append(&mut outputs_tmp);
                }
                first_loop = false;
                if !res {
                    let final_val = pipe.pop().unwrap();
                    if final_val > highest_f_signal {
                        highest_f_signal = final_val;
                    }
                    break;
                }
            }
        }

        // Return solutions
        return format!("{} {}", highest_signal, highest_f_signal);
    }
}
