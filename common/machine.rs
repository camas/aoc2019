use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;

pub struct Machine {
    memory: Vec<i64>,
    ip: usize, // Instruction pointer
}

impl Machine {
    pub fn new(mem: &[i64]) -> Machine {
        return Machine {
            memory: mem.to_vec().clone(),
            ip: 0,
        };
    }

    pub fn run(&mut self, input: impl FnMut() -> i64, output: impl FnMut(i64)) {
        self.run_until(Instr::Halt, input, output);
    }

    // Run the machine until after given instruction
    // Returns true if return after given instruction, false if halt instruction
    pub fn run_until(
        &mut self,
        halt_instr: Instr,
        mut input: impl FnMut() -> i64,
        mut output: impl FnMut(i64),
    ) -> bool {
        use Instr::*;

        // Create instruction sizes map
        let mut instr_sizes = HashMap::new();
        instr_sizes.insert(Halt, 0);
        instr_sizes.insert(Add, 3);
        instr_sizes.insert(Mult, 3);
        instr_sizes.insert(Input, 1);
        instr_sizes.insert(Output, 1);
        instr_sizes.insert(JumpTrue, 2);
        instr_sizes.insert(JumpFalse, 2);
        instr_sizes.insert(LessThan, 3);
        instr_sizes.insert(Equals, 3);

        // Main loop
        let mut last_instr: Option<Instr> = None;
        loop {
            // Check if halt instr
            match last_instr {
                Some(x) => {
                    if x == halt_instr {
                        return true;
                    }
                }
                _ => (),
            }
            // Parse opcode
            let raw_opcode = self.memory[self.ip];
            let digits = get_digits(raw_opcode);
            let opcode_i = if digits.len() == 1 {
                *digits.last().unwrap()
            } else {
                digits[digits.len() - 2] * 10 + digits.last().unwrap()
            };
            let instr: Instr = FromPrimitive::from_i64(opcode_i).unwrap();
            let instr_size = instr_sizes[&instr];
            let values: Vec<i64> = self.memory[self.ip + 1..self.ip + 1 + instr_size]
                .iter()
                .cloned()
                .collect();
            // Extract modes
            let mut modes = vec![0; instr_size];
            let diff = modes.len() + 2 - digits.len();
            if digits.len() > 2 {
                for i in 0..(digits.len() - 2) {
                    modes[diff + i] = digits[i];
                }
            }
            modes.reverse();
            // Memory access helpers
            macro_rules! read_mem {
                ($index:expr) => {{
                    let mode = modes[$index];
                    let val = values[$index];
                    match mode {
                        0 => self.memory[val as usize],
                        1 => val,
                        _ => panic!("Unknown mode"),
                    }
                }};
            }
            macro_rules! write_mem {
                ($index:expr, $set_value:expr) => {{
                    let mode = modes[$index];
                    let val: i64 = values[$index];
                    match mode {
                        0 => self.memory[val as usize] = $set_value,
                        1 => panic!("Can't write in immediate mode"),
                        _ => panic!("Unknown mode"),
                    };
                }};
            }

            // Update last_instr. Not used below
            last_instr = Some(instr);

            // Run instruction
            match instr {
                Halt => return false,
                Add => write_mem!(2, read_mem!(0) + read_mem!(1)),
                Mult => write_mem!(2, read_mem!(0) * read_mem!(1)),
                Input => write_mem!(0, input()),
                Output => output(read_mem!(0)),
                JumpTrue => {
                    if read_mem!(0) != 0 {
                        self.ip = read_mem!(1) as usize;
                        continue;
                    }
                }
                JumpFalse => {
                    if read_mem!(0) == 0 {
                        self.ip = read_mem!(1) as usize;
                        continue;
                    }
                }
                LessThan => {
                    if read_mem!(0) < read_mem!(1) {
                        write_mem!(2, 1);
                    } else {
                        write_mem!(2, 0);
                    }
                }
                Equals => {
                    if read_mem!(0) == read_mem!(1) {
                        write_mem!(2, 1);
                    } else {
                        write_mem!(2, 0);
                    }
                } //_ => panic!("Instruction {:?} not implemented", instr),
            }

            self.ip += instr_size + 1;
        }
    }

    pub fn set_mem(&mut self, index: usize, value: i64) {
        self.memory[index] = value;
    }

    pub fn get_mem(&mut self, index: usize) -> i64 {
        return self.memory[index];
    }
}

#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Instr {
    Halt = 99,
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpTrue = 5,
    JumpFalse = 6,
    LessThan = 7,
    Equals = 8,
}

// https://stackoverflow.com/a/41536521
fn get_digits(n: i64) -> Vec<i64> {
    fn x_inner(n: i64, xs: &mut Vec<i64>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    return xs;
}
