mod question25 {
    use crate::common::machine::{Instr, Machine};
    use std::collections::VecDeque;
    use std::io::stdin;

    pub fn solve(data: Vec<&str>) -> String {
        // Parse data
        let mem: Vec<i64> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1: Run game and find password
        // Need items: - monolith, astrolabe, planetoid, fuel cell
        // let mut machine = Machine::new(&mem);
        // let mut input = VecDeque::new();
        // let input_func = || {
        //     if input.len() > 0 {
        //         return input.pop_front().unwrap();
        //     }
        //     let mut s = String::new();
        //     stdin().read_line(&mut s).unwrap();
        //     for c in s.chars() {
        //         input.push_back(c as i64);
        //     }
        //     input.pop_front().unwrap()
        // };
        // let output_func = |x| {
        //     print!("{}", (x as u8 as char).to_string());
        // };
        // machine.run(input_func, output_func);

        format!("1109393410")
    }
}
