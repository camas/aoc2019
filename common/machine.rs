use crate::common::get_digits;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::ops::Range;
use Instr::*;

const MEM_CHUNK_SIZE: usize = 1000;

pub struct Machine {
    memory: Memory,
    ip: usize, // Instruction pointer
    rel_base: usize,
}

impl Machine {
    pub fn new(mem: &[i64]) -> Machine {
        Machine {
            memory: Memory::from(&mem),
            ip: 0,
            rel_base: 0,
        }
    }

    pub fn run(&mut self, input: impl FnMut() -> i64, output: impl FnMut(i64)) {
        self.run_until(Instr::Halt, input, output);
    }

    // Run the machine until after given instruction
    // Returns true if return after given instruction, false if halt instruction
    #[allow(clippy::cognitive_complexity)]
    pub fn run_until(
        &mut self,
        halt_instr: Instr,
        mut input: impl FnMut() -> i64,
        mut output: impl FnMut(i64),
    ) -> bool {
        // Main loop
        let mut last_instr: Option<Instr> = None;
        loop {
            // Check if halt instr
            if let Some(x) = last_instr {
                if x == halt_instr {
                    return true;
                }
            }
            // Parse opcode
            let raw_opcode = self.get_mem(self.ip);
            let digits = get_digits(raw_opcode);
            let opcode_i = if digits.len() == 1 {
                *digits.last().unwrap()
            } else {
                digits[digits.len() - 2] * 10 + digits.last().unwrap()
            };
            let instr: Instr = FromPrimitive::from_i64(opcode_i).unwrap();
            let instr_size = Machine::get_instruction_size(instr);
            let values: Vec<i64> = self
                .get_mem_range(self.ip + 1..self.ip + 1 + instr_size)
                .to_vec();
            // Extract modes
            let mut modes = vec![0; instr_size];
            if digits.len() > 2 {
                let diff = modes.len() + 2 - digits.len();
                modes[diff..].clone_from_slice(&digits[..digits.len() - 2]);
                modes.reverse();
            }
            // Memory access helpers
            // Modes:
            // 0 - position mode - val points to a position in memory
            // 1 - immediate mode - val is the value
            // 2 - relative mode - val + rel_base points to a position in memory
            macro_rules! read_mem {
                ($index:expr) => {{
                    let mode = modes[$index];
                    let val = values[$index];
                    match mode {
                        0 => self.memory.get(val as usize),
                        1 => val,
                        2 => self.memory.get((self.rel_base as i64 + val) as usize),
                        _ => panic!("Unknown mode"),
                    }
                }};
            }
            macro_rules! write_mem {
                ($index:expr, $set_value:expr) => {{
                    let mode = modes[$index];
                    let val: i64 = values[$index];
                    match mode {
                        0 => self.memory.set(val as usize, $set_value),
                        1 => panic!("Can't write in immediate mode"),
                        2 => self
                            .memory
                            .set((self.rel_base as i64 + val) as usize, $set_value),
                        _ => panic!("Unknown mode"),
                    };
                }};
            }

            // Update last_instr. Not used below
            last_instr = Some(instr);

            // Run instruction
            match instr {
                Halt => return false,
                Add => {
                    let added = read_mem!(0) + read_mem!(1);
                    write_mem!(2, added);
                }
                Mult => {
                    let multiplied = read_mem!(0) * read_mem!(1);
                    write_mem!(2, multiplied);
                }
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
                }
                // rel_base converted to i64 and back so that negative adjustments can be used
                RelBase => self.rel_base = (self.rel_base as i64 + read_mem!(0)) as usize,
                //_ => panic!("Instruction {:?} not implemented", instr),
            }

            self.ip += instr_size + 1;
        }
    }

    fn get_instruction_size(instr: Instr) -> usize {
        match instr {
            Halt => 0,
            Add => 3,
            Mult => 3,
            Input => 1,
            Output => 1,
            JumpTrue => 2,
            JumpFalse => 2,
            LessThan => 3,
            Equals => 3,
            RelBase => 1,
        }
    }

    pub fn set_mem(&mut self, index: usize, value: i64) {
        self.memory.set(index, value);
    }

    pub fn get_mem(&mut self, index: usize) -> i64 {
        self.memory.get(index)
    }

    pub fn get_mem_range(&mut self, range: Range<usize>) -> Vec<i64> {
        // Helper function. Not optimised
        let mut out = Vec::new();
        for i in range {
            out.push(self.get_mem(i));
        }
        out
    }
}

pub struct Memory {
    data: HashMap<usize, [i64; MEM_CHUNK_SIZE]>,
}

impl Memory {
    pub fn from(mem: &[i64]) -> Memory {
        let mut m: HashMap<usize, [i64; MEM_CHUNK_SIZE]> = HashMap::new();
        for (i, chunk) in mem.to_vec().chunks(MEM_CHUNK_SIZE).enumerate() {
            let mut d = [0; MEM_CHUNK_SIZE];
            for (i, c) in chunk.iter().enumerate() {
                d[i] = *c;
            }
            m.insert(i, d);
        }
        Memory { data: m }
    }

    pub fn get(&mut self, index: usize) -> i64 {
        let chunk_index = index / MEM_CHUNK_SIZE;
        let chunk_offset = index % MEM_CHUNK_SIZE;
        self.data.entry(chunk_index).or_insert([0; MEM_CHUNK_SIZE]);
        self.data[&chunk_index][chunk_offset]
    }

    pub fn set(&mut self, index: usize, value: i64) {
        let chunk_index = index / MEM_CHUNK_SIZE;
        let chunk_offset = index % MEM_CHUNK_SIZE;
        self.data.entry(chunk_index).or_insert([0; MEM_CHUNK_SIZE]);
        self.data.get_mut(&chunk_index).unwrap()[chunk_offset] = value;
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
    RelBase = 9,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rel_base() {
        let mem = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut m = Machine::new(&mem);
        let mut output = Vec::new();
        m.run(|| unreachable!(), |x| output.push(x));

        assert_eq!(mem, output);
    }

    #[test]
    fn test_large_number() {
        let mem = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let mut m = Machine::new(&mem);
        let mut output = Vec::new();
        m.run(|| unreachable!(), |x| output.push(x));

        assert!(
            output[0] == 1_219_070_632_396_864,
            "Result was {}",
            output[0]
        );
    }

    #[test]
    fn test_large_memory() {
        let mem = vec![104, 1_125_899_906_842_624, 99];
        let mut m = Machine::new(&mem);
        let mut output = Vec::new();
        m.run(|| unreachable!(), |x| output.push(x));

        assert!(
            output[0] == 1_125_899_906_842_624,
            "Result was {}",
            output[0]
        );
    }
}
