pub mod question5 {
    use crate::common::machine::Machine;

    pub fn solve(data: Vec<&str>) -> String {
        let mem: Vec<_> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1
        let mut machine1 = Machine::new(&mem);
        let input_func = || 1;
        let mut output = Vec::new();
        let output_func = |x| output.push(x);
        machine1.run(input_func, output_func);
        let part1 = output.last().unwrap();

        // Part 2
        let mut machine2 = Machine::new(&mem);
        let mut input_2 = vec![5];
        let input_func_2 = || input_2.pop().unwrap();
        let mut output_2 = Vec::new();
        let output_func_2 = |x| output_2.push(x);
        machine2.run(input_func_2, output_func_2);
        let part2 = output_2.last().unwrap();

        // Return solution
        return format!("{} {}", part1, part2);
    }
}
