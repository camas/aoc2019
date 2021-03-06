pub mod question2 {
    use crate::common::machine::Machine;
    pub fn solve(data: Vec<&str>) -> String {
        // Parse data
        let mem: Vec<_> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1
        let mut machine = Machine::new(&mem);
        machine.set_mem(1, 12);
        machine.set_mem(2, 2);
        machine.run(|| 1, |x| println!("{}", x));
        let part1 = machine.get_mem(0);

        // Part 2
        let mut part2 = 0;
        'noun: for noun in 1..100 {
            for verb in 1..100 {
                let mut test_machine = Machine::new(&mem);
                test_machine.set_mem(1, noun);
                test_machine.set_mem(2, verb);
                test_machine.run(|| 1, |_| {});
                if test_machine.get_mem(0) == 19_690_720 {
                    part2 = 100 * noun + verb;
                    break 'noun;
                }
            }
        }

        // Return solution
        format!("{} {}", part1, part2)
    }
}
